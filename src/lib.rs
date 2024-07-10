use godot::classes::{CharacterBody3D, ICharacterBody3D};
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,

    speed: f32,
    fall_acceleration: f32,
    target_velocity: Vector3,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            speed: 14.0,
            fall_acceleration: 75.0,
            target_velocity: Vector3::ZERO,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        let mut direction = Vector3::ZERO;
        if input.is_action_pressed(StringName::from("move_right")) {
            direction.x += 1.0;
        }
        if input.is_action_pressed(StringName::from("move_left")) {
            direction.x -= 1.0;
        }
        if input.is_action_pressed(StringName::from("move_back")) {
            direction.z += 1.0;
        }
        if input.is_action_pressed(StringName::from("move_forward")) {
            direction.z -= 1.0;
        }
        if direction != Vector3::ZERO {
            direction = direction.normalized();
            self.base()
                .get_node_as::<Node3D>("Pivot")
                .set_basis(Basis::new_looking_at(direction, Vector3::UP, false));
        }

        self.target_velocity.x = direction.x * self.speed;
        self.target_velocity.z = direction.z * self.speed;

        if !self.base().is_on_floor() {
            self.target_velocity.y -= self.fall_acceleration * delta as f32;
        }

        let _target_velocity = self.target_velocity.clone();
        self.base_mut().set_velocity(_target_velocity);
        self.base_mut().move_and_slide();
    }
}
