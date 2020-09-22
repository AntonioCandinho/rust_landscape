use crate::segment::Segment;

#[derive(Debug, PartialEq)]
pub struct VShape {
    segments: Vec<Segment>,
    left_index: usize,
    right_index: usize,
    left_limit: f64,
    right_limit: f64,
    minimal_height: f64,
}

impl VShape {
    pub fn new(mut segments: Vec<Segment>) -> VShape {
        let first_segment = &segments[0];
        let last_segment = &segments[segments.len() - 1];
        let left_limit = first_segment.height as f64;
        let right_limit = last_segment.height as f64;
        let left_index = first_segment.index as usize;
        let right_index = last_segment.index as usize;
        segments.sort_by(|a, b| a.height.cmp(&b.height));
        let minimal_height = segments[0].get_total_height();
        VShape {
            segments,
            left_index,
            right_index,
            left_limit,
            right_limit,
            minimal_height,
        }
    }

    pub fn joined_left(&self) -> bool {
        self.minimal_height >= self.left_limit
    }

    pub fn joined_right(&self) -> bool {
        self.minimal_height >= self.right_limit
    }

    pub fn right_join(&mut self, mut vshape: VShape) {
        self.right_limit = vshape.right_limit;
        self.minimal_height = if self.minimal_height < vshape.minimal_height
            { self.minimal_height }
            else { vshape.minimal_height };
        if self.right_index == vshape.left_index {
            for i in 0..vshape.segments.len() {
                if vshape.segments[i].index == vshape.left_index {
                    vshape.segments.remove(i);
                    break;
                }
            }
        }
        self.right_index = vshape.right_index;
        self.segments.append(&mut vshape.segments);
        self.segments.sort_by(
            |a, b| a.get_total_height().partial_cmp(&b.get_total_height()).unwrap(),
        );
    }

    pub fn fill(&mut self, mut rain: f64) -> f64 {
        while rain > 0.0 {
            let mut next_height: f64 = 0.0;
            let mut min_height_count: usize = 0;
            for i in 0..self.segments.len() {
                if self.segments[i].get_total_height() <= (self.minimal_height + f64::EPSILON) {
                    min_height_count += 1;
                    continue;
                }
                next_height = self.segments[i].get_total_height();
                break;
            }
            let required_rain = if next_height == 0.0
                { rain }
                else { (min_height_count as f64) * (next_height - self.minimal_height) };
            let consumed_rain = if required_rain > rain { rain } else { required_rain };
            for i in 0..min_height_count {
                self.segments[i].rain += consumed_rain / (min_height_count as f64);
            }
            self.minimal_height = self.segments[0].get_total_height();
            rain = if consumed_rain >= rain { 0.0 } else { rain - consumed_rain };
            if (next_height == self.left_limit || next_height == self.right_limit) &&
                self.minimal_height >= (next_height - f64::EPSILON) {
                break;
            }
        }
        rain
    }

    pub fn left_share(&self, vshape: &VShape) -> bool {
        self.left_index == vshape.right_index
    }

    pub fn right_share(&self, vshape: &VShape) -> bool {
        self.right_index == vshape.left_index
    }

    pub fn get_segments(&self) -> Vec<Segment> {
        let mut segments: Vec<Segment> = Vec::new();
        for segment in self.segments.iter() {
            segments.push(segment.clone());
        }
        segments
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }
}

impl Clone for VShape {
    fn clone(&self) -> VShape {
        VShape {
            segments: self.segments.clone(),
            left_limit: self.left_limit,
            right_limit: self.right_limit,
            minimal_height: self.minimal_height,
            right_index: self.right_index,
            left_index: self.left_index,
        }
    }
}
