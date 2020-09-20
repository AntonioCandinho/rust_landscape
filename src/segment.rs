#[derive(Debug,PartialEq)]
pub struct Segment {
    pub index: usize,
    pub height: usize,
    pub rain: f64,
}

impl Segment {
    pub fn new(height: usize, index: usize) -> Segment {
        Segment {index, height, rain: 0.0}
    }

    pub fn get_total_height(&self) -> f64 {
        (self.height as f64) + self.rain
    }

    pub fn has_rain(&self) -> bool {
        self.rain > 0.0
    }
}

impl Clone for Segment {
    fn clone(&self) -> Segment {
        Segment { index: self.index, height: self.height, rain: self.rain }

    }
}
