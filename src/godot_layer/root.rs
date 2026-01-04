use godot::prelude::*;

use crate::core::world::World;

/// Godot-facing wrapper around the core World.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct RealmWorld {
    world: Option<World>,
}

#[godot_api]
impl RealmWorld {
    /// Called when the node is created.
    #[func]
    pub fn initialize(&mut self) {
        godot_print!("RealmWorld.initialize() called");
        self.world = Some(World::new());
    }
}

#[godot_api]
impl INode for RealmWorld {
    fn init(_base: Base<Node>) -> Self {
        godot_print!("RealmWorld constructed");
        Self {
            world: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("RealmWorld ready()");
    }
}
