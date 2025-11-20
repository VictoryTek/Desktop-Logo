use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogoAppletConfigToml {
    pub logo_path: PathBuf,
    #[serde(default = "default_position")] pub position: String,
    #[serde(default = "default_margin")] pub margin: u32,
    #[serde(default = "default_max_logo_percent")] pub max_logo_percent: f32,
    #[serde(default = "default_opacity")] pub opacity: f32,
}

fn default_position() -> String { "BottomRight".into() }
fn default_margin() -> u32 { 64 }
fn default_max_logo_percent() -> f32 { 0.18 }
fn default_opacity() -> f32 { 0.85 }
