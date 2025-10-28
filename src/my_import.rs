pub use fltk::{
    app,
    button::{Button, CheckButton},
    enums::{Align, Color, Font, FrameType},
    frame::Frame,
    // group::{Group, Pack, PackType},
    group::{Pack, PackType},
    // input::Input,
    prelude::*,
    window::Window,
    draw,
};
pub  use mlua::Lua;
// pub  use serde::Deserialize;
pub  use std::collections::HashMap;
pub  use std::fs;
pub  use std::rc::Rc;
pub  use std::cell::RefCell;
pub  use fltk::widget::Widget;
pub  use fltk::text::{TextEditor, TextBuffer};

pub fn parse_font(font_str: &Option<String>) -> (Font, i32) {
    if let Some(s) = font_str {
        let parts: Vec<&str> = s.split(',').map(|s| s.trim()).collect();
        let font_name = parts.get(0).map(|s| s.to_lowercase()).unwrap_or_default();
        let font = match font_name.as_str() {
            "helvetica" => Font::Helvetica,
            "courier" => Font::Courier,
            "times" => Font::Times,
            _ => Font::Helvetica,
        };
        let size = parts.get(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(14);
        (font, size)
    } else {
        (Font::Helvetica, 14)
    }
}

pub fn parse_rgb_color(rgb_str: &Option<String>) -> Option<u32> {
    if let Some(rgb) = rgb_str {
        let parts: Vec<&str> = rgb.split('-').collect();
        if parts.len() == 3 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
            ) {
                return Some(((r as u32) << 16) | ((g as u32) << 8) | (b as u32));
            }
        }
    }
    None
}

pub use std::env;
pub use rfd::FileDialog;

pub fn open_file_dialog() -> mlua::Result<Option<String>> {
    let file = FileDialog::new().pick_file();
    Ok(file.map(|path| path.to_string_lossy().into_owned()))
}



pub fn save_file_dialog() -> mlua::Result<Option<String>> {
    let file = FileDialog::new().save_file();
    Ok(file.map(|path| path.to_string_lossy().into_owned()))
}



pub fn visualize_whitespace(s: &str) -> String {
    s.replace("\n", "\\n")
     .replace("\r", "\\r")
     .replace("\t", "\\t")
     .replace(" ", "·") // Optional: replace space with a visible dot
}
