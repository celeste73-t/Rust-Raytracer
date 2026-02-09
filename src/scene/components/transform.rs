use glam::*;

pub struct Transform {
    pub scale: Vec3,
    pub rotation: Vec3,
    pub position: Vec3,
}

impl Transform {
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale,
            glam::Quat::from_euler(
                glam::EulerRot::XYZ,
                self.rotation.x,
                self.rotation.y,
                self.rotation.z,
            ),
            self.translation,
        )
    }
}