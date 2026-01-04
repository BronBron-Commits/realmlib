#[derive(Copy, Clone)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
}

impl Transform {
    pub fn new(px: f32, py: f32, pz: f32) -> Self {
        Self {
            position: [px, py, pz],
            rotation: [0.0, 0.0, 0.0],
        }
    }
}