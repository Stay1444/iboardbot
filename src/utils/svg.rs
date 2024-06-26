use std::{os::unix::process::ExitStatusExt, path::PathBuf, process::Command};

use anyhow::anyhow;
use bevy_math::{Rect, Vec2};
use tracing::{debug, error};
use uuid::Uuid;

use crate::{
    protocol::{BoardAction, BoardMessage},
    utils::software_board::{SBA, SBM},
};

pub fn draw_group(bounds: Rect, svgs: Vec<String>) -> anyhow::Result<(Vec<BoardMessage>, Rect)> {
    let mut messages = vec![];
    let mut boxes = vec![];

    let width = bounds.width() / svgs.len() as f32;

    for i in 0..svgs.len() {
        let x0 = (i as f32 * width) + bounds.min.x;
        boxes.push(Rect::new(x0, bounds.min.y, x0 + width, bounds.max.y));
    }

    if boxes.len() != svgs.len() {
        error!(
            "Disparity between boxes and svgs! {} {}",
            boxes.len(),
            svgs.len()
        );

        return Err(anyhow!("Disparity between boxes and svgs"));
    }

    for i in 0..boxes.len() {
        let svg = svgs[i].clone();
        let rect = boxes[i];
        let (mut res, _) = draw(rect, svg)?;

        messages.append(&mut res);
    }

    Ok((messages, bounds))
}

pub fn draw(rect: Rect, svg: String) -> anyhow::Result<(Vec<BoardMessage>, Rect)> {
    let gcode = generate_gcode(svg)?;

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
        debug!("Scaled up to bounds: {}", message.bounds());
    }

    while message.bounds().cmpgt(size).any() {
        message.scale(0.98);
        debug!("Scaled down to bounds: {}", message.bounds());

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

    let message_bounds = message.bounds();
    let mut messages = message.build();

    tracing::debug!("SVG produced {} messages", messages.len());

    if let Some(last) = messages.last_mut() {
        last.push(BoardAction::StopDrawing);
    }

    Ok((messages, Rect::from_corners(Vec2::ZERO, message_bounds)))
}

fn generate_gcode(svg: String) -> anyhow::Result<String> {
    let temp_dir: PathBuf = "temp-conversions".into();

    let id = Uuid::new_v4();

    if !temp_dir.exists() {
        std::fs::create_dir(temp_dir.clone()).unwrap();
    }

    let mut svg_file = temp_dir.clone();
    svg_file.push(format!("{}.svg", id.to_string()));

    std::fs::write(&svg_file, svg)?;

    let mut out_file = temp_dir.clone();
    out_file.push(format!("{}.gcode", id.to_string()));

    let mut args = vec![];

    let Some(svg_file_str) = svg_file.to_str() else {
        return Err(anyhow!("Failed to convert path to string"));
    };

    let Some(out_file_str) = out_file.to_str() else {
        return Err(anyhow!("Failed to convert path to string"));
    };

    args.push(svg_file_str);
    args.push("--off");
    args.push("M4");
    args.push("--on");
    args.push("M5");
    args.push("-o");
    args.push(out_file_str);

    let result = match Command::new("svg2gcode").args(&args).status() {
        Ok(x) => x,
        Err(err) => {
            error!("Failed to invoke svg2gcode, is it installed? {err}");
            return Err(err.into());
        }
    };

    if !result.success() {
        return Err(anyhow!(
            "svg2gcode failed with status code {}",
            result.stopped_signal().unwrap_or_default()
        ));
    }

    let gcode = std::fs::read_to_string(out_file)?;

    Ok(gcode)
}
