use godot::classes::{ILabel, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Label)]
pub struct ScoreLabel {
    base: Base<Label>,
    score: i64,
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
    fn on_mob_squashed(&mut self) {
        self.score += 1;
        self.update_text();
    }
}
