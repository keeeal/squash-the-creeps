use godot::classes::{INode, Node, PathFollow3D, Timer};
use godot::global::randf;
use godot::prelude::*;

use crate::mob::Mob;
use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Node)]
struct Game {
    base: Base<Node>,
    mob_scene: Gd<PackedScene>,
}

#[godot_api]
impl INode for Game {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            mob_scene: load("res://scenes/mob.tscn"),
        }
    }

    fn ready(&mut self) {
        let callable = self.base_mut().callable("_on_mob_timer_timeout");
        let mut mob_timer = self.base().get_node_as::<Timer>("MobTimer");
        mob_timer.connect(StringName::from("timeout"), callable);
    }
}

#[godot_api]
impl Game {
    #[func]
    fn _on_mob_timer_timeout(&mut self) {
        let mut mob = self.mob_scene.instantiate_as::<Mob>();
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");
        mob_spawn_location.set_progress_ratio(randf() as f32);
        let player_position = self.base().get_node_as::<Player>("Player").get_position();
        mob.call(
            StringName::from("initialize"),
            &[
                Variant::from(mob_spawn_location.get_position()),
                Variant::from(player_position),
            ],
        );
        self.base_mut().add_child(mob.upcast());
    }
}
