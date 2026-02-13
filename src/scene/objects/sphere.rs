use crate::scene::objects::SceneObject;
use crate::scene::components::{Transform, Color};

pub struct Sphere {
    pub transform: Transform,
    pub color: Color,
    pub radius: f32,
}

impl SceneObject for Sphere {
    fn ready(&mut self) {}
    fn update(&mut self, _delta:f32) {}
}