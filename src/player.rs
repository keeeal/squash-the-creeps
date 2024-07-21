use godot::classes::{AnimationPlayer, Area3D, CharacterBody3D, ICharacterBody3D};
use godot::prelude::*;
use std::f32::consts::FRAC_PI_6;

use crate::mob::Mob;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    base: Base<CharacterBody3D>,
    speed: f32,
    fall_acceleration: f32,
    jump_impulse: f32,
    bounce_impulse: f32,
    target_velocity: Vector3,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            speed: 14.0,
            fall_acceleration: 75.0,
            jump_impulse: 20.0,
            bounce_impulse: 16.0,
            target_velocity: Vector3::ZERO,
        }
    }

    fn ready(&mut self) {
        self.base().get_node_as::<Area3D>("MobDetector").connect(
            StringName::from("body_entered"),
            self.base().callable("on_mob_detector_body_entered"),
        );
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
                .look_at(self.base().get_position() + direction);
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .set_speed_scale(4.0);
        } else {
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .set_speed_scale(1.0);
        }

        self.target_velocity.x = direction.x * self.speed;
        self.target_velocity.z = direction.z * self.speed;

        if !self.base().is_on_floor() {
            self.target_velocity.y -= self.fall_acceleration * delta as f32;
        } else if input.is_action_just_pressed(StringName::from("move_jump")) {
            self.target_velocity.y = self.jump_impulse;
        }

        for index in 0..self.base().get_slide_collision_count() {
            let collision = self.base_mut().get_slide_collision(index).unwrap();
            let collider = match collision.get_collider() {
                Some(collider) => collider.try_cast::<Node3D>().unwrap(),
                None => continue,
            };
            if collider.is_in_group(StringName::from("mob")) {
                let mut mob = collider.try_cast::<Mob>().unwrap();
                if Vector3::UP.dot(collision.get_normal()) > 0.1 {
                    mob.call(StringName::from("squash"), &[]);
                    self.target_velocity.y = self.bounce_impulse;
                    break;
                }
            }
        }

        let _target_velocity = self.target_velocity.clone();
        self.base_mut().set_velocity(_target_velocity);
        self.base_mut().move_and_slide();

        let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
        let mut pivot_rotation = pivot.get_rotation();
        pivot_rotation.x = FRAC_PI_6 * self.base().get_velocity().y / self.jump_impulse;
        pivot.set_rotation(pivot_rotation);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn on_mob_detector_body_entered(&mut self, _body: Gd<Node3D>) {
        self.die();
    }

    #[func]
    fn die(&mut self) {
        self.base_mut().emit_signal(StringName::from("hit"), &[]);
        self.base_mut().queue_free();
    }

    #[signal]
    fn hit(&self);
}
