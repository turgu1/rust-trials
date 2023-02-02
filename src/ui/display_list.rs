use super::fonts;
use super::low_ui::{Pos, Dim};

enum Align { 
  Left,
  Center,
  Right,
  Justify
}

enum TextTransform {
  None,
  UpperCase,
  LowerCase,
  Capitalize
}

enum Display {
  None,
  Inline,
  Block,
  InlineBlock
}

struct Format {
  line_height_factor : f32, ///< In EMs
  font_index : u16,
  font_size : u16,          ///< In pixels
  indent : u16,             ///< In pixels
  margin_left : u16,        ///< In pixels
  margin_right : u16,       ///< In pixels
  margin_top : u16,         ///< In pixels
  margin_bottom : u16,      ///< In pixels
  screen_left : u16,        ///< In pixels
  screen_right : u16,       ///< In pixels
  screen_top : u16,         ///< In pixels
  screen_bottom : u16,      ///< In pixels
  width : u16,              ///< In pixels
  height : u16,             ///< In pixels
  vertical_align : u16,     ///< In pixels
  trim : bool,
  pre : bool,
  font_style : fonts::FaceStyle,
  align : Align,
  text_transform : TextTransform,
  display : Display,
}

enum ComputeMode {
  Location,
  Move,
  Display
}

enum DisplayListEntry { 
  Glyph          { pos: Pos, },
  Image          { pos: Pos, },
  Highlight      { pos: Pos, dim: Dim },
  ClearHighlight { pos: Pos, dim: Dim },
  ClearRegion    { pos: Pos, dim: Dim },
  SetRegion      { pos: Pos, dim: Dim },
  Rounded        { pos: Pos, dim: Dim },
  ClearRounded   { pos: Pos, dim: Dim }
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