use std::{
    io::Read,
    process::{Command, Stdio},
};

use bevy_math::Rect;

use crate::protocol::BoardMessage;

use super::svg;

pub fn write(rect: Rect, text: String) -> (Vec<BoardMessage>, Rect) {
    let mut args = vec![];

    args.push(text.as_str());
    args.push("-f");
    args.push("./fonts/Roboto.ttf");

    let mut child = Command::new("text2svg")
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdout = child.stdout.take().unwrap();

    let mut data = String::new();

    _ = stdout.read_to_string(&mut data);

    svg::draw(rect, data)
}
