// use gdnative::api::{AnimatedSprite, ImageTexture, ShaderMaterial, Texture};
use gdnative::prelude::*;

// use gd_extras::gdp;
use gd_extras::movement::Move2D;
// use gd_extras::input::InputEventExt;
// use gd_extras::node_ext::NodeExt;

use legion::*;

use crate::gameworld::{Delta, Direction};

// COMPONENTS

// #[derive(NativeClass)]
// #[inherit(KinematicBody2D)]
// @COMP
pub struct Player(pub Ref<KinematicBody2D, Shared>);

unsafe impl Send for Player {}
unsafe impl Sync for Player {}

#[system(for_each)]
pub fn move_player(dir: &mut Direction, player: &mut Player, #[resource] delta_sec: &Delta) {
    // gdp!("SYS: move_player with dT: {}", delta_sec.0);
    const SPEED: f32 = 2000.0;
    // let new_position = pos.0 + Vector2::new(SPEED * delta_sec.0, 0.);
    dir.0 = dir.0.normalize();
    let vel = dir.0 * SPEED * delta_sec.0;

    let player = unsafe { player.0.assume_safe() };
    // player.translate(vel);
    player.move_and_slide_default(vel, Vector2::zero());
}

// #[methods]
// impl Player {
//     pub fn new(_owner: &KinematicBody2D) -> Self {
//         Self {}
//     }

//     #[export]
//     pub fn _unhandled_input(&self, owner: &KinematicBody2D, event: Variant) {
//         gdp!("_unhandled_input()");
//         let event = event
//             .try_to_object::<InputEvent>()
//             .expect("InputEvent expected");

//         let event = unsafe { event.assume_safe() };

//         if event.action_pressed("ui_accept") {
//             gdp!("TEXTURE update");
//             self.swap_palette(owner);
//         }
//     }

//     fn swap_palette(&self, owner: &KinematicBody2D) {
//         gdp!("swap_palette()");
//         let animated_sprite = owner.get_and_cast::<AnimatedSprite>("player_sprite");

//         let material = animated_sprite.material().unwrap();
//         let shader_mat = material.cast::<ShaderMaterial>().unwrap();
//         let shader_mat = unsafe { shader_mat.assume_safe() };

//         let texture = self.color_texture();
//         let texture = unsafe { texture.assume_unique() };

//         shader_mat.set_shader_param("palette", texture);
//     }

//     fn color_texture(&self) -> Ref<Texture, Unique> {
//         gdp!("color_texture()");

//         let image = Image::new();
//         image.create(256, 256, false, Image::FORMAT_RGBA8);

//         image.lock();
//         for y in 0..256 {
//             for x in 0..256 {
//                 image.set_pixel(x, y, Color::rgb(0.0, 0.0, 0.0));
//             }
//         }
//         image.unlock();

//         let image_tex = ImageTexture::new();
//         image_tex.create_from_image(image, 0);

//         image_tex.upcast::<Texture>()
//     }
// }
