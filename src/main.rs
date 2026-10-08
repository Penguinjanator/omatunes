mod app;
mod audio;
mod config;
mod db;
mod library;
mod locale;
mod paths;
mod stats;
mod ui;

fn main() -> iced::Result {
    if std::env::var_os("RUST_LOG").is_some() {
        let _ = env_logger::try_init();
    }

    // Single instance: the lock stays held until the process exits. A second
    // launch asks the running instance to show its window instead.
    let instance_lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(paths::instance_lock())
        .ok();
    if let Some(lock) = &instance_lock {
        if let Err(std::fs::TryLockError::WouldBlock) = lock.try_lock() {
            if let Ok(socket) = std::net::UdpSocket::bind("127.0.0.1:0") {
                let _ = socket.send_to(b"show", "127.0.0.1:18888");
            }
            return Ok(());
        }
    }

    if std::env::var_os("WGPU_BACKEND").is_none() {
        std::env::set_var("WGPU_BACKEND", "gl,vulkan");
    }
    config::load();   // first: define music_dir, language, volume…
    db::init();       // initialize local database
    stats::init();    // initialize listening statistics
    locale::load();
    ui::theme::load_system_theme();
    // Starts in the background (bar widget / MPRIS only); `--show` opens the
    // window right away.
    let res = app::run(std::env::args().any(|a| a == "--show"));
    // Best-effort synchronous flush on normal exit (covers close via window X, SIGTERM, etc.).
    // Does not cover SIGKILL (kill -9) — for that, high-value writes now flush() immediately (see db.rs).
    // Liked pending writes: db.json already has liked state (crash-safe), file tag is eventual mirror
    // flush_sync drains any pending liked tag writes for the currently playing track (fd now closed)
    crate::library::pending::flush_sync();
    db::flush_sync();
    stats::flush_sync();
    res
}
