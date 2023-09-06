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
crabbythumb width height source_dir dest_dir
```

Example:

```shell
crabbythumb 150 125 images/original images/thumbnails
```
