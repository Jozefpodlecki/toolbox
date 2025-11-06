use clap::{Parser, ValueEnum};
use image::{imageops::FilterType, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};
use std::path::PathBuf;


#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    Pad,
    Stretch,
}

#[derive(Parser)]
#[command(author, version, about = "Scale image to a square (no cropping).")]
struct Args {
    /// Input image path
    input: PathBuf,

    /// Output image path
    output: PathBuf,

    /// Target square size in pixels (e.g. 2000)
    #[arg(short, long, default_value = "2000")]
    size: u32,

    /// Mode: `pad` (fit and pad) or `stretch` (force to square)
    #[arg(short, long, value_enum, default_value_t = Mode::Pad)]
    mode: Mode,

    /// Background color for padding as hex, e.g. "ffffff" or "000000FF" (default: transparent)
    #[arg(long, default_value = "00000000")]
    bg: String,
}

fn parse_bg(hex: &str) -> Rgba<u8> {
    // Accept 6 or 8 hex chars: RRGGBB or RRGGBBAA
    let s = hex.trim_start_matches('#');
    let bytes = match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&s[6..8], 16).unwrap_or(255);
            (r, g, b, a)
        }
        _ => (0, 0, 0, 0),
    };
    Rgba([bytes.0, bytes.1, bytes.2, bytes.3])
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let img = image::open(&args.input).map_err(|e| {
        anyhow::anyhow!("Failed to open input {:?}: {}", args.input.to_string_lossy(), e)
    })?;

    let target = args.size.max(1);
    match args.mode {
        Mode::Stretch => {
            let squeezed = img.resize_exact(target, target, FilterType::Lanczos3);
            squeezed
                .save(&args.output)
                .map_err(|e| anyhow::anyhow!("Failed to save output: {}", e))?;
            println!(
                "Wrote stretched {}x{} -> {}x{}",
                img.width(),
                img.height(),
                target,
                target
            );
        }
        Mode::Pad => {
            let (w, h) = img.dimensions();
            let max_dim = w.max(h);
            let new_w = (w as u128 * target as u128 / max_dim as u128) as u32;
            let new_h = (h as u128 * target as u128 / max_dim as u128) as u32;

            let resized = img.resize(new_w, new_h, FilterType::Lanczos3);

            let bg_rgba = parse_bg(&args.bg);
            let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> =
                ImageBuffer::from_pixel(target, target, bg_rgba);

            let resized_rgba: DynamicImage = match resized {
                DynamicImage::ImageRgba8(_) => resized,
                other => other.to_rgba8().into(),
            };

            let x = (target.saturating_sub(new_w)) / 2;
            let y = (target.saturating_sub(new_h)) / 2;

            canvas
                .copy_from(&resized_rgba.to_rgba8(), x, y)
                .map_err(|e| anyhow::anyhow!("Failed to composite image: {}", e))?;

            DynamicImage::ImageRgba8(canvas)
                .save(&args.output)
                .map_err(|e| anyhow::anyhow!("Failed to save output: {}", e))?;

            println!(
                "Wrote fitted+padded {}x{} -> {}x{} (resized to {}x{}, padded at {},{})",
                w, h, target, target, new_w, new_h, x, y
            );
        }
    }

    Ok(())
}
