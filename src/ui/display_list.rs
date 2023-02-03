#![allow(dead_code, unused)]

use std::default;

use super::{fonts, Bitmap};
use super::low_ui::{Pos, Dim};

pub enum Align { 
  Left,
  Center,
  Right,
  Justify
}

pub enum TextTransform {
  None,
  UpperCase,
  LowerCase,
  Capitalize
}

pub enum Display {
  None,
  Inline,
  Block,
  InlineBlock
}

pub struct Format {
  line_height_factor : f32, // In EMs
  font_index         : u16,
  font_size          : u16, // In pixels
  indent             : u16, //     .
  margin_left        : u16, //     .
  margin_right       : u16, //     .
  margin_top         : u16,
  margin_bottom      : u16,
  screen_left        : u16,
  screen_right       : u16,
  screen_top         : u16,
  screen_bottom      : u16, //     .
  width              : u16, //     .
  height             : u16, //     .
  vertical_align     : u16, // In pixels
  trim               : bool,
  pre                : bool,
  font_style         : fonts::FaceStyle,
  align              : Align,
  text_transform     : TextTransform,
  display            : Display,
}

#[derive(Default)]
pub enum ComputeMode {
  #[default]
  Location,
  Move,
  Display
}

enum DisplayListEntry { 
  Glyph          { pos: Pos, glyph: fonts::Glyph, kern: i16, is_space: bool },
  Image          { pos: Pos, bitmap: Bitmap, advance: i16 },
  Highlight      { pos: Pos, dim: Dim },
  ClearHighlight { pos: Pos, dim: Dim },
  ClearRegion    { pos: Pos, dim: Dim },
  SetRegion      { pos: Pos, dim: Dim },
  Rounded        { pos: Pos, dim: Dim },
  ClearRounded   { pos: Pos, dim: Dim }
}

type DisplayList = Vec<DisplayListEntry>;

#[derive(Default)]
struct PageGenerator {
  display_list       : DisplayList,    // The list of artefacts and their position to put on screen
  line_list          : DisplayList,    // Line preparation for paragraphs
  compute_mode       : ComputeMode,
  screen_is_full     : bool,           // True if screen no more space to add characters
  pos                : Pos,            // Current drawing Screen position


  // Screen limits for page content
  min_pos            : Pos,
  max_pos            : Pos,

  // Left / Right paragraph limits
  para_max_x         : i16, 
  para_min_x         : i16,

  // Computed current Line Width
  line_width         : i16,
  
  // Computed maximum glyphs hight on current line
  glyphs_height      : i16,

  line_height_factor : f32,
  
  para_indent        : i16,
  top_margin         : i16,
}

impl PageGenerator {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn start(&mut self, fmt : &Format) {
    self.pos     = Pos(fmt.screen_left, fmt.screen_top);
    self.min_pos = Pos(fmt.screen_left, fmt.margin_top);
    //self.max_pos = Pos()
  }
}

/*
struct DisplayListEntry {
  union Kind {
    struct GryphEntry {            ///< Used for GLYPH
      Font::Glyph * glyph;         ///< Glyph
      int16_t       kern;
      bool          is_space;
    } glyph_entry;
    struct ImageEntry {            ///< Used for IMAGE
      Image::ImageData image;       
      int16_t          advance;    ///< Horizontal advance on the baseline
    } image_entry;
    struct RegionEntry {           ///< Used for HIGHLIGHT, CLEAR_HIGHLIGHT, SET_REGION and CLEAR_REGION
      Dim dim;                     ///< Region dimensions
    } region_entry;
    Kind() {}
  } kind;
  Pos pos;                         ///< Screen coordinates
  DisplayListCommand command;      ///< Command
};
*/