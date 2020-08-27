use gdnative::api::InputEvent;
use gdnative::prelude::*;

// `InputEvent` extension
pub trait InputEventExt {
  // fn strength(&self, key: &str) -> f32;

  fn action_pressed(&self, key: &str) -> bool;

}

impl InputEventExt for InputEvent {
  fn action_pressed(&self, key: &str) -> bool {
      // args: (action name, allow echo)
      self.is_action_pressed(key.to_string(), false)
  }
}