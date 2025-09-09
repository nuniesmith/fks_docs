use std::{path::Path, fs};
use walkdir::WalkDir;
use crate::model::{SearchResponse, SearchResultEntry};
use percent_encoding::percent_decode_str;
use std::time::Instant;

pub fn search(root: &Path, query: &str, limit: usize) -> anyhow::Result<SearchResponse> {
    let decoded = percent_decode_str(query).decode_utf8_lossy();
    let term = decoded.trim();
    let start = Instant::now();
    let mut results: Vec<SearchResultEntry> = Vec::new();
    if term.is_empty() { return Ok(SearchResponse { query: term.into(), total_results: 0, results, took_ms: 0 }); }
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() { continue; }
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if !matches!(ext, "md" | "markdown" | "txt" | "html" | "htm") { continue; }
        let Ok(content) = fs::read_to_string(path) else { continue; };
        let mut score = 0u32;
        let mut snippets = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if line.to_ascii_lowercase().contains(&term.to_ascii_lowercase()) {
                score += 1;
                if snippets.len() < 5 {
                    snippets.push(format!("{}: {}", i+1, line.trim()));
                }
            }
        }
        if score > 0 {
            let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/");
            results.push(SearchResultEntry { path: rel, score, line_snippets: snippets });
        }
        if results.len() >= limit { break; }
    }
    results.sort_by(|a,b| b.score.cmp(&a.score).then(a.path.cmp(&b.path)));
    let took = start.elapsed().as_millis();
    let total = results.len();
    Ok(SearchResponse { query: term.into(), total_results: total, results, took_ms: took })
}
