#[derive(Debug,PartialEq)]
pub struct Segment {
    pub height: usize,
    pub rain: f64,
}

impl Segment {
    pub fn new(height: usize) -> Segment {
        Segment {height, rain: 0.0}
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
        Segment { height: self.height, rain: self.rain }

    }
}
