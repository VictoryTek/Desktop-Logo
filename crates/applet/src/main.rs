use desktop_logo_applet::{LogoAppletConfigToml, LogoAppletConfig, Position, run_applet};
use std::path::Path;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
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
        logo_path: parsed.logo_path,
        position,
        margin: parsed.margin,
        max_logo_percent: parsed.max_logo_percent,
        opacity: parsed.opacity,
    };
    run_applet(config).context("running applet")?;
    Ok(())
}
