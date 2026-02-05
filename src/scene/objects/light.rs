mod transform;
mod color;

pub struct Light { 
    pub transform: Transform,
    pub color: Color,
    pub intensity: f32,
}