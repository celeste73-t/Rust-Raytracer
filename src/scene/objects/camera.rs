use crate::scene::objects::SceneObject;
use crate::scene::components::Transform;

pub struct Camera {
    pub transform: Transform,
    pub fov: i32,
}

impl SceneObject for Camera {
    fn ready() {}
    fn update() {}
}