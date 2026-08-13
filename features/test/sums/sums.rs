struct feature_Sums;
impl feature_Sums {
    fn main() {
        existing.main();
        let c = add(colour { r: 0.1, g: 0.2, b: 0.3, ..Default::default() },
                    colour { r: 0.4, g: 0.4, b: 0.4, ..Default::default() });
        println!("colour sum: {:?}", c);
        let v = add(vec2 { x: 1.0, y: 2.0 }, vec2 { x: 3.0, y: 4.0 });
        println!("vec sum: {:?}", v);
        let w = vec2 { x: 10.0, y: 20.0 } + vec2 { x: 1.0, y: 2.0 };
        println!("vec op+: {:?}", w);
    }
}
