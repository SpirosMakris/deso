// use gdnative::GodotObject;
use gdnative::prelude::*;
use gdnative::api::{AnimatedSprite};
pub trait NodeExt: GodotObject + std::fmt::Debug {
  fn get_and_cast<T: GodotObject + SubClass<Node>>(&self, path: &str) -> TRef<'_, T, Shared>;
}

macro_rules! node_ext {
  ($type: ident) => {
    impl NodeExt for $type {
      fn get_and_cast<T: GodotObject + SubClass<Node>>(&self, path: &str) -> TRef<'_, T, Shared> {
        let node = self.get_node(path.to_string())
          .expect("Node not found");

        unsafe {
          let n = node.assume_safe();
          n.cast::<T>()
            .expect("Invalid note type")
        }
      }
    }
  };

}

node_ext!(Node);
// node_ext!(AnimatedSprite);
// node_ext!(KinematicBody2D);



