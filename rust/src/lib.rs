use godot::prelude::*;

mod gltf_loader;
mod player;

struct Meetia;

#[gdextension]
unsafe impl ExtensionLibrary for Meetia {}
