use gdnative::prelude::*;

mod gameworld;
mod player;

fn init(handle: InitHandle) {
    handle.add_class::<gameworld::GameWorld>();
    handle.add_class::<player::Player>();
}

godot_init!(init);
