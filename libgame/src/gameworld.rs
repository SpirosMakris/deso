use gd_extras::input::InputEventExt;
use gd_extras::{gd_err, gdp};
use gdnative::prelude::*;

use gdnative::api::{PackedScene, ResourceLoader};

use lazy_static::lazy_static;
use legion::*;

use std::sync::Mutex;

// WORLD
lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::default());
}

pub fn with_world<F>(mut f: F)
where
    F: FnMut(&mut World),
{
    let res = WORLD.try_lock().map(|mut world| f(&mut world));
    if res.is_err() {
        gd_err!("with_world: {:?}", res);
    }
}

// RESOURCES
pub struct Delta(pub f32);

// SCHEDULES
fn setup_process_schedule() -> Schedule {
    Schedule::builder()
        // .add_thread_local(move_node_system())
        .build()
}

use crate::player;

fn setup_physics_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(player::move_player_system())
        .build()
}

// COMPONENTS
// #[derive(Debug, Copy, Clone, PartialEq)]
// @COMP
// pub struct Kine2Comp(Ref<KinematicBody2D, Shared>);

// unsafe impl Send for Kine2Comp {}
// unsafe impl Sync for Kine2Comp {}

// @COMP
#[derive(Debug, Copy, Clone)]
pub struct Direction(pub Vector2);

// SYSTEMS
// There are two types of systems that can be added in Legion
// (three if you consider thread_local_fn).
// A Runnable (thread local) and a Schedulable (not thread local).
// To keep things simple anything that touches the scene tree to be part of a Runnable
//  and everything that is just plain data to be Schedulable.
// Note that it’s safe to touch the scene tree when adding a thread local system.
// Therefore all systems that manipulate Godot nodes should be added with add_thread_local.
// Components that are wrapping Godot nodes requires unsafe impl of Send and Sync.

// #[system(for_each)]
// fn move_node(node: &mut Kine2Comp, #[resource] delta: &Delta) {
//     let speed = 100f32;
//     let vel = Vector2::new(1.0, 0.0) * speed * delta.0;

//     let inner = unsafe { node.0.assume_safe() };
//     inner.global_translate(vel);
// }

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GameWorld {
    resources: Resources,
    physics: Schedule,
    process: Schedule,
}

#[methods]
impl GameWorld {
    pub fn new(_owner: &Node2D) -> Self {
        let process = setup_process_schedule();
        let physics = setup_physics_schedule();

        let mut resources = Resources::default();
        resources.insert(Delta(0.));

        Self {
            resources,
            physics,
            process,
        }
    }

    #[export]
    pub fn _ready(&self, owner: &Node2D) {
        gdp!("GameWorld _ready()");

        // let node = owner.get_and_cast::<KinematicBody2D>("Player").claim();

        // with_world(|world| {
        //     world.push((Kine2Comp(node),));
        // });

        // Create a player
        for i in 0..4 {

            let player_position = Vector2::new(100. * i as f32 + 50., 100.);
            let player = create_player(player_position);
            owner.add_child(player, true);
            
            with_world(|world| {
                world.push(
                    (Direction(Vector2::new(1., 0.)), player::Player(player))
                );
            });
        }
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
    pub fn _process(&mut self, _owner: &Node2D, _delta: f64) {
        // Execute all process ECS systems
        with_world(|world| {
            self.process.execute(world, &mut self.resources);
        });
    }

    #[export]
    pub fn _physics_process(&mut self, _owner: &Node2D, delta: f64) {
        // Update Delta resource
        self.resources
            .get_mut::<Delta>()
            .map(|mut d| d.0 = delta as f32);

        // Execute physics process ECS systems
        with_world(|world| {
            self.physics.execute(world, &mut self.resources);
        });
    }
}

fn create_player(pos: Vector2) -> Ref<KinematicBody2D> {
    let loader = ResourceLoader::godot_singleton();
    let scene = loader
        .load("res://scenes/player.tscn", "PackedScene", false)
        .unwrap();

    let scene = scene.cast::<PackedScene>().unwrap();

    let scene = unsafe { scene.assume_safe() };

    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .expect("Should be able to instance Player scene");

    let instance = unsafe { instance.assume_safe() };

    let instance = instance
        .cast::<KinematicBody2D>()
        .expect("Could not cast loaded scene into Kine2D");
    instance.set_position(pos);
    instance.claim()
}
