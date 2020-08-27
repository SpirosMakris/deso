use gd_extras::gdp;
use gd_extras::input::InputEventExt;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GameWorld {}

#[methods]
impl GameWorld {
    pub fn new(_owner: &Node2D) -> Self {
        Self {}
    }

    #[export]
    pub fn _ready(&self, _owner: &Node2D) {
        gdp!("GameWorld _ready()");
    }

    #[export]
    pub fn _unhandled_input(&self, owner: &Node2D, event: Variant) {
        let event = event
            .try_to_object::<InputEvent>()
            .expect("I expect this to be an input event");

        let event = unsafe { event.assume_safe() };

        if event.action_pressed("ui_cancel") {
            gdp!("QUIT SIGNAL");
            owner
                .get_tree()
                .map(|tree| unsafe { tree.assume_safe() }.quit(0));
        }
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
