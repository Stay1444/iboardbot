use std::path::PathBuf;

use anyhow::anyhow;
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};

const FONTS_DIR: &str = "fonts";

pub fn load(preference: Option<String>) -> anyhow::Result<PathBuf> {
    // Validate that user preference is a correct font and that it exists.
    if let Some(path) = validate_preference(preference) {
        return Ok(path);
    }

    let font_dir: PathBuf = FONTS_DIR.into();

    // Load any font file present in the fonts folder
    for file in font_dir.read_dir()? {
        let file = file?;
        if !file.file_type()?.is_file() {
            continue;
        }

        let file = file.path();
        let Some(extension) = file.extension() else {
            continue;
        };

        if extension != "ttf" {
            continue;
        }

        return Ok(file);
    }

    // Try to load a system font
    let handle = SystemSource::new().select_best_match(&[FamilyName::Serif], &Properties::new())?;

    if let Handle::Path {
        path,
        font_index: _,
    } = handle
    {
        return Ok(path);
    }

    Err(anyhow!("No valid font found"))
}

fn validate_preference(preference: Option<String>) -> Option<PathBuf> {
    let Some(preference) = preference else {
        return None;
    };

    let Ok(path): Result<PathBuf, _> = preference.parse() else {
        return None;
    };

    let extension = path.extension()?.to_str()?;

    if extension != "ttf" {
        return None;
    }

    let file_name = path.file_name()?.to_str()?.to_owned();

    let mut path: PathBuf = FONTS_DIR.into();
    path.push(file_name);

    if path.exists() {
        return Some(path);
    }

    None
}
