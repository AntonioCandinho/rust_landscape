use crate::segment::Segment;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Landscape {
    segments: Vec<Segment>
}

impl Landscape {
    pub fn new(segment_heights: &[usize]) -> Landscape {
        Landscape{
            segments: segment_heights.iter()
                                     .map(|height| Segment::new(*height))
                                     .collect()
        }
    }

    pub fn from_segments(segments: &[Segment]) -> Landscape {
        Landscape {segments: segments.to_vec()}
    }

    pub fn rain(&mut self, hours: usize) {
        for _ in 0..hours {
            for segment in self.segments.iter_mut() {
                segment.rain += 1;
            }
            loop {
                let mut changed = false;
                for i in 0..self.segments.len() {
                    if i > 0 {
                        let count = units_of_water_to_neighbour(&self.segments[i], &self.segments[i -1]);
                        if count > 0 {
                            self.segments[i].rain -= count;
                            self.segments[i -1].rain += count;
                            changed = true
                        }
                    }
                    if i < self.segments.len() -1 {
                        let count = units_of_water_to_neighbour(&self.segments[i], &self.segments[i + 1]);
                        if count > 0 {
                            self.segments[i].rain -= count;
                            self.segments[i + 1].rain += count;
                            changed = true
                        }
                    }
                }
                if !changed {
                    break;
                }
            }
        }
    }

    pub fn get_segments(&self) -> Vec<Segment> {
        self.segments.clone()
    }

}

impl Clone for Landscape {
    fn clone(&self) -> Landscape {
        Landscape { segments: self.get_segments() }
    }
}

fn units_of_water_to_neighbour(from: &Segment, to: &Segment) -> usize {
    let from_height = from.height + from.rain - 1;
    let to_height = to.height + to.rain;
    if from_height > to_height {
        let rain = from_height - to_height;
        return std::cmp::min(rain, from.rain);
    }
    0
}
