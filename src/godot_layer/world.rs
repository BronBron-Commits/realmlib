use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node, init)]
pub struct RealmWorld {
    #[base]
    base: Base<Node>,
}

/* ---------------- Rust-side constructor ---------------- */

impl RealmWorld {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

/* ---------------- Godot virtuals ---------------- */

#[godot_api]
impl INode for RealmWorld {
    fn ready(&mut self) {
        godot_print!("RealmWorld ready");
    }
}

/* ---------------- Godot-exposed API ---------------- */

#[godot_api]
impl RealmWorld {
    /// Smoke-test method to verify C# → Rust calls
    #[func]
    fn ping(&self) -> GString {
        godot_print!("RealmWorld.ping called");
        "realmlib alive".into()
    }

    /// Temporary test hook for object creation
    #[func]
    fn add_object(&mut self, id: i32, model: GString) {
        godot_print!(
            "RealmWorld.add_object called -> id={} model={}",
            id,
            model
        );
    }
}
