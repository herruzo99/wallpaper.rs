use crate::error::{Error, Result};
use std::fs::File;

pub fn download_image(url: &str) -> Result<String> {
    // Use our specific error variant instead of a string literal
    let cache_dir = dirs::cache_dir().ok_or(Error::NoCacheDir)?;
    let mut file_path = cache_dir;
    file_path.push("wallpaper");
    let mut file = File::create(&file_path)?;

    // This now works because our Error enum can be created `from` a reqwest::Error
    reqwest::blocking::get(url)?.copy_to(&mut file)?;

    // Return the path to the downloaded file, using our specific error variant
    Ok(file_path
        .to_str()
        .ok_or(Error::InvalidFilePath)?
        .to_string())
}