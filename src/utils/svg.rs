use std::{path::PathBuf, process::Command};

use bevy_math::{Rect, Vec2};
use tracing::info;
use uuid::Uuid;

use crate::{
    protocol::BoardMessage,
    utils::{qtree::QuadTree, SBM},
};

use super::SBA;

pub fn draw_group(bounds: Rect, svgs: Vec<String>) -> Vec<BoardMessage> {
    let mut qtree = QuadTree::<String>::new(bounds, 4);
    let mut messages = vec![];

    qtree.feed(svgs);

    qtree.iter(&mut |rect, svg| {
        messages.append(&mut draw(rect, svg));
    });

    messages
}

pub fn draw(rect: Rect, svg: String) -> Vec<BoardMessage> {
    let gcode = generate_gcode(svg);

    let mut message = SBM::new(1);

    for line in gcode.lines() {
        if line.starts_with(";") {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if let Some(instruction) = tokens.first() {
            if instruction == &"G0" || instruction == &"G1" {
                let mut x: Option<f32> = None;
                let mut y: Option<f32> = None;

                for token in tokens.iter().skip(1) {
                    if token.starts_with('X') {
                        x = token[1..].parse().ok();
                    } else if token.starts_with('Y') {
                        y = token[1..].parse().ok();
                    }
                }

                message.push(SBA::Move(
                    x.unwrap_or_default() * 0.1,
                    y.unwrap_or_default() * 0.1,
                ));
            }

            if tokens.contains(&"M4") {
                message.push(SBA::PenUp);
            }
            if tokens.contains(&"M5") {
                message.push(SBA::PenDown);
            }
        }
    }

    let size = rect.size();

    while message.bounds().cmplt(size).all() {
        message.scale(1.1);
        info!("Scaled up to bounds: {}", message.bounds());
    }

    while message.bounds().cmpgt(size).any() {
        message.scale(0.98);
        info!("Scaled down to bounds: {}", message.bounds());

        let bounds = message.bounds();
        if bounds.cmpgt(size * 10.0).all() {
            message.scale(0.1);
        }
    }

    for action in &mut message.actions {
        if let SBA::Move(x, y) = action {
            let inverted_y = rect.max.y - *y;
            *y = inverted_y;

            *x += rect.min.x;
            *y += rect.min.y;
        }
    }

    let messages = message.build();

    tracing::info!("SVG produced {} messages", messages.len());

    messages
}

fn generate_gcode(svg: String) -> String {
    let temp_dir: PathBuf = "temp-conversions".into();

    let id = Uuid::new_v4();

    if !temp_dir.exists() {
        std::fs::create_dir(temp_dir.clone()).unwrap();
    }
    let mut svg_file = temp_dir.clone();
    svg_file.push(format!("{}.svg", id.to_string()));

    std::fs::write(&svg_file, svg).unwrap();

    let mut out_file = temp_dir.clone();
    out_file.push(format!("{}.gcode", id.to_string()));

    let mut args = vec![];

    args.push(svg_file.to_str().unwrap());
    args.push("--off");
    args.push("M4");
    args.push("--on");
    args.push("M5");
    args.push("-o");
    args.push(out_file.to_str().unwrap());

    Command::new("svg2gcode").args(&args).status().unwrap();

    let gcode = std::fs::read_to_string(out_file).unwrap();

    gcode
}
