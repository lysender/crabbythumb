use std::path::Path;

#[derive(Clone)]
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
        let source_dir = args[3].clone();
        let dest_dir = args[4].clone();

        // Convert width and height to integers
        let width = match width_str.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                return Err("Width must be a number");
            }
        };
        let height = match height_str.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
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

        Ok(Config {
            width,
            height,
            source_dir,
            dest_dir,
        })
    }
}