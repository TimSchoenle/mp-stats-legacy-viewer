//! The `Content-Security-Policy` block: whether the server sends one, and which third-party
//! concessions it makes room for.
//!
//! What is *not* here is the policy itself. The header the server sends is derived from the
//! shell it is about to serve — every inline `<script>` in `index.html` hashed at startup by
//! [`csp-shell`](https://github.com/TimSchoenle/csp-shell) — because a policy written out in
//! configuration drifts from the bundle the moment anyone rebuilds the frontend, and the browser
//! reports that drift by refusing the scripts and rendering nothing.
//!
//! So the keys below are only the part that genuinely differs between deployments: which
//! Cloudflare products this deployment has switched on. Each is a concession to something
//! Cloudflare runs on the served page, and each costs something, which is why none of them is
//! on by default.

use serde::Deserialize;

/// Whether the server attaches a `Content-Security-Policy`, and what it makes room for.
#[derive(Debug, Clone, Deserialize)]
pub struct CspConfig {
    /// Send the header at all.
    ///
    /// On by default, and the only reason to turn it off is that something in front of this
    /// server already sets the header — two `Content-Security-Policy` headers on one response
    /// are intersected by the browser, so the effective policy becomes the strictest reading of
    /// both and the page breaks in a way neither policy explains. Disabling it to make a page
    /// work is fixing the symptom: the console names the directive that refused the resource,
    /// and the fix belongs in the shell or in the keys below.
    #[serde(default = "CspConfig::default_enabled")]
    pub enabled: bool,
    /// Concessions to the Cloudflare products running in front of this deployment.
    #[serde(default)]
    pub cloudflare: CloudflareConfig,
}

impl CspConfig {
    const fn default_enabled() -> bool {
        true
    }
}

impl Default for CspConfig {
    fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
            cloudflare: CloudflareConfig::default(),
        }
    }
}

/// Which Cloudflare products the policy has to accommodate.
///
/// All off by default: the origins and the nonce below are only correct for a deployment that
/// actually runs these products, and a policy that admits what it does not need is a policy that
/// permits what it does not need.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CloudflareConfig {
    /// Reserve a per-response nonce in `script-src`.
    ///
    /// Needed by every Cloudflare product that injects an inline `<script>` **at the edge** —
    /// Bot Fight Mode, JavaScript Detections, the challenge platform. That script did not exist
    /// when the shell was hashed, so `script-src` refuses it and the detection silently never
    /// runs: bot management appears enabled and does nothing. Cloudflare's documented answer is
    /// to read the nonce out of the `Content-Security-Policy` response header and copy it onto
    /// what it injects, which is why nothing has to be stamped into the shell.
    ///
    /// Two obligations come with it, and only the first is met from inside this process:
    ///
    /// 1. The shell must be served `Cache-Control: no-cache`. The server does that for every
    ///    document it answers with while this key is `true` — a nonce served from a cache is
    ///    shared by every reader of that cache entry, which admits exactly the inline script the
    ///    nonce exists to constrain.
    /// 2. No Cloudflare Cache Rule may cache the shell. A "Cache Everything" rule overrides the
    ///    origin `Cache-Control`, satisfying the first obligation at the origin and violating it
    ///    at the edge. Nothing here can detect that; it belongs in the deployment checklist.
    ///
    /// The concession is real but narrow: an injected script that can already run could read the
    /// header back off a same-origin fetch and admit further inline script. It cannot forge a
    /// nonce ahead of time — 128 CSPRNG bits, minted per response — and it reaches no off-origin
    /// host that the rest of the policy does not already allow.
    #[serde(default)]
    pub script_nonce: bool,
    /// Admit `https://challenges.cloudflare.com` in `script-src` **and** `frame-src`.
    ///
    /// For a Turnstile widget rendered *in* a page this server serves. One origin, two
    /// directives, because Turnstile loads `api.js` and then frames the widget from the same
    /// host; admitting the script without the frame renders an empty box.
    ///
    /// A managed-challenge interstitial needs nothing here — that is a Cloudflare-served
    /// document carrying its own policy.
    #[serde(default)]
    pub turnstile: bool,
    /// Admit Cloudflare Web Analytics: the beacon script, and the endpoint it reports to.
    ///
    /// For the manual snippet only. The automatic injection Cloudflare performs at the edge is an
    /// inline `<script>` this server never saw, so it needs [`script_nonce`](Self::script_nonce)
    /// rather than these origins.
    #[serde(default)]
    pub web_analytics: bool,
}
