pub mod low_ui;

use std::rc::Rc;
use std::cell::RefCell;

// use low_ui::*;
// use crate::fltk_low_ui::FLTKLowUi as HalUI;

pub use low_ui::*;
pub use crate::fltk_low_ui::FLTKLowUi as HalUI;

pub fn new() -> Rc<RefCell<HalUI>> {
  Rc::new(RefCell::new(LowUi::new()))
}