use godot::prelude::*;

mod core;
mod godot_layer;

struct RealmlibExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RealmlibExtension {}

