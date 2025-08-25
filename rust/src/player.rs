use godot::classes::{INode3D, Input, Node3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Player3D {
    speed: f32,
    rotation_speed: f32,
    base: Base<Node3D>,
}

#[godot_api]
impl Player3D {
    #[func]
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    #[func]
    pub fn set_rotation_speed(&mut self, rotation_speed: f32) {
        self.rotation_speed = rotation_speed;
    }
}

#[godot_api]
impl INode3D for Player3D {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            speed: 5.0,
            rotation_speed: 2.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut movement = Vector3::ZERO;
        let mut rotation_input = 0.0;

        // WASD movement
        if input.is_action_pressed("move_forward") {
            movement.z -= 1.0;
        }
        if input.is_action_pressed("move_backward") {
            movement.z += 1.0;
        }
        if input.is_action_pressed("move_left") {
            movement.x -= 1.0;
        }
        if input.is_action_pressed("move_right") {
            movement.x += 1.0;
        }

        // Mouse rotation or arrow keys for rotation
        if input.is_action_pressed("rotate_left") {
            rotation_input -= 1.0;
        }
        if input.is_action_pressed("rotate_right") {
            rotation_input += 1.0;
        }

        // Apply movement relative to current rotation
        if movement.length() > 0.0 {
            movement = movement.normalized() * self.speed * delta as f32;

            // Transform movement based on current rotation
            let transform = self.base().get_transform();
            let forward = -transform.basis.col_c(); // Z axis (forward)
            let right = transform.basis.col_a(); // X axis (right)

            let world_movement = forward * movement.z + right * movement.x;

            let current_position = self.base().get_position();
            self.base_mut()
                .set_position(current_position + world_movement);
        }

        // Apply rotation
        if rotation_input != 0.0 {
            let rotation_amount = rotation_input * self.rotation_speed * delta as f32;
            self.base_mut().rotate_y(rotation_amount);
        }
    }
}

