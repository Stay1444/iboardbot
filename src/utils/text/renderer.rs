use std::{
    io::Read,
    process::{Command, Stdio},
};

use bevy_math::Rect;
use tracing::error;

use crate::{protocol::BoardMessage, utils::svg};

use super::font_loader;

pub fn render(
    rect: Rect,
    text: String,
    font_preference: Option<String>,
) -> anyhow::Result<(Vec<BoardMessage>, Rect)> {
    let font_path = font_loader::load(font_preference)?;

    let mut args = vec![];

    args.push(text.as_str());
    args.push("-f");
    args.push(font_path.to_str().unwrap_or_default());

    let mut child = match Command::new("text2svg")
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(x) => x,
        Err(err) => {
            error!("Error spawning text2svg, is it installed? {err}");
            return Err(err.into());
        }
    };

    let mut stdout = child.stdout.take().expect("Child standard output");

    let mut data = String::new();

    stdout.read_to_string(&mut data)?;

    Ok(svg::draw(rect, data))
}
