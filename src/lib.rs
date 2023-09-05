use std::fs;
use std::path::Path;
use std::error::Error;
use image::{GenericImageView, ImageBuffer, RgbImage, imageops, DynamicImage, GenericImage};

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

    let files = list_files(&config.source_dir)?;
    for file in files {
        println!("file: {}", file);
        create_thumb(&file, &config)?;
    }
    Ok(())
}

fn list_files(dir: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files: Vec<String> = Vec::new();
    let path = Path::new(dir);
    for entry in fs::read_dir(path)? {
        if let Ok(item) = entry {
            let item_path = item.path();
            let ext = item_path.extension().unwrap().to_str().unwrap();
            let filename = item_path.file_name().unwrap().to_str().unwrap();

            if ["jpg", "png", "gif", "jpeg"].contains(&ext.to_lowercase().as_str()) {
                files.push(String::from(filename));
            }
        }
    }
    Ok(files)
}

fn create_thumb(filename: &String, config: &Config) -> Result<(), Box<dyn Error>>{
    let source_file = Path::new(config.source_dir.as_str()).join(filename);
    let dest_file = Path::new(config.dest_dir.as_str()).join(filename);

    let img = image::open(source_file)?;
    // Copy image to remove exif information
    let mut no_exif = DynamicImage::new_rgb8(img.width(), img.height());
    no_exif.copy_from(&img, 0, 0);

    let cropped = no_exif.thumbnail_exact(config.width, config.height);
    cropped.save(dest_file)?;

    Ok(())
}

fn create_thumb_v3(filename: &String, config: &Config) -> Result<(), Box<dyn Error>>{
    let source_file = Path::new(config.source_dir.as_str()).join(filename);
    let dest_file = Path::new(config.dest_dir.as_str()).join(filename);

    let mut img = image::open(source_file)?;
    let cropped = img.thumbnail_exact(config.width, config.height);
    cropped.save(dest_file)?;

    Ok(())
}

fn create_thumb_v2(filename: &String, config: &Config) -> Result<(), Box<dyn Error>>{
    let source_file = Path::new(config.source_dir.as_str()).join(filename);
    let source_file_str = source_file.to_str().unwrap();
    println!("source_file_str, {}", source_file_str);

    let dest_file = Path::new(config.dest_dir.as_str()).join(filename);
    let dest_file_str = dest_file.to_str().unwrap();
    println!("dest_file_str: {}", dest_file_str);

    let mut img = image::open(source_file)?;

    // let cropped = img.thumbnail_exact(config.width, config.height);

    let source_width = img.width();
    let source_height = img.height();

    let aspect_ratio = config.width as f32 / config.height as f32;
    let current_aspect_ratio = source_width as f32 / source_height as f32;

    let (crop_width, crop_height, x_offset, y_offset) = if current_aspect_ratio > aspect_ratio {
        // Crop horizontally (landscape mode)
        let crop_width = (source_height as f32 * aspect_ratio) as u32;
        let x_offset = (source_width - crop_width) / 2;
        (crop_width, source_height, x_offset, 0)
    } else {
        // Crop vertically (portrait mode)
        let crop_height = (source_width as f32 / aspect_ratio) as u32;
        let y_offset = (source_height - crop_height) / 2;
        (source_width, crop_height, 0, y_offset)
    };

    // Crop the image
    let cropped = img.crop(x_offset, y_offset, crop_width, crop_height);

    // Resize the cropped image to the desired dimensions
    let resized_img = cropped.resize_exact(config.width, config.height, imageops::FilterType::Lanczos3);


    // Save the resized image
    resized_img.save(dest_file)?;

    // The dimensions method returns the images width and height.
    // println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    // println!("{:?}", img.color());
    Ok(())
}
