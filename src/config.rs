use std::path::PathBuf;
use clap::Parser;

#[derive(Clone, Debug)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub source_dir: PathBuf,
    pub dest_dir: PathBuf,
}

impl Config {
    pub fn build(args: Args) -> Result<Config, &'static str> {
        // Check for correct dimensions for thumbnails
        if args.width < 100 || args.width > 200 {
            return Err("Width must be between 100 to 200 pixels");
        }
        if args.height < 100 || args.height > 200 {
            return Err("Height must be between 100 to 200 pixels");
        }
        if args.height > args.width {
            return Err("Width must be greater than or equal to height.");
        }

        if args.source_dir == args.dest_dir {
            return Err("Source dir and dest dir must be different.");
        }

        // Check whether the source or dest dirs are valid
        if !args.source_dir.is_dir() {
            return Err("Source dir must exist.");
        }
        if !args.dest_dir.is_dir() {
            return Err("Dest dir must exist.");
        }

        Ok(Config {
            width: args.width,
            height: args.height,
            source_dir: args.source_dir,
            dest_dir: args.dest_dir,
        })
    }
}

/// CLI tool to create thumbnail images from source_dir to dest_dir
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Thumbnail image width
    #[arg(long, default_value_t = 150)]
    pub width: u32,

    /// Thumbnail image height
    #[arg(long, default_value_t = 125)]
    pub height: u32,

    /// Source directory containing the original images
    #[arg(short, long)]
    pub source_dir: PathBuf,

    /// Destination directory to save the generated thumbnail images
    #[arg(short, long)]
    pub dest_dir: PathBuf,
}

