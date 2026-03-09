//! Proxy quarantine persistence helpers.
//!
//! Stores blocked proxies into a separate file so operators can replace them easily.

use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::warn;

pub const DEFAULT_PROXY_QUARANTINE_FILE: &str = "/tmp/downloadtool-quarantined-proxies.txt";

pub fn proxy_quarantine_file_from_env() -> Option<PathBuf> {
    let configured = std::env::var("PROXY_QUARANTINE_FILE")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    match configured {
        Some(path) => Some(PathBuf::from(path)),
        None => Some(PathBuf::from(DEFAULT_PROXY_QUARANTINE_FILE)),
    }
}

pub fn load_quarantined_proxies(path: &Path) -> HashSet<String> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return HashSet::new(),
    };
    let reader = BufReader::new(file);
    let mut quarantined = HashSet::new();

    for line in reader.lines().map_while(Result::ok) {
        if let Some(proxy_url) = extract_proxy_url(&line) {
            quarantined.insert(proxy_url);
        }
    }

    quarantined
}

pub fn append_quarantine_record(path: &Path, proxy_url: &str, reason: &str) {
    if let Some(parent) = path.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            warn!(
                path = %path.display(),
                err = %error,
                "Failed to create proxy quarantine directory"
            );
            return;
        }
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();

    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut file) => {
            if let Err(error) = writeln!(file, "{timestamp}\t{proxy_url}\t{reason}") {
                warn!(
                    path = %path.display(),
                    err = %error,
                    "Failed to append proxy quarantine record"
                );
            }
        }
        Err(error) => {
            warn!(
                path = %path.display(),
                err = %error,
                "Failed to open proxy quarantine file"
            );
        }
    }
}

fn extract_proxy_url(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    // TSV format: <unix_ts>\t<proxy_url>\t<reason>
    let columns: Vec<&str> = trimmed.split('\t').collect();
    if columns.len() >= 2 && columns[1].contains("://") {
        return Some(columns[1].trim().to_string());
    }

    // Backward-compatible plain format.
    if trimmed.contains("://") {
        return Some(trimmed.to_string());
    }

    None
}
