use godot::prelude::*;

mod game;
mod mob;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
