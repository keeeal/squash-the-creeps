use godot::prelude::*;

mod game;
mod mob;
mod player;
mod score_label;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
