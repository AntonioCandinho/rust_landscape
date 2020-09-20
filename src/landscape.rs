use crate::segment::Segment;

#[derive(Debug, PartialEq)]
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
            self.rain_for_an_hour();
        }
    }

    pub fn rain_for_an_hour(&mut self) {
        let mut w = 0;
        for segment in self.segments.iter_mut() {
            segment.rain += 1.0;
        }
        loop {
            let mut changed = false;
            println!("Segments {:?}", self.segments);
            for i in 0..self.segments.len() {
                w =  w + 1;
                changed |= self.view_segment_at_index(i);
            }
            if !changed {
                break;
            }
        }
        println!("{}", w);
    }

    fn view_segment_at_index(&mut self, i: usize) -> bool {
        let mut changed = false;
        if !self.segments[i].has_rain() {
            return changed;
        }
        let mut left_flow = if i > 0 
            { self.get_required_flow(i, i -1) }
            else { 0.0 };
        let mut right_flow = if i < self.segments.len() - 1 
            { self.get_required_flow(i, i + 1)}
            else { 0.0 };
        

        let max_rain = if right_flow > std::f64::EPSILON && left_flow > std::f64::EPSILON 
            { self.segments[i].rain / 2.0 }
            else { self.segments[i].rain };
            
        // optimize unit splitting
        if right_flow > std::f64::EPSILON && left_flow > std::f64::EPSILON {
            left_flow *= 2.0 / 3.0;
            right_flow *= 2.0 / 3.0;
        } else {
            left_flow *= 0.5;
            right_flow *= 0.5;
        }

        // prioritize right over left for fast convergence
        //right_flow = if right_flow.round() == 1.0
        //    { right_flow }
        //    else { right_flow / 2.0};
        //left_flow = if left_flow.round() == 1.0 && right_flow >= 1.0 
        //    { 0.0 } 
        //    else { left_flow / 2.0};
        
        if right_flow > std::f64::EPSILON {
            let required_flow = self.get_equalizing_move_flow(max_rain, right_flow);
            self.segments[i].rain -= required_flow;
            self.segments[i + 1].rain += required_flow;
            changed = true;
        }
        if left_flow > std::f64::EPSILON {
            let required_flow = self.get_equalizing_move_flow(max_rain, left_flow);
            self.segments[i].rain -= required_flow;
            self.segments[i - 1].rain += required_flow;
            changed = true;
        }
        changed
    }

    
    fn get_equalizing_move_flow(&self, max_rain: f64, required_flow: f64) -> f64 {
        if required_flow > max_rain {
            return max_rain;
        }
        required_flow
    }

    fn get_required_flow(&self, from: usize, to: usize) -> f64 {
        let diff = self.segments[from].get_total_height() - self.segments[to].get_total_height();
        if diff < 0.0 {
            return 0.0;
        }
         diff
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

