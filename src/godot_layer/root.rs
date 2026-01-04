use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct RealmWorld {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl RealmWorld {
    /// Simple callable method to prove Godot -> Rust works
    #[func]
    pub fn debug_ping(&self) {
        godot_print!("RealmWorld.debug_ping() called");
    }
}

#[godot_api]
impl INode for RealmWorld {
    fn init(base: Base<Node>) -> Self {
        godot_print!("RealmWorld constructed");
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("RealmWorld ready()");
    }
}
