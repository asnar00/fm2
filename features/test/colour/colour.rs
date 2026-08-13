pub struct colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

struct feature_Colour;
impl feature_Colour {
    fn add(a: colour, b: colour) -> colour {
        colour { r: a.r + b.r, g: a.g + b.g, b: a.b + b.b, ..Default::default() }
    }
}
