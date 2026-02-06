pub trait SceneObject {
    // Called when the object is added to the scene
    fn ready(&mut self) {}
    // Called every frame
    fn update(&mut self, _dt: f32) {}
}
