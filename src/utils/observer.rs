#![allow(dead_code)]

use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

/// An event type.
// #[derive(PartialEq, Eq, Hash, Copy)]
// pub enum Event {
//   Load,
//   Save,
// }

type Events<E> = HashMap<E, Vec<Box<dyn Fn(E)>>>;

/// Publisher sends events to subscribers (listeners).
#[derive(Default)]
pub struct Publisher<E: Eq + Hash + Copy> {
  events: Events<E>,
}

impl<E: Eq + Hash + Copy> Publisher<E> {
  pub fn new() -> Self {
    Self {
      events: HashMap::new()
    }
  }

  pub fn subscribe(&mut self, event_type: E, listener: impl Fn(E) + 'static) {
    self.events
      .entry(event_type)
      .or_default();
    self.events
      .get_mut(&event_type)
      .unwrap()
      .push(Box::new(listener));
  }

  // pub fn unsubscribe(&mut self, event_type: E, listener: impl Fn(E) + 'static) {
  //     self.events
  //         .get_mut(&event_type)
  //         .unwrap()
  //         .retain(|&x| x != listener);
  // }

  pub fn notify(&self, event: E) {
    if let Some(listeners) = self.events.get(&event) {
      for listener in listeners {
        listener(event);
      }
    }
  }
}
