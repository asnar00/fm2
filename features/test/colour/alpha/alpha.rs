pub struct colour {
    pub a: f32,
}

struct feature_Alpha;
impl feature_Alpha {
    fn add(a: colour, b: colour) -> colour {
        let alpha_sum = a.a + b.a;
        let mut c = existing.add(a, b);
        c.a = alpha_sum;
        c
    }
}
