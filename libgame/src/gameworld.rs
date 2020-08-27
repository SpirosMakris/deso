use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GameWorld {

}

#[methods]
impl GameWorld {
    pub fn new(_owner: &Node2D) -> Self {
        Self {}
    }

    #[export]
    pub fn _ready(&self, _owner: &Node2D) {
        // @RMV
    }

    #[export]
    pub fn _unhandled_input(&self, _owner: &Node2D, _event: Variant) {
        // @RMV
    }

    #[export]
    pub fn _process(&self, _owner: &Node2D, _delta: f64) {
        // @RMV
    }

    #[export]
    pub fn _physics_process(&self, _owner: &Node2D, __delta: f64) {
        // @RMV
    }
}