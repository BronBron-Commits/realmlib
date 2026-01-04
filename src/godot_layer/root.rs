use godot::prelude::*;

use crate::core::world::World;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct RealmWorld {
    #[base]
    base: Base<Node>,

    world: World,
}

#[godot_api]
impl RealmWorld {
    #[func]
    pub fn debug_ping(&self) {
        godot_print!("RealmWorld.debug_ping() called");
    }
}

#[godot_api]
impl INode for RealmWorld {
    fn init(base: Base<Node>) -> Self {
        godot_print!("RealmWorld constructed");

        Self {
            base,
            world: World::new(),
        }
    }

    fn ready(&mut self) {
        godot_print!("RealmWorld ready()");
    }
}
