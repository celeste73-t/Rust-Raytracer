use crate::scene::objects::SceneObject;
use crate::scene::components::{Transform, Color};

pub struct Light { 
    pub transform: Transform,
    pub color: Color,
    pub intensity: f32,
}

impl SceneObject for Light {
    fn ready() {}
    fn update() {}
}