#[derive(Debug)]
#[derive(PartialEq)]
pub struct Segment {
    pub height: usize,
    pub rain: usize,
}

impl Segment {
    pub fn new(height: usize) -> Segment {
        Segment {height, rain: 0}
    }

    pub fn get_total_height(&self) -> usize {
        self.height + self.rain
    }
}

impl Clone for Segment {
    fn clone(&self) -> Segment {
        Segment { height: self.height, rain: self.rain }

    }
}
