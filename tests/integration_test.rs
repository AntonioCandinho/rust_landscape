use rand::Rng;
extern crate rust_landscape;

use rust_landscape::Landscape;
use rust_landscape::Segment;


#[test]
fn given_some_example_landscape() {
    let mut landscape = Landscape::new(&vec![3, 1, 6, 4, 8, 9]);

    landscape.rain(1);
    assert_eq!(landscape.get_segments(), vec![
        Segment { height: 3, rain: 1.0, index: 0},
        Segment { height: 1, rain: 3.0, index: 1},
        Segment { height: 6, rain: 0.0, index: 2},
        Segment { height: 4, rain: 2.0, index: 3},
        Segment { height: 8, rain: 0.0, index: 4},
        Segment { height: 9, rain: 0.0, index: 5},
    ]);

    landscape.rain(1);
    assert_eq!(landscape.get_segments(), vec![
        Segment { height: 3, rain: 3.5, index: 0},
        Segment { height: 1, rain: 5.5, index: 1},
        Segment { height: 6, rain: 0.5, index: 2},
        Segment { height: 4, rain: 2.5, index: 3},
        Segment { height: 8, rain: 0.0, index: 4},
        Segment { height: 9, rain: 0.0, index: 5},
    ]);

    landscape.rain(1);
    assert_eq!(landscape.get_segments(), vec![
        Segment { height: 3, rain: 5.0, index: 0},
        Segment { height: 1, rain: 7.0, index: 1},
        Segment { height: 6, rain: 2.0, index: 2},
        Segment { height: 4, rain: 4.0, index: 3},
        Segment { height: 8, rain: 0.0, index: 4},
        Segment { height: 9, rain: 0.0, index: 5},
    ]);
}

#[test]
fn should_work_with_repeated_height_landscapes() {
    let mut landscape = Landscape::new(&vec![1, 8, 8, 8, 1]);
    landscape.rain(1);
    assert_eq!(landscape.get_segments(), vec![
        Segment { height: 1, rain: 2.5, index: 0},
        Segment { height: 8, rain: 0.0, index: 1},
        Segment { height: 8, rain: 0.0, index: 2},
        Segment { height: 8, rain: 0.0, index: 3},
        Segment { height: 1, rain: 2.5, index: 4},
    ]);

}

#[test]
fn should_work_with_stair_landscapes() {
    let mut landscape = Landscape::new(&vec![1, 2, 3, 4, 5, 6]);
    landscape.rain(1);
    assert_eq!(landscape.get_segments(), vec![
        Segment { height: 1, rain: 3.0, index: 0},
        Segment { height: 2, rain: 2.0, index: 1},
        Segment { height: 3, rain: 1.0, index: 2},
        Segment { height: 4, rain: 0.0, index: 3},
        Segment { height: 5, rain: 0.0, index: 4},
        Segment { height: 6, rain: 0.0, index: 5},
    ]);
}


#[test]
fn should_work_with_stair_saw_tooth() {
    let mut landscape = Landscape::new(&vec![1, 2, 1, 2, 1, 2]);
    landscape.rain(1);
    assert_eq!(landscape.get_segments(), vec![
        Segment { height: 1, rain: 1.5, index: 0},
        Segment { height: 2, rain: 0.5, index: 1},
        Segment { height: 1, rain: 1.5, index: 2},
        Segment { height: 2, rain: 0.5, index: 3},
        Segment { height: 1, rain: 1.5, index: 4},
        Segment { height: 2, rain: 0.5, index: 5},
    ]);
}

// PROPERTY BASED UNIT TESTING
//
// Something like this would be useful
// https://docs.rs/proptest/0.10.1/proptest/
//
#[test]
fn every_landscape_should_eventually_become_steady() {
    let mut landscape = given_random_landscape(100 as usize, 1000 as usize);
    landscape.rain(1000);
    let segments = landscape.get_segments();

    for i in 0..segments.len() - 1 {
        let diff: isize = segments[i].get_total_height() as isize - segments[i+1].get_total_height() as isize;
        if diff < -1 || diff > 1 {
            panic!(
                "In an steady system gaps between segments should not be greater than 1 {} {:?} {:?}",
                diff,
                segments[i],
                segments[i+1],
            );
        }
    }
}

#[test]
fn n_hours_of_1_unit_rain_should_be_the_same_as_1_hour_of_n_unit_rain() {
    let rain_time = 1000 as usize;
    let mut landscape = given_random_landscape(100 as usize, 100 as usize);
    let mut landscape_copy = landscape.clone();
    for _ in 0..rain_time {
        landscape.rain(1);
    }
    landscape_copy.rain(rain_time);
    let segments = landscape.get_segments();
    let copy_segments = landscape_copy.get_segments();
    for i in 0..segments.len() {
        assert_eq!(segments[i].index, copy_segments[i].index);
        assert_eq!(segments[i].height, copy_segments[i].height);
        assert_eq!((segments[i].rain - copy_segments[i].rain).round(), 0.0);
    }
}

fn given_random_landscape(max_segment_height: usize, max_segment_number: usize) -> Landscape {
    let mut rng = rand::thread_rng();
    let segments_count: usize = rng.gen_range(1, max_segment_number);
    let mut segments: Vec<usize> = vec![0; segments_count];
    for segment in segments.iter_mut() {
        *segment = rng.gen_range(1, max_segment_height);
    }
    Landscape::new(&segments)
}
