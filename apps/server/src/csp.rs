//! The `Content-Security-Policy` this server attaches to the documents it serves.
//!
//! The policy is not written out here and it is not written out in configuration. It is derived
//! from the shell the server is about to serve: [`csp_shell`] reads `index.html` at startup and
//! computes a `'sha256-…'` for every inline `<script>` in it, the way the HTML parser computes
//! one. That is the whole point of deriving it — trunk regenerates the inline WASM bootstrap on
//! every frontend build, and a hand-maintained hash would go stale silently, with a blank page
//! and a console message as the only evidence.
//!
//! What configuration decides is only the part that differs between deployments: the Cloudflare
//! concessions in `[server.csp.cloudflare]`. See [`mp_stats_config::CloudflareConfig`].

use anyhow::{Context, Result};
use axum::Router;
use axum::extract::{Request, State};
use axum::http::HeaderValue;
use axum::http::header::{CACHE_CONTROL, CONTENT_SECURITY_POLICY, CONTENT_TYPE};
use axum::middleware::{Next, from_fn_with_state};
use axum::response::Response;
use csp_shell::presets::cloudflare;
use csp_shell::{Csp, Policy, ScanResult};
use mp_stats_config::{CloudflareConfig, CspConfig};
use std::path::Path;
use std::sync::Arc;

/// Layer the policy for `index_path` onto `router`.
///
/// Fails closed, and fails before the listener binds. The shell boots WebAssembly from an inline
/// `<script>`, so a policy assembled without that script's hash is a blank page; refusing to
/// start names the cause once, in the place that can still do something about it.
///
/// # Errors
///
/// If the shell cannot be read, or if the assembled policy does not render to a valid header
/// value.
pub(crate) fn attach(router: Router, config: &CspConfig, index_path: &Path) -> Result<Router> {
    if !config.enabled {
        eprintln!(
            "warning: `server.csp.enabled` is false - serving without a Content-Security-Policy"
        );
        return Ok(router);
    }

    let scan = csp_shell::scan_shell_at(index_path)
        .with_context(|| format!("scanning {} for inline scripts", index_path.display()))?;

    // Documented scanner limits, not failures: the scan produced hashes, but one of them may
    // cover the wrong text. Reported loudly because the browser's half of this is silent.
    for warning in &scan.warnings {
        eprintln!("warning: {}: {warning}", index_path.display());
    }

    let rendered = Rendered::new(assemble(&config.cloudflare, &scan))
        .context("rendering the Content-Security-Policy")?;

    Ok(router.layer(from_fn_with_state(Arc::new(rendered), set_headers)))
}

/// The policy for a scanned shell, with the concessions `config` asks for.
///
/// Split from [`attach`] so the assembly is a pure function of the two inputs that decide it,
/// which is also what lets the tests below cover it without a file on disk.
fn assemble(config: &CloudflareConfig, scan: &ScanResult) -> Policy {
    let mut csp = Csp::spa_wasm().with_scan(scan);

    if config.turnstile {
        csp = cloudflare::turnstile(csp);
    }
    if config.web_analytics {
        csp = cloudflare::web_analytics(csp);
    }
    if config.script_nonce {
        csp = cloudflare::script_nonce(csp);
    }

    csp.build()
}

/// The header value to attach, in the two shapes a policy comes in.
enum Rendered {
    /// No nonce reserved, so the header never changes: rendered once at startup and cloned onto
    /// every document.
    Constant(HeaderValue),
    /// A nonce slot is reserved, so the header carries a freshly minted value per response — and
    /// with it the `Cache-Control` that keeps one reader's nonce from becoming everyone's.
    PerResponse(Policy),
}

impl Rendered {
    /// Render once, up front.
    ///
    /// For a constant policy that render *is* the value every response carries. For a
    /// per-response one it is a boot-time proof that this policy converts to a header value, so
    /// the conversion in [`set_headers`] — which differs only by the nonce spliced into it —
    /// cannot become the thing that fails while serving.
    fn new(policy: Policy) -> Result<Self> {
        let value = header_value(&policy.headers().content_security_policy)?;

        Ok(if policy.is_per_response() {
            Self::PerResponse(policy)
        } else {
            Self::Constant(value)
        })
    }
}

