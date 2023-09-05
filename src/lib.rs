use std::{env, fs};
use std::error::Error;
use std::path::Path;

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub source_dir: String,
    pub dest_dir: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 5 {
            return Err("Usage: crabbythumb width height source_dir dest_dir");
        }

        let width_str = args[1].clone();
        let height_str = args[2].clone();
        let mut source_dir = args[3].clone();
        let mut dest_dir = args[4].clone();

        // Convert width and height to integers
        let width = match width_str.parse::<u32>() {
            Ok(num) => num,
            Err(err) => {
                return Err("Width must be a number");
            }
        };
        let height = match height_str.parse::<u32>() {
            Ok(num) => num,
            Err(err) => {
                return Err("Height must be a number");
            }
        };

        // Check for correct dimensions for thumbnails
        if width < 100 || width > 200 {
            return Err("Width must be between 100 to 200 pixels");
        }
        if height < 100 || height > 200 {
            return Err("Height must be between 100 to 200 pixels");
        }
        if height > width {
            return Err("Width must be greater than or equal to height.");
        }

        if source_dir == dest_dir {
            return Err("Source dir and dest dir must be different.");
        }

        // Check whether the source or dest dirs are valid
        if !Path::new(&source_dir).is_dir() {
            return Err("Source dir must exist.");
        }
        if !Path::new(&dest_dir).is_dir() {
            return Err("Dest dir must exist.");
        }

        // Convert paths to canonical paths
        // source_dir = String::from(fs::canonicalize(source_dir).unwrap().to_str().unwrap());
        // dest_dir = String::from(fs::canonicalize(dest_dir).unwrap().to_str().unwrap());

        Ok(Config {
            width,
            height,
            source_dir,
            dest_dir,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Source: {}", config.source_dir);
    println!("Destination: {}", config.dest_dir);
    Ok(())
}
