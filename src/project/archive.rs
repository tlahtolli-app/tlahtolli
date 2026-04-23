/// This module handles compression and decompression of .tl file using 7zip.

use std::path::{Path, PathBuf};
use std::fs::{self, remove_file};
use sevenz_rust::{Archive, SevenZWriter, SevenZReader};

// Errors that can occur
#[derive(Debug)]
pub enum ArchiveError {
    NotFound(PathBuf), // file or directory doesn't exist
    Io(std::io::Error), // I/O error while reading or writing
    InvalidArchive(String), // 7z archive is malformed or not a valid .tl file
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveError::NotFound(p) => write!(f, "File not found: {}", p.display()),
            ArchiveError::Io(e) => write!(f, "I/O error: {}", e),
            ArchiveError::InvalidArchive(msg) => write!(f, "Invalid archive: {}", msg),
        }
    }
}

impl From<std::io::Error> for ArchiveError {
    fn from(e: std::io::Error) -> Self {
        ArchiveError::Io(e)
    }
}

pub fn decompress(tl_path: &Path, temp_dir: &Path) -> Result<(), ArchiveError> {
    if !tl_path.exists() {
        return  Err(ArchiveError::NotFound(tl_path.to_path_buf()));
    }

    // Create the temp dir if doesn't exist
    fs::create_dir_all(temp_dir)?;

    sevenz_rust::decompress_file(tl_path, temp_dir)
        .map_err(|e| ArchiveError::InvalidArchive(e.to_string()))?;

    Ok(())
}

pub fn compress(temp_dir: &Path, tl_path: &Path) -> Result<(), ArchiveError> {
    if !temp_dir.exists() {
        return Err(ArchiveError::NotFound(temp_dir.to_path_buf()));
    }

    if tl_path.exists() {
        fs::remove_file(tl_path)?;
    }

    if let Some(parent) = tl_path.parent() {
        fs::create_dir_all(parent)?;
    }

    sevenz_rust::compress_to_path(temp_dir, tl_path)
        .map_err(|e| ArchiveError::InvalidArchive(e.to_string()))?;

    Ok(())
}

pub fn create_project_structure(temp_dir: &Path) -> Result<(), ArchiveError> {
    fs::create_dir_all(temp_dir.join("chapters"))?;
    fs::create_dir_all(temp_dir.join("assets").join("images"))?;
    fs::create_dir_all(temp_dir.join("styles"))?;

    Ok(())
}