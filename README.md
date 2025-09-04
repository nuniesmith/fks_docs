# fks_docs

Rust microservice to index and serve project documentation (Markdown/HTML) to the React web UI.

## Features

- List documents: GET /docs/list
- Retrieve document (with ETag + optional rendered HTML): GET /docs/{path}
- Full‑text (line) search: GET /docs/search?q=term&limit=50
- Health: GET /health
- Markdown -> HTML rendering (disable via FKS_DOCS_RENDER_MD_HTML=0)

## Environment Variables

- FKS_DOCS_PORT (default 8040)
- FKS_DOCS_DIR (default ./docs)
- FKS_DOCS_RENDER_MD_HTML (default true)

## Build & Run (host)

```bash
cargo run --release
```

## Docker

`docker-compose.yml` contains service definition `fks_docs`.
