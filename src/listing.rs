use std::{path::{Path, PathBuf}, fs};
use walkdir::WalkDir;
use chrono::{DateTime, Utc};
use crate::model::DocMeta;

pub fn list_docs(root: &Path) -> anyhow::Result<Vec<DocMeta>> {
    let mut out = Vec::new();
    if !root.exists() { return Ok(out); }
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() { continue; }
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap_or(path);
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_string();
        // Only include common doc extensions
        if !matches!(ext.as_str(), "md" | "markdown" | "txt" | "html" | "htm") { continue; }
        let md = entry.metadata().ok();
        let (modified, size) = md.map(|m| {
            let modified = m.modified().ok().map(|s| DateTime::<Utc>::from(s));
            (modified, m.len())
        }).unwrap_or((None, 0));
        out.push(DocMeta {
            path: rel.to_string_lossy().replace('\\', "/"),
            name: path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string(),
            ext,
            size,
            modified,
        });
    }
    out.sort_by(|a,b| a.path.cmp(&b.path));
    Ok(out)
}

pub fn load_doc(root: &Path, rel: &str) -> anyhow::Result<Option<(DocMeta, String)>> {
    if rel.contains("..") { return Ok(None); }
    let mut full = PathBuf::from(root);
    full.push(rel);
    if !full.exists() { return Ok(None); }
    let content = fs::read_to_string(&full)?;
    let meta_fs = fs::metadata(&full)?;
    let ext = full.extension().and_then(|s| s.to_str()).unwrap_or("").to_string();
    let modified = meta_fs.modified().ok().map(|s| chrono::DateTime::<Utc>::from(s));
    let meta = DocMeta { path: rel.to_string(), name: full.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string(), ext, size: meta_fs.len(), modified };
    Ok(Some((meta, content)))
}
