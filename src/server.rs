use std::{path::PathBuf, sync::Arc};
use axum::{Router, routing::get, extract::{Path, State, Query}, http::{StatusCode, HeaderMap}, response::IntoResponse};
use tower_http::{cors::CorsLayer, compression::CompressionLayer, trace::TraceLayer};
use serde::Deserialize;
use crate::{listing, search};
use crate::model::{DocContent};
use sha2::{Sha256, Digest};
use pulldown_cmark::{Parser, Options, html};
use tracing::{info, error};

#[derive(Clone)]
pub struct AppState {
    pub docs_root: Arc<PathBuf>,
    pub render_markdown: bool,
}

pub async fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/docs/list", get(list_docs))
        .route("/docs/search", get(search_docs))
    .route("/docs/{*path}", get(get_doc))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
}

async fn health() -> impl IntoResponse { axum::Json(serde_json::json!({"status":"ok","service":"fks_docs","ts": chrono::Utc::now()})) }

async fn list_docs(State(state): State<AppState>) -> impl IntoResponse {
    match listing::list_docs(&state.docs_root) {
        Ok(list) => axum::Json(list).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("error: {e}")).into_response()
    }
}

#[derive(Deserialize)]
struct SearchParams { q: String, #[serde(default="default_limit")] limit: usize }
fn default_limit() -> usize { 50 }

async fn search_docs(State(state): State<AppState>, Query(q): Query<SearchParams>) -> impl IntoResponse {
    match search::search(&state.docs_root, &q.q, q.limit.min(200)) {
        Ok(resp) => axum::Json(resp).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("error: {e}")).into_response()
    }
}

async fn get_doc(State(state): State<AppState>, Path(path): Path<String>, headers: HeaderMap) -> impl IntoResponse {
    let rel = path.trim_start_matches('/') ;
    match listing::load_doc(&state.docs_root, rel) {
        Ok(Some((meta, content))) => {
            // Compute etag
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            let etag = format!("\"{:x}\"", hasher.finalize());
            if let Some(if_none) = headers.get("if-none-match").and_then(|v| v.to_str().ok()) {
                if if_none == etag { return (StatusCode::NOT_MODIFIED, [("ETag", etag)]).into_response(); }
            }
            let rendered_html = if state.render_markdown && matches!(meta.ext.as_str(), "md" | "markdown") {
                let mut opts = Options::empty();
                opts.insert(Options::ENABLE_TABLES);
                let parser = Parser::new_ext(&content, opts);
                let mut html_out = String::new();
                html::push_html(&mut html_out, parser);
                Some(html_out)
            } else { None };
            let doc = DocContent { meta, etag: etag.clone(), content, rendered_html };
            let mut resp = axum::Json(doc).into_response();
            resp.headers_mut().insert("ETag", etag.parse().unwrap());
            resp
        }
        Ok(None) => (StatusCode::NOT_FOUND, "not found").into_response(),
        Err(e) => {
            error!(error=?e, "load_doc failed");
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }
}

pub async fn run() -> anyhow::Result<()> {
    let host = std::env::var("FKS_DOCS_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = std::env::var("FKS_DOCS_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8040);
    let docs_dir = std::env::var("FKS_DOCS_DIR").unwrap_or_else(|_| "docs".into());
    let render_md = std::env::var("FKS_DOCS_RENDER_MD_HTML").map(|v| matches!(v.as_str(), "1"|"true"|"TRUE"|"yes"|"on")).unwrap_or(true);
    let root = PathBuf::from(&docs_dir);
    info!(?root, host, port, render_md, "starting fks_docs server");
    let state = AppState { docs_root: Arc::new(root), render_markdown: render_md };
    let app = build_router(state).await;
    let bind_addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
