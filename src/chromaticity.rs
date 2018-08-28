pub struct Chromaticity {
    pub x: f32,
    pub y: f32,
}

impl Chromaticity {
    fn new(x: f32, y: f32) -> Chromaticity {
        Chromaticity{x, y}
    }
}