use godot::classes::{ILabel, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Label)]
pub struct ScoreLabel {
    base: Base<Label>,
    score: u64,
}

#[godot_api]
impl ILabel for ScoreLabel {
    fn init(base: Base<Label>) -> Self {
        Self { base, score: 0 }
    }

    fn ready(&mut self) {
        self.update_text();
    }
}

impl ScoreLabel {
    fn update_text(&mut self) {
        let _score = self.score;
        self.base_mut()
            .set_text(GString::from(format!("Score: {}", _score)));
    }
}

#[godot_api]
impl ScoreLabel {
    #[func]
    fn on_mob_squashed(&mut self, combo_count: u32) {
        self.score += (2 as u64).pow(combo_count);
        self.update_text();
    }
}
