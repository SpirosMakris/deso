use gdnative::api::{GlobalConstants, InputEventMouse, Node2D};
use gdnative::prelude::*;

use gd_extras::gdp;

const THRESHOLD: f32 = 10.;

fn is_dragging(start: Vector2, current: Vector2) -> bool {
    let diff = (start - current).abs();

    diff.x >= THRESHOLD || diff.y >= THRESHOLD
}

enum Mouse {
    Up,                     // Mouse button not pressed
    Down(Vector2),          // Mouse pressed at location
    Drag(Vector2, Vector2), // Tracks the drag rectangle
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct UnitSelector {
    mouse: Mouse,
}

#[methods]
impl UnitSelector {
    pub fn new(_owner: &Node2D) -> Self {
        Self { mouse: Mouse::Up }
    }

    #[export]
    pub fn _unhandled_input(&mut self, owner: &Node2D, event: Variant) {
        let event = event.try_to_object::<InputEventMouse>();
        match event {
            Some(mouse_evt) => {
                // let mouse_evt = unsafe { mouse_evt.assume_safe() };
                // gdp!("MOUSE EVENT: {:?}", mouse_evt);
                self.mouse_input(owner, mouse_evt);
            }
            None => {}
        }
    }

    fn mouse_input(&mut self, owner: &Node2D, event: Ref<InputEventMouse>) {
        let event = unsafe { event.assume_safe() };

        // @TODO: This, oddly enough, detects ALL mouse pressed when tested
        let is_down =
            event.button_mask() & GlobalConstants::BUTTON_LEFT == GlobalConstants::BUTTON_LEFT;
        let mouse_pos = event.global_position();

        match is_down {
            // Previous state match
            false => match self.mouse {
                Mouse::Up => {}, // No change here
                Mouse::Down(start) => {
                    // Click
                    gdp!("MOUSE CLICK release");
                    self.mouse = Mouse::Up;
                },
                Mouse::Drag(start, end) => {
                    // Handle drag
                    gdp!("MOUSE DRAG release");
                    self.mouse = Mouse::Up;
                },
            },

            true => match self.mouse {
                Mouse::Up => {
                    // Previous was Up, so we enter down
                    self.mouse = Mouse::Down(mouse_pos);
                },

                Mouse::Down(start) => {
                    // Check if passed threshold for Dragging
                    if is_dragging(start, mouse_pos) {
                        self.mouse = Mouse::Drag(start, mouse_pos);
                    }
                },

                Mouse::Drag(_, ref mut end) => {
                    // Update end position
                    *end = mouse_pos;
                }
            },
        }
    }
}
