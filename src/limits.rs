use std::ops::Range;

#[derive(Debug)]
pub struct Limits {
    pub angle: Range<f64>,
    pub length: f64,
    pub child_count: usize,
    pub depth: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            angle: -0.5..0.5, // radians
            length: 70.0,
            child_count: 2,
            depth: 15,
        }
    }
}
