//use rand::Rng;
extern crate rust_landscape;

use rust_landscape::Landscape;
use rust_landscape::Segment;


#[test]
fn given_some_example_landscape() {
    let mut landscape = Landscape::new(&vec![3, 1, 6, 4, 8, 9]);

    landscape.rain(1);
    assert_eq!(landscape.get_segments(), vec![
        Segment { height: 3, rain: 1.0 },
        Segment { height: 1, rain: 3.0 },
        Segment { height: 6, rain: 0.0 },
        Segment { height: 4, rain: 2.0 },
        Segment { height: 8, rain: 0.0 },
        Segment { height: 9, rain: 0.0 },
    ]);
    
    //assert_eq!(landscape.get_segments(), vec![
    //    Segment { height: 3, rain: 3.5 },
    //    Segment { height: 1, rain: 5.5 },
    //    Segment { height: 6, rain: 0.5 },
    //    Segment { height: 4, rain: 2.5 },
    //    Segment { height: 8, rain: 0.0 },
    //    Segment { height: 9, rain: 0.0 },
    //]);

    //landscape.rain(1);
    //assert_eq!(landscape.get_segments(), vec![
    //    Segment { height: 3, rain: 3 },
    //    Segment { height: 1, rain: 6 },
    //    Segment { height: 6, rain: 2 },
    //    Segment { height: 4, rain: 5 },
    //    Segment { height: 8, rain: 1 },
    //    Segment { height: 9, rain: 1 },
    //]);
}

//
//#[test]
//fn rain_should_move_as_it_drops() {
//    let mut landscape = given_random_landscape(100, 100);
//    let mut same_landscape = landscape.clone();
//
//    same_landscape.rain(3);
//    for _ in 0..3 {
//        landscape.rain(1);
//    }
//
//    assert_eq!(landscape, same_landscape);
//}
//
//#[test]
//fn every_landscape_should_eventually_become_steady() {
//    let mut landscape = given_random_landscape(100 as usize, 100 as usize);
//    landscape.rain(1000);
//    let segments = landscape.get_segments();
//
//    for i in 0..segments.len() - 1 {
//        let diff: isize = segments[i].get_total_height() as isize - segments[i+1].get_total_height() as isize;
//        if diff < -1 || diff > 1 {
//            panic!(
//                "In an steady system gaps between segments should not be greater than 1 {} {:?} {:?}",
//                diff,
//                segments[i],
//                segments[i+1],
//            );
//        }
//    }
//}
//
//fn given_random_landscape(max_segment_height: usize, max_segment_number: usize) -> Landscape {
//    let mut rng = rand::thread_rng();
//    let segments_count: usize = rng.gen_range(1, max_segment_number);
//    let mut segments: Vec<usize> = vec![0; segments_count];
//    for segment in segments.iter_mut() {
//        *segment = rng.gen_range(1, max_segment_height);
//    }
//    Landscape::new(&segments)
//}
