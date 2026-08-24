//! No code. `release-please` versions packages, and the workspace root has to be one so that the
//! README generator has a manifest to read, so the root package needs a target and this is it.
//!
//! The platform is three programs over one directory of files. `apps/converter` rewrites the raw
//! dumps under `data/` into the sharded tree documented at the root of `mp-stats-core`,
//! `apps/server` serves that tree and the built frontend as static files, and `apps/frontend` is
//! the Yew client that fetches out of it and does the querying in the browser.
