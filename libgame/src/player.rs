use gdnative::api::{AnimatedSprite, ShaderMaterial, Texture};
use gdnative::prelude::*;

use gd_extras::gdp;
use gd_extras::input::InputEventExt;
use gd_extras::node_ext::NodeExt;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {}

#[methods]
impl Player {
    pub fn new(_owner: &KinematicBody2D) -> Self {
        Self {}
    }

    #[export]
    pub fn _unhandled_input(&self, owner: &KinematicBody2D, event: Variant) {
        let event = event
            .try_to_object::<InputEvent>()
            .expect("InputEvent expected");

        let event = unsafe { event.assume_safe() };

        if event.action_pressed("ui_accept") {
            gdp!("TEXTURE update");
            self.swap_palette(owner);
        }
    }

    fn swap_palette(&self, owner: &KinematicBody2D) {
        let animated_sprite = owner.get_and_cast::<AnimatedSprite>("player_sprite");

        let material = animated_sprite.material().unwrap();
        let shader_mat = material.cast::<ShaderMaterial>().unwrap();
        let texture = self.color_texture().unwrap();
    }

    fn color_texture(&self) -> Option<Texture> {
        None
    }
}
