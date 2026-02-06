pub struct Scene {
    objects: Vec<Box<dyn SceneObject>>,
}

impl Scene {
    // Construct a new scene
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    // Add a new object to the scene
    pub fn add(&mut self, mut obj: Box<dyn SceneObject>) {
        obj.ready();
        self.objects.push(obj);
    }
    // Update all the object of the scene, called all the frame
    pub fn update(&mut self, dt: f32) {
        for obj in self.objects.iter_mut() {
            obj.update(dt);
        }
    }
}