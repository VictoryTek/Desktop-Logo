mod overlay_window;
mod config_watcher;
use std::path::Path;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    use std::process;
    // Determine config file path: XDG config, then bundled, then CWD fallback
    let xdg_config = std::env::var("XDG_CONFIG_HOME").ok()
        .map(|p| Path::new(&p).join("desktop-logo.toml"))
        .filter(|p| p.exists());
    let home_config = dirs::config_dir()
        .map(|p| p.join("desktop-logo.toml"))
        .filter(|p| p.exists());
    let bundled = Path::new("/app/share/desktop-logo-applet/desktop-logo.toml");
    let cwd = Path::new("desktop-logo.toml");
    let cfg_path = xdg_config
        .or(home_config)
        .or_else(|| if bundled.exists() { Some(bundled.to_path_buf()) } else { None })
        .unwrap_or_else(|| cwd.to_path_buf());

    // Watch config file for changes and restart on change (only if file exists)
    if cfg_path.exists() {
        config_watcher::watch_config(cfg_path.clone(), || {
            eprintln!("Config changed, restarting...");
            process::exit(42);
        });
    }

    // Load config
    let raw = std::fs::read_to_string(&cfg_path).with_context(|| format!("reading {}", cfg_path.display()))?;
    let parsed: desktop_logo_applet::LogoAppletConfigToml = toml::from_str(&raw).with_context(|| "parsing desktop-logo.toml")?;

    // Ensure ~/.config/desktop-logo-applet/ exists and has a logo.png
    if let Some(cfg_dir) = dirs::config_dir() {
        let app_dir = cfg_dir.join("desktop-logo-applet");
        if !app_dir.exists() {
            std::fs::create_dir_all(&app_dir).ok();
        }
        let user_logo = app_dir.join("logo.png");
        if !user_logo.exists() {
            let bundled_logo = Path::new("/app/share/desktop-logo-applet/logo.png");
            if bundled_logo.exists() {
                // Try to copy the bundled logo to the config dir
                let _ = std::fs::copy(bundled_logo, &user_logo);
            }
        }
    }

    // Determine logo path: user config dir first, then bundled, then as in config
    let user_logo = dirs::config_dir()
        .map(|p| p.join("desktop-logo-applet/logo.png"))
        .filter(|p| p.exists());
    let bundled_logo = Path::new("/app/share/desktop-logo-applet/logo.png");
    let config_logo = Path::new(&parsed.logo_path);
    let logo_path = user_logo
        .or_else(|| if bundled_logo.exists() { Some(bundled_logo.to_path_buf()) } else { None })
        .unwrap_or_else(|| config_logo.to_path_buf());

    overlay_window::spawn_overlay_window(
        &logo_path,
        &parsed.position,
        parsed.margin,
        parsed.max_logo_percent,
        parsed.opacity,
    );

    // Block forever (overlay runs in its own thread)
    loop { std::thread::park(); }
}
