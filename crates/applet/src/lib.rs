use std::path::{Path, PathBuf};
use anyhow::{Result, Context, bail};
use cosmic::iced::{Length};
use cosmic::iced::widget::{image, row, column, horizontal_space, vertical_space};
use cosmic::widget::{container};
use cosmic::app::{Core, Settings, Task};
use cosmic::{Application, Element};
use serde::Deserialize;

#[derive(Clone, Copy, Debug)]
pub enum Position { TopLeft, TopRight, BottomLeft, BottomRight }
fn parse_position(s: &str) -> Position {
    match s.to_ascii_lowercase().as_str() {
        "topleft" => Position::TopLeft,
        "topright" => Position::TopRight,
        "bottomleft" => Position::BottomLeft,
        "bottomright" => Position::BottomRight,
        _ => Position::BottomRight,
    }
}

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

pub struct LogoAppletConfig {
    pub logo_path: PathBuf,
    pub position: Position,
    pub margin: u32,
    pub max_logo_percent: f32,
    pub opacity: f32,
}

pub struct LogoApplet {
    pub config: LogoAppletConfig,
    pub core: Core,
    logo_handle: Option<cosmic::iced::widget::image::Handle>,
    logo_dims: (u32,u32),
}

impl LogoApplet {
    pub fn new(config: LogoAppletConfig, core: Core) -> Self { Self { config, core, logo_handle: None, logo_dims: (0,0) } }

    pub fn from_toml_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).with_context(|| format!("reading config {:?}", path))?;
        let raw: LogoAppletConfigToml = toml::from_str(&content).with_context(|| "parsing toml config")?;
        let position = parse_position(&raw.position);
        Ok(Self::new(LogoAppletConfig {
            logo_path: raw.logo_path,
            position,
            margin: raw.margin,
            max_logo_percent: raw.max_logo_percent,
            opacity: raw.opacity,
        }, Core::default()))
    }
    fn load_logo(&mut self) -> Result<()> {
        if !self.config.logo_path.exists() { bail!("logo path does not exist: {:?}", self.config.logo_path); }
        // Load to get intrinsic dimensions
        let img = image::Handle::from_path(self.config.logo_path.clone());
        // We cannot read dimensions from Handle directly without decoding; do a lightweight decode via image crate.
        let dyn_img = image::open(&self.config.logo_path)?;
        let (w,h) = dyn_img.dimensions();
        // scale based on shortest side * percent? For overlay, treat panel size assumption ~ 800px min reference.
        let reference = w.min(h) as f32;
        let target = (reference * self.config.max_logo_percent).max(1.0);
        let scale = target / reference;
        let scaled_w = (w as f32 * scale).round().max(1.0) as u32;
        let scaled_h = (h as f32 * scale).round().max(1.0) as u32;
        self.logo_handle = Some(img);
        self.logo_dims = (scaled_w, scaled_h);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Message { Init }

impl Application for LogoApplet {
    type Executor = cosmic::executor::Default;
    type Flags = LogoAppletConfig; // pass parsed config directly
    type Message = Message;
    const APP_ID: &'static str = "com.example.DesktopLogoApplet";

    fn core(&self) -> &Core { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let applet = LogoApplet { config: flags, core, logo_handle: None, logo_dims: (0,0) };
        (applet, Task::perform(async {}, |_| Message::Init))
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Init => {
                let _ = self.load_logo();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Build layout with corner positioning
        if let (Some(handle), (w,h)) = (&self.logo_handle, self.logo_dims) {
            let img = image(handle.clone())
                .width(Length::Fixed(w as f32))
                .height(Length::Fixed(h as f32));
            let margin = self.config.margin as f32;
            let positioned: Element<_> = match self.config.position {
                Position::TopLeft => row!(img).into(),
                Position::TopRight => row!(horizontal_space(Length::Fill), img).into(),
                Position::BottomLeft => column!(vertical_space(Length::Fill), img).into(),
                Position::BottomRight => column!(vertical_space(Length::Fill), row!(horizontal_space(Length::Fill), img)).into(),
            };
            container(positioned).padding(margin).into()
        } else {
            container(cosmic::widget::text("Loading logo...")).into()
        }
    }
}

// Helper to launch the applet (used by binary main)
pub fn run_applet(config: LogoAppletConfig) -> cosmic::iced::Result {
    cosmic::applet::run::<LogoApplet>(config)
}
