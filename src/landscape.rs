use std::collections::VecDeque;

use crate::segment::Segment;
use crate::vshape::VShape;

#[derive(Debug, PartialEq)]
pub struct Landscape {
    pub vshapes: VecDeque<VShape>,
    pub segment_count: usize,
}

impl Landscape {
    pub fn new(segment_heights: &[usize]) -> Landscape {
        let mut vshapes: VecDeque<VShape> = VecDeque::new();
        let mut segments: Vec<Segment> = Vec::new();
        let mut have_direction: bool = false;
        let mut ascending: bool = false;
        for i in 0..segment_heights.len() {
            segments.push(Segment::new(segment_heights[i], i));
            if  i + 1 == segment_heights.len()  {
                vshapes.push_back(VShape::new(segments));
                break;
            }
            if !have_direction {
                ascending = segment_heights[i] < segment_heights[i + 1];
                have_direction = true;
            }
            if ascending && segment_heights[i] > segment_heights[i + 1] {
                let mut same_height_segments: Vec<Segment> = Vec::new();
                for _ in 0..segments.len() {
                    let droped = segments.remove(segments.len() - 1);
                    let height = droped.height;
                    same_height_segments.push(droped);
                    if segments.len() != 0 {
                        if segments[segments.len() -1].height != height {
                            break;
                        }
                    }
                }
                same_height_segments.sort_by(|a, b| a.index.cmp(&b.index));
                let this_bp = (same_height_segments.len() as f64 / 2.0).ceil() as usize;
                for i in 0..this_bp {
                    segments.push(same_height_segments[i].clone());
                }
                vshapes.push_back(VShape::new(segments));
                segments = Vec::new();
                let next_bp = (same_height_segments.len() as f64 / 2.0).floor() as usize;
                for i in next_bp..same_height_segments.len() - 1  {
                    segments.push(same_height_segments[i].clone());
                }
                segments.push(Segment::new(segment_heights[i], i));
                ascending = false;
            }
            if !ascending && segment_heights[i] < segment_heights[i + 1] {
                ascending = true;
            }
        }
        Landscape { vshapes, segment_count: segment_heights.len() }
    }

    pub fn rain(&mut self, hours: usize) {
        let mut total_water = (hours * self.segment_count) as f64;
        let mut remaining_water = total_water;
        while total_water > (0.0 + f64::EPSILON) {
            for i in 0..self.vshapes.len() {
                let vshape_water = (total_water * self.get_vshape_water_factor(i)) / (self.segment_count as f64);
                let uneeded_water = self.vshapes[i].fill(vshape_water);
                remaining_water -= vshape_water - uneeded_water;
            }
            self.join_vshapes();
            total_water = remaining_water;
        }
    }

    fn join_vshapes(&mut self) {
        let mut joining = true;
        while joining {
            joining = false;
            for i in 0..self.vshapes.len() {
                if i + 1 < self.vshapes.len() && self.vshapes[i].joined_right()  {
                    let right_shape = self.vshapes.remove(i + 1);
                    self.vshapes[i].right_join(right_shape.unwrap());
                    joining = true;
                    break;
                }
                if i > 0 && self.vshapes[i].joined_left()  {
                    let my_shape = self.vshapes.remove(i);
                    self.vshapes[i - 1].right_join(my_shape.unwrap());
                    joining = true;
                    break;
                }
            }
        }
    }

    fn get_vshape_water_factor(&self, vshape_index: usize) -> f64 {
        let vshape = &self.vshapes[vshape_index];
        let mut vshape_water: f64 = (vshape.segment_count() as f64) - 1.0;
        if vshape_index == 0 {
            vshape_water += 0.5;
        }
        if vshape_index + 1 < self.vshapes.len() && !vshape.right_share(&self.vshapes[vshape_index + 1]) {
            vshape_water += 0.5;
        }
        if vshape_index + 1 == self.vshapes.len() {
            vshape_water += 0.5;
        }
        if vshape_index > 0 && !vshape.left_share(&self.vshapes[vshape_index - 1]) {
            vshape_water += 0.5;
           
        }
        vshape_water
    }

    pub fn get_segments(&self) -> Vec<Segment> {
        let mut segments: Vec<Segment> = Vec::new();
        for vshape in self.vshapes.iter() {
            let mut partial_segments = vshape.get_segments();
            segments.append(&mut partial_segments);
        }
        segments.sort_by(|a, b| a.index.cmp(&b.index));
        segments.dedup_by(|a, b| a.index == b.index);
        segments
    }

}

impl Clone for Landscape {
    fn clone(&self) -> Landscape {
        Landscape {
            segment_count: self.segment_count,
            vshapes: self.vshapes.clone(),
        }
    }
}
