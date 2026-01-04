use std::collections::HashMap;
use super::object::WorldObject;
use super::transform::Transform;

pub struct World {
    pub objects: HashMap<u32, WorldObject>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add_object(
        &mut self,
        id: u32,
        model_name: String,
        bytes: Vec<u8>,
        transform: Transform,
    ) {
        self.objects.insert(id, WorldObject {
            id,
            model_name,
            bytes,
            transform,
        });
    }
}

