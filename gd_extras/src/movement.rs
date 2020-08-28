use gdnative::api::KinematicBody2D;
use gdnative::prelude::*;

pub trait Move2D {
    // Default implementation of move_and_slide
    fn move_and_slide_default(&self, velocity: Vector2, up: Vector2) -> Vector2;
}

// KinematicBody2D
impl Move2D for KinematicBody2D {
    fn move_and_slide_default(&self, velocity: Vector2, up: Vector2) -> Vector2 {
        let stop_on_slope = false;
        let max_slides = 4;
        let floor_max_angle = 0.785998;
        let infinite_inertia = true;

        self.move_and_slide(
            velocity,
            up,
            stop_on_slope,
            max_slides,
            floor_max_angle,
            infinite_inertia,
        )
    }
}
