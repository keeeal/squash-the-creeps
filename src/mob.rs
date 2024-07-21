use godot::global::{randf_range, randi_range};
use std::f64::consts::FRAC_PI_4;

use godot::classes::{
    AnimationPlayer, CharacterBody3D, ICharacterBody3D, VisibleOnScreenNotifier3D,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Mob {
    base: Base<CharacterBody3D>,
    min_speed: f32,
    max_speed: f32,
}

#[godot_api]
impl ICharacterBody3D for Mob {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            min_speed: 10.0,
            max_speed: 18.0,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }

    fn ready(&mut self) {
        self.base()
            .get_node_as::<VisibleOnScreenNotifier3D>("VisibleOnScreenNotifier3D")
            .connect(
                StringName::from("screen_exited"),
                self.base().callable("on_screen_exited"),
            );
    }
}

#[godot_api]
impl Mob {
    #[func]
    fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
        self.base_mut()
            .look_at_from_position(start_position, player_position);
        self.base_mut()
            .rotate_y(randf_range(-FRAC_PI_4, FRAC_PI_4) as f32);
        let random_speed = randi_range(self.min_speed as i64, self.max_speed as i64);
        let rotation_y = self.base().get_rotation().y;
        self.base_mut().set_velocity(
            (Vector3::FORWARD * random_speed as f32).rotated(Vector3::UP, rotation_y),
        );
        self.base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer")
            .set_speed_scale(random_speed as f32 / self.min_speed)
    }

    #[func]
    fn on_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn squash(&mut self, combo_count: u32) {
        println!("Squashed with combo count: {}", combo_count);
        self.base_mut()
            .emit_signal(StringName::from("squashed"), &[Variant::from(combo_count)]);
        self.base_mut().queue_free();
    }

    #[signal]
    fn squashed(&self);
}
