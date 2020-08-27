use gdnative::prelude::*;
use gd_extras::node_ext::NodeExt;


#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {}

#[methods]
impl Player {
  pub fn new(_owner: &KinematicBody2D) -> Self {
    Self {}
  }
}