use std::fs;
use std::path::Path;
use exif::{In, Tag};
use image::imageops;
use std::error::Error;

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

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut handles = vec![];

    // Ideally, we don't want to store the list of files in memory
    // so we will try to refactor this to create thumbnail per iteration on readdir
    let files = list_files(&config.source_dir)?;
    for file in files {
        // Clone parameters, there must be a better way to do this right?
        let current_file = file.clone();
        let current_config = config.clone();

        let handle = tokio::spawn(async move {
            create_thumb(&current_file, &current_config).await.unwrap();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
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

            // Ensure we only process these types of images for now...
            if ["jpg", "png", "gif", "jpeg"].contains(&ext.to_lowercase().as_str()) {
                files.push(String::from(filename));
            }
        }
    }
    Ok(files)
}

fn parse_exif_orientation(path: &Path) -> Result<u32, Box<dyn Error>> {
    let file = fs::File::open(path)?;

    let mut buf_reader = std::io::BufReader::new(&file);
    let exit_reader = exif::Reader::new();
    let exif = exit_reader.read_from_container(&mut buf_reader)?;

    // Default to 1 if cannot identify orientation
    let result = match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => {
            match orientation.value.get_uint(0) {
                Some(v @ 1..=8) => v,
                _ => 1,
            }
        },
        None => 1,
    };

    Ok(result)
}

async fn create_thumb(filename: &String, config: &Config) -> Result<(), Box<dyn Error>> {
    let source_file = Path::new(config.source_dir.as_str()).join(filename);
    let dest_file = Path::new(config.dest_dir.as_str()).join(filename);

    let orientation = match parse_exif_orientation(&source_file) {
        Ok(v) => v,
        Err(_) => 1,
    };

    let img = image::open(source_file)?;

    // Rotate based on exit orientation before cropping
    let mut rotated_img = match orientation {
        8 => img.rotate90(),
        3 => img.rotate180(),
        6 => img.rotate90(),
        _ => img
    };

    let source_width = rotated_img.width();
    let source_height = rotated_img.height();

    // This one is brought to you by chad jipitty
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

    // Crop the image using scaled dimensions, cutting off some parts
    let cropped = rotated_img.crop(x_offset, y_offset, crop_width, crop_height);

    // Resize the cropped image to the desired dimensions
    let resized_img = cropped.resize_exact(config.width, config.height, imageops::FilterType::Lanczos3);

    // Save the resized image
    resized_img.save(dest_file)?;

    println!("{}", filename);

    Ok(())
}
