mod ui;
mod utils;

use std::rc::Rc;
use std::cell::RefCell;

use ui::low_ui::{*, fltk_low_ui::FLTKLowUi};

fn test_input_handler(event: Event) {
  println!("test_input_handler called back with event {event:?}");
}

use fontdue::Metrics;

fn show_char(ui: &Rc<RefCell<FLTKLowUi>>, xpos: u16, metrics: Metrics, bitmap: Vec<u8>) -> u16 {
  let mut idx = 0;
  for j in 0..metrics.height {
    for i in 0..metrics.width {
      ui.borrow().draw_pixel(
        Pos(
          i as u16 + (xpos as i32 + metrics.xmin) as u16, 
          (50 + j as i32 - (metrics.height as i32 + metrics.ymin)) as u16
        ), 
        255 - bitmap[idx]);
      idx = idx + 1;
      // print!("{} ", bitmap[j * metrics.width + i]);
    }
    // println!("");
  }
  (xpos as f32 + metrics.advance_width) as u16
}

fn font_test(ui: &Rc<RefCell<FLTKLowUi>>) {
  // Read the font data.
  let font = include_bytes!("../fonts/AbyssinicaSIL-Regular.ttf") as &[u8];
  // Parse it into the font type.
  let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
  let mut xpos = 0;
  let mut last_ch = ' ';
  for c in "VAijgficxVM".chars() {
    let (metrics, bitmap) = font.rasterize(c, 24.0);
    //println!("Metrics: {:?}", metrics);
    xpos = show_char(ui, xpos, metrics, bitmap);
    if last_ch != ' ' {
      if let Some(kern) = font.horizontal_kern(last_ch, c, 24.0) {
        println!("Kerning between {} and {}: {}", last_ch, c, kern);
      }
    }
    last_ch = c;
  }
  for c in "VaijgficxVM".chars() {
    let (metrics, bitmap) = font.rasterize(c, 40.0);
    //println!("Metrics: {:?}", metrics);
    xpos = show_char(ui, xpos, metrics, bitmap);
  }
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

  font_test(&ui);

  ui.borrow().subscribe(f, Event::Click(0, 0));
  
  ui.borrow().subscribe(test_input_handler, Event::Test);
  ui.borrow().subscribe(move |event| { println!("Nothing happened! {event:?} {s}"); }, Event::Nothing);
  ui.borrow().run();
}