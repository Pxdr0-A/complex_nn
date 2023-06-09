/* Miscellaneous functions

    This script is does not worry about optimization!

    Function here are used to aid the creation of the sample dataset:
        Poor eficiency may be found from now on.    

 */

use std::collections::HashMap;
use crate::{math::random::lcg, prelude::dataset::TabularDataset};

pub fn build_random_centers(centers: &mut HashMap<String, Vec<f64>>,
    shape: &[usize],
    n_classes: usize,
    mut seed: u128
) {
    // loop scope for constructing centers
    // i n classes
    let mut scale;
    for n in 0..n_classes {
        let mut added_vec: Vec<f64> = Vec::with_capacity(shape[1]);
        // j coordinate (features)
        for _ in 0..shape[1] {
            // build centers out of random numbers
            scale = lcg(&mut seed) * 100.0;
            added_vec.push(scale);
        }
        centers.insert(
        format!("center {}", n).to_string(),
        added_vec);
    }
}

pub fn add_random_points(
    dataset: &mut TabularDataset<String, f64, f64>,
    centers: &HashMap<String, Vec<f64>>,
    shape: &[usize],
    n_classes: usize,
    mut seed: u128
) {
    // rest of the rows (n_classes were already done)
    let mut row: Vec<f64>;
    let mut class_val: f64;
    let mut key: String;
    let mut center: &Vec<f64>;
    let mut index: usize;
    // for an additional random number
    for c in 0..shape[0] {

        // guarantee the initial values are one of each class
        if c < n_classes {
            class_val = c as f64;
        } else {
            class_val = (lcg(&mut seed) * (n_classes as f64 - 1.0)).round();
        }
        
        key = format!("center {}", class_val as usize).to_string();

        // unwrap does not panic
        // expect panics if the result is None
        center = centers.get(&key)
            .expect("Did not find the value for the search key");

        index = 0;
        let mut scale: f64 = 0.0;
        let mut signal_val: f64 = 0.0;
        row = vec![1.0; shape[1]].into_iter().map(|x| {
            // go through the coordinates
            index += 1;
            
            scale = lcg(&mut seed) * 10.0;
            signal_val = if lcg(&mut seed).round() == 0.0 {
                lcg(&mut seed)
            } else {
                -lcg(&mut seed)};

            x * center[index - 1] + signal_val * scale
        }).collect();
        
        dataset.add_row(&mut row, class_val);
    }
}