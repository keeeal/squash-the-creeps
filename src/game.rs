use godot::classes::{ColorRect, INode, InputEvent, Node, PathFollow3D, Timer};
use godot::global::randf;
use godot::prelude::*;

use crate::mob::Mob;
use crate::player::Player;
use crate::score_label::ScoreLabel;

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
        self.base().get_node_as::<Timer>("MobTimer").connect(
            StringName::from("timeout"),
            self.base().callable("on_mob_timer_timeout"),
        );
        self.base().get_node_as::<Player>("Player").connect(
            StringName::from("hit"),
            self.base().callable("on_player_hit"),
        );

        self.base()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .hide();
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed(StringName::from("ui_accept"))
            && self
                .base()
                .get_node_as::<ColorRect>("UserInterface/Retry")
                .is_visible()
        {
            self.base().get_tree().unwrap().reload_current_scene();
        }
    }
}

#[godot_api]
impl Game {
    #[func]
    fn on_mob_timer_timeout(&mut self) {
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
        mob.connect(
            StringName::from("squashed"),
            self.base()
                .get_node_as::<ScoreLabel>("UserInterface/ScoreLabel")
                .callable("on_mob_squashed"),
        );
        self.base_mut().add_child(mob.upcast());
    }

    #[func]
    fn on_player_hit(&mut self) {
        self.base().get_node_as::<Timer>("MobTimer").stop();
        self.base()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .show();
    }
}
