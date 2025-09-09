use tracing_subscriber::{EnvFilter, fmt};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="fks_docs", about="FKS Documentation Service")] 
struct Args {
    /// Listen address (host:port)
    #[arg(long, default_value="0.0.0.0:8040")]
    listen: String,
    /// Docs directory to serve
    #[arg(long)]
    dir: Option<String>,
    /// Disable markdown HTML rendering
    #[arg(long, default_value_t=true)]
    render_md_html: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logging
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();
    let args = Args::parse();
    // Apply CLI overrides via env for reuse of server::run()
    if let Some((host, port)) = args.listen.split_once(':') {
        std::env::set_var("FKS_DOCS_HOST", host);
        if let Ok(p) = port.parse::<u16>() { std::env::set_var("FKS_DOCS_PORT", p.to_string()); }
    } else if let Ok(p) = args.listen.parse::<u16>() { // if only port given
        std::env::set_var("FKS_DOCS_PORT", p.to_string());
    }
    if let Some(dir) = args.dir { std::env::set_var("FKS_DOCS_DIR", dir); }
    if !args.render_md_html { std::env::set_var("FKS_DOCS_RENDER_MD_HTML", "0"); }
    fks_docs::server::run().await
}