/// Attach the policy to the documents it governs, and nothing else.
async fn set_headers(
    State(rendered): State<Arc<Rendered>>,
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;

    if !is_document(&response) {
        return response;
    }

    match rendered.as_ref() {
        Rendered::Constant(policy) => {
            response
                .headers_mut()
                .insert(CONTENT_SECURITY_POLICY, policy.clone());
        }
        Rendered::PerResponse(policy) => {
            let csp_shell::Headers {
                content_security_policy,
                cache_control,
                ..
            } = policy.headers();

            let policy = header_value(&content_security_policy)
                .expect("`Rendered::new` rendered this policy to a header value at startup");
            response
                .headers_mut()
                .insert(CONTENT_SECURITY_POLICY, policy);

            // An obligation, not a suggestion: a nonce served from a cache is pinned across every
            // reader of that entry, which admits exactly the inline script it exists to
            // constrain. `Cache-Control` is overwritten rather than appended to for that reason.
            if let Some(cache_control) = cache_control {
                response
                    .headers_mut()
                    .insert(CACHE_CONTROL, HeaderValue::from_static(cache_control));
            }
        }
    }

    response
}

/// Whether this response is a document the policy governs.
///
/// A `Content-Security-Policy` on a stylesheet, a font or a `/data` blob is inert, and the
/// `Cache-Control: no-cache` a per-response nonce obliges would be actively wrong there: the
/// obligation belongs to the document carrying the nonce, not to the assets it pulls in.
fn is_document(response: &Response) -> bool {
    let Some(content_type) = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
    else {
        return false;
    };

    let essence = content_type
        .split_once(';')
        .map_or(content_type, |(essence, _parameters)| essence);
    essence.trim().eq_ignore_ascii_case("text/html")
}

