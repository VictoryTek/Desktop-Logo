mod overlay_window;
mod config_watcher;
use std::path::Path;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    use std::process;
    // Watch config file for changes and restart on change
    config_watcher::watch_config("desktop-logo.toml", || {
        eprintln!("Config changed, restarting...");
        process::exit(42);
    });

    // Load config from desktop-logo.toml in current working directory
    let cfg_path = Path::new("desktop-logo.toml");
    let raw = std::fs::read_to_string(cfg_path).with_context(|| "reading desktop-logo.toml")?;
    let parsed: desktop_logo_applet::LogoAppletConfigToml = toml::from_str(&raw).with_context(|| "parsing desktop-logo.toml")?;

    // Launch the overlay window (click-through logo)
    overlay_window::spawn_overlay_window(
        &parsed.logo_path,
        &parsed.position,
        parsed.margin,
        parsed.max_logo_percent,
        parsed.opacity,
    );

    // Block forever (overlay runs in its own thread)
    loop { std::thread::park(); }
}
