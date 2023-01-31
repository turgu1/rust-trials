#![allow(dead_code)]
#![allow(unused)]

use std::fs;

use fontdue::Metrics;
use crate::utils::errors::{UserError, InternalError};
use crate::ui::low_ui::Bitmap;

pub enum FaceStyle { 
  Normal = 0, 
  Bold, 
  Italic, 
  BoldItalic 
}

pub struct Glyph {
  pub metrics: Metrics,
  pub data: Vec<u8>,
}

pub struct FontEntry {
  pub name: String,
  pub face_style: FaceStyle,
  pub font: fontdue::Font,
}

impl FontEntry {

  pub fn new_from_memory(name: String, font_data: &[u8], face_style: FaceStyle) -> Result<Self, InternalError> {
    if let Ok(font) = fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default()) {
      Ok(Self {
        name,
        face_style,
        font,
      })
    }
    else {
      Err(InternalError::SomeError("fontdue: font format not compatible", name.to_string()))
    }
  }

  pub fn new_from_file(name: String, filename: &String, face_style: FaceStyle) -> Result<Self, InternalError> {
    let font_data = fs::read(filename)?;
    FontEntry::new_from_memory(name, &font_data, face_style)
  }

  pub fn get_glyph(&self, character: char, px: f32) -> Glyph {
    let (metrics, data) = self.font.rasterize(character, 24.0);
    Glyph {
      metrics,
      data,
    }
  }

  pub fn get_glyph_metrics(&self, character: char, px: f32) -> Metrics {
    self.font.metrics(character, px)
  }
}