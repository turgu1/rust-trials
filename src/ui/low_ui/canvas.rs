use super::*;

use crate::ui::fonts::Glyph;

const WIDTH: u16 = 600;
const HEIGHT: u16 = 800;

pub struct Canvas {
  pub frame_buffer: Vec<u8>
}

impl Canvas {
  pub fn new() -> Self {
    Self {
      frame_buffer: vec![255; WIDTH as usize * HEIGHT as usize * 4]
    }
  }

  #[inline]
  pub fn get_width(&self) -> u16 { WIDTH }

  #[inline]
  pub fn get_height(&self) -> u16 { HEIGHT }

  pub fn put(&mut self, pos: Pos, color: Color) {
    if color != WHITE {
      let Pos(x, y) = pos;
      if (x < WIDTH as u16) && (y < HEIGHT as u16) {
        let idx = ((y as usize * WIDTH as usize) + x as usize) * 4;
        self.frame_buffer[idx    ] = color;
        self.frame_buffer[idx + 1] = color;
        self.frame_buffer[idx + 2] = color;
        self.frame_buffer[idx + 3] = 0xFF;
      }
    }
  }

  pub fn draw_rectangle(&mut self, pos: Pos, dim: Dim, color: Color) {
    let Pos(x, y) = pos;
    let Dim(w, h) = dim;
    for i in  0..w { self.put(Pos(x + i, y), color); }

    for i in 1..h {
      self.put(Pos(x, y + i), color);
      self.put(Pos(x + w - 1, y + i), color);
    }
    for i in  0..w { self.put(Pos(x + i, y + h - 1), color); }
  }

  pub fn draw_glyph(&mut self, glyph: &Glyph, pos: Pos) -> u16 {
    let mut idx = 0;
    let Pos(xpos, ypos) = pos;
    for col in 0..glyph.metrics.height {
      for line in 0..glyph.metrics.width {
        self.put(
          Pos(
            line as u16 + (xpos as i32 + glyph.metrics.xmin) as u16, 
            (ypos as i32 + col as i32 - (glyph.metrics.height as i32 + glyph.metrics.ymin)) as u16
          ), 
          255 - glyph.data[idx]);
        idx += 1;
      }
    }
    (xpos as f32 + glyph.metrics.advance_width) as u16
  }
  
}
