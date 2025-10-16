use std::path::PathBuf;
use clap::Parser;
use desktop_logo_core::{CompositeOptions, Position, composite_logo};
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about = "Composite a logo onto a wallpaper.")]
struct Args {
    /// Path to base wallpaper
    #[arg(long)]
    base: PathBuf,
    /// Path to logo image (PNG recommended)
    #[arg(long)]
    logo: PathBuf,
    /// Output path (inferred format by extension)
    #[arg(long)]
    out: PathBuf,
    /// Position: tl, tr, bl, br
    #[arg(long, default_value = "br")]
    position: String,
    /// Margin in pixels
    #[arg(long, default_value_t = 64)]
    margin: u32,
    /// Max logo percent of shortest side (0-1)
    #[arg(long, default_value_t = 0.18)]
    max_logo_percent: f32,
    /// Opacity (0-1)
    #[arg(long, default_value_t = 0.85)]
    opacity: f32,
}

fn parse_position(s: &str) -> Result<Position> {
    Ok(match s.to_ascii_lowercase().as_str() {"tl"=>Position::TopLeft,"tr"=>Position::TopRight,"bl"=>Position::BottomLeft,"br"=>Position::BottomRight,_=> anyhow::bail!("invalid position; use tl|tr|bl|br")})
}

fn main() -> Result<()> {
    let args = Args::parse();
    let position = parse_position(&args.position)?;
    let opts = CompositeOptions {
        base_path: &args.base,
        logo_path: &args.logo,
        output_path: &args.out,
        position,
        margin: args.margin,
        max_logo_percent: args.max_logo_percent,
        opacity: args.opacity,
    };
    composite_logo(&opts)?;
    println!("Wrote {}", args.out.display());
    Ok(())
}
