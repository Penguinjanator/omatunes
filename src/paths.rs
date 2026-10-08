use std::path::PathBuf;

fn config_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(home).join(".config/omatunes")
}

pub fn db() -> PathBuf {
    config_dir().join("db.json")
}

pub fn stats() -> PathBuf {
    config_dir().join("stats.json")
}

pub fn config_toml() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn library_cache() -> PathBuf {
    config_dir().join("library_cache.json")
}

/// Lock file held for the lifetime of the running instance. Lives in
/// `$XDG_RUNTIME_DIR` (per-user tmpfs) when available.
pub fn instance_lock() -> PathBuf {
    std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(config_dir)
        .join("omatunes.lock")
}

