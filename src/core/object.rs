use super::transform::Transform;

pub struct WorldObject {
    pub id: u32,
    pub model_name: String,
    pub bytes: Vec<u8>,
    pub transform: Transform,
}