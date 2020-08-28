use gdnative::prelude::*;
use gdnative::api::{Node2D};

use gd_extras::gdp;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct UnitSelector {}

#[methods]
impl UnitSelector {
  pub fn new(_owner: &Node2D) -> Self {
    Self {}
  }

  #[export]
  pub fn _unhandled_input(&self, _owner: &Node2D, event: Variant) {
    gdp!("UnitSelector -> _unhandled_input()");
  }
}