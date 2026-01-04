use godot::prelude::*;
use crate::core::world::World;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static WORLD: Lazy<Mutex<World>> = Lazy::new(|| {
    Mutex::new(World::new())
});

#[derive(GodotClass)]
#[class(base=Node)]
pub struct RealmlibRoot {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl RealmlibRoot {
    #[func]
    pub fn init_world(&self) {
        let mut world = WORLD.lock().unwrap();
        world.objects.clear();
        godot_print!("realmlib: world initialized");
    }

    #[func]
    pub fn add_test_cube(&self) {
        godot_print!("realmlib: test object added (visual next)");
    }
}

#[godot_api]
impl NodeVirtual for RealmlibRoot {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}
