pub struct vec2 {
    pub x: f32,
    pub y: f32,
}

struct feature_Vec;
impl feature_Vec {
    fn add(a: vec2, b: vec2) -> vec2 {
        vec2 { x: a.x + b.x, y: a.y + b.y }
    }
}
