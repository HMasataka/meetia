use godot::prelude::*;

mod gltf_loader;
mod player;
mod vr_player;
mod world_builder;

struct Meetia;

#[gdextension]
unsafe impl ExtensionLibrary for Meetia {}
