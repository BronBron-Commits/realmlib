use godot::prelude::*;

struct RealmlibExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RealmlibExtension {}
