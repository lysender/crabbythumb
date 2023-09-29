# crabbythumb

Creates thumbnails of images from `source_dir` to `dest_dir`:
- `source_dir` must contain valid JPG, GIF or PNG images.
- Images are cropped and resized to the exact dimension provided.
- Cropping is positioned at the top of the image, removing excess area where necessary.
- Images with EXIF orientation tags are automatically rotated.
- Thumbnails are stored at `dest_dir` using the same filename.

## Build

Clone the repo and build using cargo.

```shell
cargo build --release
```

## Usage

```shell
crabbythumb --width 150 --height 125 --source_dir images/original --dest_dir images/thumbnails
```

Note: width and height are optional and defaults to 150x125 pixels.
