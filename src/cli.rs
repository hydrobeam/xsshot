use clap::{Parser, ValueEnum};
use image::ImageOutputFormat;

#[derive(Parser, Debug)]
#[command(name = "x-sc")]
#[command(author = "Laith Bahodi <laithbahodi@gmail.com>")]
#[command(about = "Basic screenshot tool for X")]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    /// Which window to take a screenshot of
    ///
    /// Defaults to the full screen.
    ///
    /// A subset of the full window class can be provided, a screenshot will be taken
    /// of the first match.
    #[arg(short, long)]
    pub window: Option<String>,

    /// Where the topleft corner of the screenshot is located.
    ///
    /// Defaults to (0, 0).
    #[arg(short, long, num_args(2), default_values=["0", "0"])]
    pub position: Vec<i16>,

    /// Size of the screenshot.
    ///
    /// Defaults to the dimensions of the target window.
    #[arg(short, long, num_args(2))]
    pub size: Option<Vec<u16>>,

    #[arg(short, long, value_enum, default_value = "png")]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Png,
    Jpg,
    Jpeg,
    Gif,
    Bmp,
}

impl From<OutputFormat> for ImageOutputFormat {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Png => ImageOutputFormat::Png,
            OutputFormat::Jpeg => ImageOutputFormat::Jpeg(50),
            OutputFormat::Jpg => ImageOutputFormat::Jpeg(50),
            OutputFormat::Gif => ImageOutputFormat::Gif,
            OutputFormat::Bmp => ImageOutputFormat::Bmp,
        }
    }
}

impl OutputFormat {
    pub fn to_mime_type(self) -> &'static [u8] {
        match self {
            OutputFormat::Png => b"image/png",
            OutputFormat::Jpg | OutputFormat::Jpeg => b"image/jpeg",
            OutputFormat::Gif => b"image/gif",
            OutputFormat::Bmp => b"image/bmp",
        }
    }
}