/// Convert a rendered policy into a header value.
///
/// Cannot fail for a policy this crate builds — `csp-policy` refuses every term that could put a
/// byte `http` rejects into the header — but the conversion is checked rather than asserted,
/// because the boot-time call is where that guarantee is worth confirming.
fn header_value(policy: &str) -> Result<HeaderValue> {
    HeaderValue::from_str(policy).with_context(|| format!("`{policy}` is not a valid header value"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::routing::get;
    use tower::ServiceExt as _;

    /// A shell shaped like the one trunk emits: an inline module script that boots the WASM
    /// bundle, which is the script the whole derivation exists for.
    const SHELL: &str = r#"<!DOCTYPE html><html><head>
        <link rel="preload" href="/app_bg.wasm" as="fetch"/>
        <script type="module">import init from '/app.js'; init('/app_bg.wasm');</script>
        </head><body></body></html>"#;

    fn policy_for(config: &CloudflareConfig) -> String {
        assemble(config, &csp_shell::scan_shell(SHELL))
            .headers()
            .content_security_policy
    }

    /// The reason this is derived rather than configured: the hash of the shell's inline
    /// bootstrap reaches `script-src`, so the header and the bundle cannot disagree.
    #[test]
    fn the_shells_inline_script_is_hashed_into_the_policy() {
        let policy = policy_for(&CloudflareConfig::default());

        assert!(
            policy.contains("script-src 'self' 'wasm-unsafe-eval' 'sha256-"),
            "the inline bootstrap must be hashed into script-src: {policy}"
        );
    }

    /// Every concession is opt-in. A default deployment admits no Cloudflare origin and reserves
    /// no nonce, so nothing here widens a policy for a product that is not running.
    #[test]
    fn no_concession_is_made_by_default() {
        let policy = assemble(&CloudflareConfig::default(), &csp_shell::scan_shell(SHELL));

        assert!(!policy.is_per_response());
        assert_eq!(policy.headers().cache_control, None);
        assert!(
            !policy.headers().content_security_policy.contains("nonce-"),
            "no nonce without `script_nonce`"
        );
        assert!(
            !policy
                .headers()
                .content_security_policy
                .contains("cloudflare.com"),
            "no Cloudflare origin without the key that asks for one"
        );
    }

    /// One origin, two directives. Admitting the script without the frame renders an empty
    /// widget, which is the half of this a hand-written policy leaves out.
    #[test]
    fn turnstile_admits_the_widget_host_in_both_directives() {
        let policy = policy_for(&CloudflareConfig {
            turnstile: true,
            ..CloudflareConfig::default()
        });

        assert!(policy.contains("script-src 'self' 'wasm-unsafe-eval' 'sha256-"));
        assert!(policy.contains("https://challenges.cloudflare.com"));
        assert!(
            policy.contains("frame-src 'self' https://challenges.cloudflare.com"),
            "the widget is framed from the same host it is loaded from: {policy}"
        );
    }

    /// Two hosts, and the `static.` prefix on only one of them.
    #[test]
    fn web_analytics_admits_the_beacon_and_its_endpoint() {
        let policy = policy_for(&CloudflareConfig {
            web_analytics: true,
            ..CloudflareConfig::default()
        });

        assert!(policy.contains("https://static.cloudflareinsights.com"));
        assert!(policy.contains("https://cloudflareinsights.com"));
    }

    /// The nonce is what the edge-injected bot-detection script runs under, and the hashes stay
    /// alongside it: under CSP3 a script executes if it matches *any* source expression.
    #[test]
    fn script_nonce_makes_the_policy_per_response() {
        let policy = assemble(
            &CloudflareConfig {
                script_nonce: true,
                ..CloudflareConfig::default()
            },
            &csp_shell::scan_shell(SHELL),
        );

        assert!(policy.is_per_response());
        assert_eq!(policy.headers().cache_control, Some("no-cache"));

        let first = policy.headers().content_security_policy;
        let second = policy.headers().content_security_policy;
        assert_ne!(first, second, "a nonce is minted per response");
        assert!(first.contains("'nonce-"));
        assert!(first.contains("'sha256-"), "the hashes survive the nonce");
    }

    async fn html() -> Response {
        (
            [(CONTENT_TYPE, "text/html; charset=utf-8")],
            "<!doctype html>",
        )
            .into_response()
    }

    async fn blob() -> Response {
        ([(CONTENT_TYPE, "application/octet-stream")], [0_u8; 4]).into_response()
    }

    fn routed(config: &CspConfig, index_path: &Path) -> Router {
        attach(
            Router::new()
                .route("/", get(html))
                .route("/data/blob", get(blob))
                .route("/health/live", get(async || StatusCode::OK)),
            config,
            index_path,
        )
        .expect("the shell written by the caller must scan and render")
    }

    /// Writes `SHELL` somewhere `attach` can read it, since that is the one part of this module
    /// that needs a real file.
    fn shell_on_disk(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("mp-stats-csp-{name}.html"));
        std::fs::write(&path, SHELL).expect("writing the test shell");
        path
    }

    /// The header lands on the document and on nothing else. A `Content-Security-Policy` on a
    /// `/data` blob is inert weight on every response.
    #[tokio::test]
    async fn only_documents_carry_the_policy() {
        let index_path = shell_on_disk("documents-only");
        let router = routed(&CspConfig::default(), &index_path);

        let document = router
            .clone()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert!(
            document
                .headers()
                .get(CONTENT_SECURITY_POLICY)
                .is_some_and(|policy| policy.to_str().unwrap().contains("'sha256-"))
        );

        for path in ["/data/blob", "/health/live"] {
            let response = router
                .clone()
                .oneshot(Request::get(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert!(
                response.headers().get(CONTENT_SECURITY_POLICY).is_none(),
                "{path} is not a document"
            );
        }

        std::fs::remove_file(&index_path).ok();
    }

    /// The first of the two obligations a reserved nonce brings, met on the response itself. The
    /// second — no Cloudflare Cache Rule over the shell — is not observable from here.
    #[tokio::test]
    async fn a_nonced_document_is_served_no_cache() {
        let index_path = shell_on_disk("no-cache");
        let config = CspConfig {
            cloudflare: CloudflareConfig {
                script_nonce: true,
                ..CloudflareConfig::default()
            },
            ..CspConfig::default()
        };

        let response = routed(&config, &index_path)
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(
            response
                .headers()
                .get(CACHE_CONTROL)
                .map(|v| v.to_str().unwrap()),
            Some("no-cache")
        );
        assert!(
            response
                .headers()
                .get(CONTENT_SECURITY_POLICY)
                .is_some_and(|policy| policy.to_str().unwrap().contains("'nonce-"))
        );

        std::fs::remove_file(&index_path).ok();
    }

    /// `enabled = false` reaches the responses, rather than being a key that reads as if it does
    /// something.
    #[tokio::test]
    async fn disabling_the_policy_removes_the_header() {
        let index_path = shell_on_disk("disabled");
        let config = CspConfig {
            enabled: false,
            ..CspConfig::default()
        };

        let response = routed(&config, &index_path)
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert!(response.headers().get(CONTENT_SECURITY_POLICY).is_none());

        std::fs::remove_file(&index_path).ok();
    }

    /// An unreadable shell stops the boot instead of degrading to a hashless policy. The shell is
    /// substantially inline-scripted, so the degraded outcome is a blank page.
    #[test]
    fn an_unreadable_shell_fails_the_boot() {
        let error = attach(
            Router::new(),
            &CspConfig::default(),
            Path::new("does/not/exist/index.html"),
        )
        .expect_err("a missing shell must not yield a policy");

        assert!(
            error.to_string().contains("index.html"),
            "the error must name the file: {error}"
        );
    }
}
