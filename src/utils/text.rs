use std::{
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
};

use bevy_math::Rect;
use tracing::error;

use crate::protocol::BoardMessage;

use super::svg;

fn get_font(font: Option<String>) -> anyhow::Result<Option<PathBuf>> {
    if let Some(font) = font {
        // TODO: Possible read out of fonts directory here.
        if font.contains("..") || font.contains("/") {
            // suspicious file, try with default font
            return get_font(None);
        }

        let mut font_path = PathBuf::new();
        font_path.push("fonts");
        font_path.push(font);

        if !font_path.exists() {
            return get_font(None);
        }

        let extension = font_path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        if extension != "ttf" {
            return get_font(None);
        }

        return Ok(Some(font_path));
    }

    let mut path = PathBuf::new();
    path.push("fonts");

    for file in path.read_dir()? {
        let file = file?;
        let path = file.path();

        let extension = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        if extension == "ttf" {
            return Ok(Some(path));
        }
    }

    Ok(None)
}

pub fn write(
    rect: Rect,
    text: String,
    font: Option<String>,
) -> anyhow::Result<(Vec<BoardMessage>, Rect)> {
    let font_path = get_font(font)?;

    let Some(font_path) = font_path else {
        error!("Could not draw text as no font was specified!");
        return Ok((vec![], Rect::new(0., 0., 0., 0.)));
    };

    let mut args = vec![];

    args.push(text.as_str());
    args.push("-f");
    args.push(font_path.to_str().unwrap_or_default());

    let mut child = Command::new("text2svg")
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdout = child.stdout.take().unwrap();

    let mut data = String::new();

    _ = stdout.read_to_string(&mut data);

    Ok(svg::draw(rect, data))
}
