use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct RealmlibRoot {
    #[base]
    base: Base<Node>,
}

impl RealmlibRoot {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl INode for RealmlibRoot {
    fn ready(&mut self) {
        godot_print!("RealmlibRoot ready");
    }
}
