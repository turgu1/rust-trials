mod ui;
mod utils;

use std::rc::Rc;
use std::cell::RefCell;

use ui::low_ui::{*, fltk_low_ui::FLTKLowUi};
use ui::fonts::{FontEntry, FaceStyle};

fn test_input_handler(event: Event) {
  println!("test_input_handler called back with event {event:?}");
}

use utils::errors::InternalError;

fn font_test(ui: &Rc<RefCell<FLTKLowUi>>) -> Result<(), InternalError> {
  // Read the font data.
  let font_entry = FontEntry::new_from_file(
    "AbyssinicaSIL".to_string(), 
    &"fonts/AbyssinicaSIL-Regular.ttf".to_string(),
    FaceStyle::Normal)?;

  let mut xpos = 0;
  for c in "VAijgficxVM".chars() {
    let glyph = font_entry.get_glyph(c, 24.0);
    xpos = ui.borrow().draw_glyph(&glyph, Pos(xpos, 50));
  }
  for c in "VAijgficxVM".chars() {
    let glyph = font_entry.get_glyph(c, 40.0);
    xpos = ui.borrow().draw_glyph(&glyph, Pos(xpos, 50));
  }
  Ok(())
}

fn main() {
  let ui = ui::new();
  let s = "Allo".to_string();
 
  let the_ui = ui.clone();
  let f = move |event| { 
    if let Event::Click(x, y) = event { 
      the_ui.borrow().draw_rectangle(Pos(x, y), Dim(50, 50), BLACK);
      the_ui.borrow().redraw(); 
    }
  };

  font_test(&ui).unwrap();

  ui.borrow().subscribe(f, Event::Click(0, 0));
  
  ui.borrow().subscribe(test_input_handler, Event::Test);
  ui.borrow().subscribe(move |event| { println!("Nothing happened! {event:?} {s}"); }, Event::Nothing);
  ui.borrow().run();
}