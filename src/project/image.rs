use std::path::{Path, PathBuf};
use std::fs;
use std::result::Result;
use image::ImageReader;

// Errors that can occur during image operation
#[derive(Debug)]
pub enum ImageError {
    NotFound(PathBuf),
    DecodeError(String),
    EncodeError(String),
    Io(std::io::Error),    
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::NotFound(p) => write!(f, "Image not found: {}", p.display()),
            ImageError::DecodeError(msg) => write!(f, "Failed to decode image: {}", msg),
            ImageError::EncodeError(msg) => write!(f, "Failed to encode WebP: {}", msg),
            ImageError::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl From<std::io::Error> for ImageError {
    fn from(e: std::io::Error) -> Self {
        ImageError::Io(e)
    }
}

/// Imports an image into the project, converts it to WebP,
/// and saves it into the assets/images/ directory.
///
/// Returns the path of the saved WebP file.
pub fn import_image(source: &Path, assets: &Path, image_id: &uuid::Uuid) -> Result<PathBuf, ImageError> {
    if !source.exists() {
        return Err(ImageError::NotFound(source.to_path_buf()));
    }

    // Ensure the assets/images folder exists
    let images_dir = assets.join("images");
    fs::create_dir_all(&images_dir)?;

    // Load the source image
    let img = ImageReader::open(source)
        .map_err(|e| ImageError::DecodeError(e.to_string()))?
        .decode()
        .map_err(|e| ImageError::DecodeError(e.to_string()))?;

    // Convert to WebP and save with UUID as filename
    let output = images_dir.join(format!("{}.webp", image_id));

    img.save(&output).map_err(|e| ImageError::EncodeError(e.to_string()))?;

    Ok(PathBuf::from("assets/images").join(format!("{}.webp", image_id)))
}

/// Importas a cover image into the project
/// 
/// The cover always have a fixed filename and path, as there's only one
/// cover per project. If a cover already exists, it's overwritten.
pub fn import_cover(source: &Path, assets_dir: &Path) -> Result<PathBuf, ImageError> {
    if !source.exists() {
        return Err(ImageError::NotFound(source.to_path_buf()));
    }

    fs::create_dir_all(assets_dir)?;

    let img = ImageReader::open(source)
        .map_err(|e| ImageError::DecodeError(e.to_string()))?
        .decode()
        .map_err(|e| ImageError::DecodeError(e.to_string()))?;

    let output = assets_dir.join("cover.webp");

    img.save(&output).map_err(|e| ImageError::EncodeError(e.to_string()))?;

    Ok(PathBuf::from("assets/cover.webp"))
}

/// Converts a WebP image to JPEG or PNG for legacy EPUB export
/// 
/// Called by export::compat when user enables legacy reader support.
/// Images with alpha channel are converted to PNG, any other images are converted to JPEG.
pub fn convert_legacy(source: &Path) -> Result<PathBuf, ImageError> {
    if !source.exists() {
        return Err(ImageError::NotFound(source.to_path_buf()));
    }

    let img = ImageReader::open(source)
        .map_err(|e| ImageError::DecodeError(e.to_string()))?
        .decode()
        .map_err(|e| ImageError::DecodeError(e.to_string()))?;

    // Check if the image has an alpha channel (transparency)
    let has_alpha = matches!(
        img.color(),
        image::ColorType::Rgb8
            | image::ColorType::Rgba16
            | image::ColorType::La8
            | image::ColorType::La16
    );

    let output = if has_alpha {
        source.with_extension("png")
    } else {
        source.with_extension("jpg")
    };

    img.save(&output).map_err(|e| ImageError::EncodeError(e.to_string()))?;

    Ok(output)
}