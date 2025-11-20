use desktop_logo_applet::{LogoAppletConfigToml, LogoAppletConfig, Position, run_applet};
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
    let parsed: LogoAppletConfigToml = toml::from_str(&raw).with_context(|| "parsing desktop-logo.toml")?;
    let position = match parsed.position.to_ascii_lowercase().as_str() {
        "topleft" => Position::TopLeft,
        "topright" => Position::TopRight,
        "bottomleft" => Position::BottomLeft,
        _ => Position::BottomRight,
    };
    let config = LogoAppletConfig {
        logo_path: parsed.logo_path.clone(),
        position,
        margin: parsed.margin,
        max_logo_percent: parsed.max_logo_percent,
        opacity: parsed.opacity,
    };

    // Launch the overlay window (click-through logo)
    overlay_window::spawn_overlay_window(
        &parsed.logo_path,
        &parsed.position,
        parsed.margin,
        parsed.max_logo_percent,
        parsed.opacity,
    );

    // Run the main applet (Cosmic)
    run_applet(config).context("running applet")?;
    Ok(())
}
