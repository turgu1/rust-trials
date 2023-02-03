#![allow(dead_code, unused)]

use std::fs;

use fltk::app::App;
use fltk::examples::system_fonts;
use fontdue::Metrics;
use crate::utils::errors::{UserError, InternalError};
use crate::ui::low_ui::Bitmap;

#[derive(Debug, PartialEq)]
pub enum FaceStyle { 
  Normal = 0, 
  Bold, 
  Italic, 
  BoldItalic 
}

pub struct Glyph {
  pub metrics: Metrics,
  pub raster: Option<Vec<u8>>,
}

pub struct FontEntry {
  pub name: String,
  pub face_style: FaceStyle,
  pub font: fontdue::Font,
}

impl FontEntry {

  fn new_from_memory(name: String, font_data: &[u8], face_style: FaceStyle) -> Result<Self, InternalError> {
    if let Ok(font) = fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default()) {
      Ok(Self {
        name,
        face_style,
        font,
      })
    }
    else {
      Err(InternalError::StringError(format!("fontdue: font format not compatible: {name}")))
    }
  }

  fn new_from_file(name: String, filename: &String, face_style: FaceStyle) -> Result<Self, InternalError> {
    let font_data = fs::read(filename)?;
    FontEntry::new_from_memory(name, &font_data, face_style)
  }

  pub fn get_glyph(&self, character: char, px: f32, with_raster: bool) -> Glyph {
    if with_raster {
      let (metrics, raster) = self.font.rasterize(character, px);
      Glyph {
        metrics,
        raster: Some(raster),
      }
    }
    else {
      let metrics = self.font.metrics(character, px);
      Glyph {
        metrics,
        raster: None,
      }
    }
  }

  pub fn get_glyph_metrics(&self, character: char, px: f32) -> Metrics {
    self.font.metrics(character, px)
  }
}

pub struct AppFont {
  name: String,
  filename: String,
  face_style: FaceStyle,
}

pub struct Fonts {
  font_entries: Vec<FontEntry>,
  user_fonts: Vec<AppFont>,
  system_fonts: Vec<AppFont>,
}

macro_rules! unwrap_or_continue {
  ($e: expr, $e1: expr, $e2: expr) => (
      match $e {
          Some(e) => e,
          None => {
            println!("Warning reading fonts_list.xml: Attribute {} not found for {}.", $e2, $e1);
            continue;
          },
      }
  )
}

impl Fonts {
  pub fn new() -> Self {
    Self {
      font_entries : Vec::new(),
      user_fonts : Vec::new(),
      system_fonts : Vec::new(),
    }
  }

  pub fn setup(&mut self) -> Result<(), InternalError> {
    let text = std::fs::read_to_string("fonts/fonts_list.xml")?;

    if let Ok(doc) = roxmltree::Document::parse(&text) {
      let root = doc.root_element();
  
      if root.tag_name().name().eq("fonts") {
        for group in root.children().filter(|n| n.is_element() && n.tag_name().name().eq("group")) {
          let group_name = unwrap_or_continue!(group.attribute("name"), "group", "name");
          for font in group.children().filter(|n| n.is_element() && n.tag_name().name().eq("font")) {
            let name = unwrap_or_continue!(font.attribute("name"), "font", "name");
            for sfont in font.children().filter(|n| n.is_element()) {
              let tag_name = sfont.tag_name().name();
              let filename = unwrap_or_continue!(sfont.attribute("filename"), tag_name, "filename");
              let face_style = match sfont.tag_name().name() {
                "normal" => FaceStyle::Normal,
                "bold" => FaceStyle::Bold,
                "italic" => FaceStyle::Italic,
                "bold-italic" => FaceStyle::BoldItalic,
                _ => { 
                  println!("Warning reading fonts_list.xml: Unknown font face style: {}.", sfont.tag_name().name()); 
                  continue; 
                }
              };
              let afont = AppFont {
                name: name.to_string(), 
                filename: filename.to_string(), 
                face_style
              };
              match group_name {
                "SYSTEM" => self.system_fonts.push(afont),
                "USER" => self.user_fonts.push(afont),
                _ => {
                  println!("Warning reading fonts_list.xml: Unknown font group name: {group_name}");
                  continue;
                }
              } // match group_name
            } // for sfont...
          } // for font...
        } // for group...
      } // if root...

      self.retrieve_defined_system_font("TEXT", FaceStyle::Normal)?;
      self.retrieve_defined_system_font("TEXT", FaceStyle::Italic)?;
      self.retrieve_defined_system_font("ICON", FaceStyle::Normal)?;

      Ok(())
    }
    else {
      Err(InternalError::StrError("Unable to parse XML file fonts_list.xml."))
    }
  }

  pub fn get(&self, index: usize) -> Result<&FontEntry, InternalError> {
    if let Some(f) = self.font_entries.get(index) { Ok(f) }
    else if let Some(f) = self.font_entries.get(1) { Ok(f) }
    else {
      Err(InternalError::StringError(format!("Fonts::get(): Wrong index: {}", index)))
    }
  }

  pub fn add_from_file(&mut self, name: String, filename: &String, face_style: FaceStyle) -> Result<&FontEntry, InternalError> {
    self.font_entries.push(FontEntry::new_from_file(name, filename, face_style)?);
    Ok(self.font_entries.last().unwrap())
  }

  pub fn add_from_memory(&mut self, name: String, font_data: &[u8], face_style: FaceStyle) -> Result<&FontEntry, InternalError> {
    self.font_entries.push(FontEntry::new_from_memory(name, font_data, face_style)?);
    Ok(self.font_entries.last().unwrap())
  }

  fn retrieve_defined_system_font(&mut self, name: &str, face_style: FaceStyle) -> Result<(), InternalError> {
    let mut app_font: Option<&AppFont> = None;
    for afont in self.system_fonts.iter() {
      if (afont.name == name) && (afont.face_style == face_style) {
        app_font = Some(afont);
        break;
      }
    }
    if let Some(font_info) = app_font {
      let mut filename = "fonts/".to_owned();
      filename.push_str(&font_info.filename);
      self.add_from_file(name.to_string(), &filename, face_style)?;
      Ok(())
    }
    else {
      Err(InternalError::StringError(format!("Unable to find font {name} with FaceStyle {face_style:?}")))
    }
  }

  pub fn get_font_at_index(&self, index: usize) -> &FontEntry {
    &self.font_entries[index]
  }
  
}