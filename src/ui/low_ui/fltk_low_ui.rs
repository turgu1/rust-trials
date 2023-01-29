#![allow(dead_code)]

use fltk::{
  prelude::*,
  app::{App as  Application, Scheme, channel, Receiver},
  // draw::{draw_line, draw_point, draw_rect_fill, set_draw_color, set_line_style, LineStyle},
  // enums::{Color, Event as FltkEvent, FrameType},
  frame::Frame,
  // surface::ImageSurface,
  enums::Event as FltkEvent,
  window::Window,
  // button::Button,
  draw,
};

use super::*;
use super::canvas::*;
use crate::utils::observer::Publisher;

use std::cell::RefCell;

const GAP:  i32 = 10;

pub struct FLTKLowUi {
  app: Application,
  receiver: Receiver<Event>,
  canvas: RefCell<Canvas>,
  publisher: RefCell<Publisher<Event>>,
  frame: RefCell<Frame>,
}

impl FLTKLowUi {
  fn build_ui(canvas: &RefCell<Canvas>) -> (Receiver<Event>, RefCell<Frame>) {
    let (sender, receiver) = channel::<Event>();
  
    let mut window =  Window::default()
      .with_size(WIDTH + GAP*2, HEIGHT + GAP*2)
      .with_label("FLTK Trial");
  
    let frame = RefCell::new(Frame::default()
      .with_size(WIDTH, HEIGHT)
      .with_pos(GAP, GAP));

    // let mut boo_button = Button::default()
    //   .with_pos(GAP, GAP)
    //   .with_size(75, 25)
    //   .with_label("Boo!!");
    // boo_button.emit(sender, Event::Test);

    window.end();
    window.show();

    unsafe { draw::draw_rgba_nocopy(&mut *frame.borrow_mut(), &canvas.borrow().frame_buffer); }

    frame.borrow_mut().handle({
      move |frame, event| { 
        // sender.send(Event::Test);
        if event == FltkEvent::Push {
          let coords = fltk::app::event_coords(); 
          sender.send(Event::Click((coords.0 - frame.x()) as u16, (coords.1 - frame.y()) as u16)); 
        }
        true 
      }
    });

    (receiver, frame)
  }

}


impl LowUi for FLTKLowUi  {

  fn new() -> FLTKLowUi {
    let app = Application::default().with_scheme(Scheme::Gtk);
    let canvas = RefCell::new(Canvas::new());
    let (receiver, frame) = FLTKLowUi::build_ui(&canvas);

    Self {
      app,
      publisher: RefCell::new(Publisher::<Event>::new()), 
      receiver,
      canvas,
      frame,
    }
  }

  fn run(&self) {
    while self.app.wait() {
      if let Some(event) = self.receiver.recv() {
        self.publisher.borrow().notify(event);
      }
    }
  }

  fn subscribe(&self, listener: impl Fn(Event) + 'static, event_type: Event) {
    self.publisher.borrow_mut().subscribe(event_type, listener);
  }

  fn draw_rectangle(&self, pos: Pos, dim: Dim, color: Color) {
    self.canvas.borrow_mut().draw_rectangle(pos, dim, color);
  }

  fn draw_pixel(&self, pos: Pos, color: Color) {
    self.canvas.borrow_mut().put(pos, color);
  }

  fn redraw(&self) {
    self.frame.borrow_mut().redraw();
  }
}
