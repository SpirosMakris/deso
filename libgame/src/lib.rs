use gdnative::prelude::*;

pub mod gameworld;
pub mod map;
pub mod player;
pub mod unit_selector;

fn init(handle: InitHandle) {
    handle.add_class::<gameworld::GameWorld>();
    handle.add_class::<unit_selector::UnitSelector>();
}

godot_init!(init);
