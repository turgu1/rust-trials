#![allow(dead_code)]

pub mod fltk_low_ui;
mod canvas;

use derivative::Derivative;
use super::fonts::Glyph;

#[derive(Debug, Eq, Copy, Clone, Derivative)]
#[derivative(Hash, PartialEq)]
pub enum Event {
  Click(
    #[derivative(Hash="ignore")]
    #[derivative(PartialEq="ignore")]
    u16,
    #[derivative(Hash="ignore")]
    #[derivative(PartialEq="ignore")]
    u16),
  Test,
  Nothing,
}

#[derive(Default)]
pub struct Pos(pub u16, pub u16);
#[derive(Default)]
pub struct Dim(pub u16, pub u16);

pub type Color = u8;

pub const BLACK : u8 = 0x00;
pub const WHITE : u8 = 0xFF;

pub struct Bitmap {
  data: Vec<u8>,
  dim: Dim,
  stride: u16,
}

pub trait LowUi {
  fn new() -> Self;
  fn run(&self);
  fn subscribe(&self, listener: impl Fn(Event) + 'static, event_type: Event);
  fn draw_rectangle(&self, pos: Pos, dim: Dim, color: Color);
  fn draw_pixel(&self, pos: Pos, color: Color);
  fn draw_glyph(&self, glyph: &Glyph, pos: Pos) -> u16;
  fn redraw(&self);
}