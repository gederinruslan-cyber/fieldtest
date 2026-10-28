use std::fs;

/// Data rows of `engine/fixtures/<name>`: `#` lines and the header row skipped, fields split on
/// commas and trimmed.
pub fn fixture(name: &str) -> Vec<Vec<String>> {
    let path = format!("{}/../../fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .skip(1)
        .map(|l| l.split(',').map(|f| f.trim().to_string()).collect())
        .collect()
}
