use std::{fmt::write, path::PathBuf, process::Command, time::SystemTime};

use bevy_math::Vec2;
use tracing::info;
use uuid::{Timestamp, Uuid};

use crate::{api::services::boards::entities::BoardDimensions, protocol::{BoardAction, BoardMessage}};

use super::{coords::CoordinateProjector, SBA};

pub fn draw(
    dimensions: BoardDimensions,
    svg: String,
    scale: f32,
    erase: bool,
) -> Vec<BoardMessage> {
    let gcode = generate_gcode(svg);

    let mut messages = vec![];
    let mut actions = vec![];
    let mut is_pen_down = false;

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

                actions.push(SBA::Move(
                    (x.unwrap_or_default() * scale).min(3999.0),
                    (y.unwrap_or_default() * scale).min(3999.0),
                ));
            }

            if tokens.contains(&"M4") {
                actions.push(SBA::PenUp);
                is_pen_down = false;
            }
            if tokens.contains(&"M5") {
                actions.push(SBA::PenDown);
                is_pen_down = true;
            }
        }

        if actions.len() > 220 {
            let mut msg = BoardMessage::new(messages.len() as u8 + 1);

            if is_pen_down {
                msg.push(BoardAction::PenDown);
            }

            for action in &actions {
                msg.push(action.clone().into());
            }

            messages.push(msg);

            actions.clear();
        }
    }

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
