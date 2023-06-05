use std::{sync::Arc, f64::consts::PI, vec};

use crate::prelude::dataset::TabularDataset;

use plotters::prelude::*;

pub fn scatter_template(x: Vec<f64>, y: Vec<f64>, target: Vec<f64>) {
    // should probabily check if x and y have the same len()
    let mut data: Vec<(f64, f64, f64)> = Vec::with_capacity(x.len());
    let mut max_target: f64 = -f64::INFINITY;
    for i in 0..x.len() {
        if target[i] > max_target {
            max_target = target[i];
        }
        data.push((x[i], y[i], target[i]));
    }


    let root_area = BitMapBackend::new("./out/data.png", (1200, 1000))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(-20.0..150.0, -20.0..150.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
    
    let mut color: Vec<f64> = vec![0.0; 3];
    ctx.draw_series(data.iter().map(|point| {
        color[0] = 255.0 * ((2.0 * PI * point.2 / max_target) + 0.0).sin();
        color[1] = 255.0 * ((2.0 * PI * point.2 / max_target) + PI/4.0).sin();
        color[2] = 255.0 * ((2.0 * PI * point.2 / max_target) + PI/2.0).sin();
        
        Circle::new(
            (point.0, point.1), 
            4, 
            &RGBColor(
                color[0] as u8, 
                color[1] as u8, 
                color[2] as u8))
    }))
        .unwrap();
}

pub fn arrange_points(
    dataset: &TabularDataset<String, f64, f64>, 
    shape: &[usize]
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {

    let mut x: Vec<f64> = Vec::with_capacity(shape[0]);
    let mut y: Vec<f64> = Vec::with_capacity(shape[0]);
    let mut target_vec: Vec<f64> = Vec::with_capacity(shape[0]);
    let mut point: &[f64];
    let mut target: &f64;
    for i in 0..shape[0] {
        (point, target) = dataset.row(&i);
        // assuming two dimensions
        x.push(point[0]);
        y.push(point[1]);

        target_vec.push(*target);
    }

    (x, y, target_vec)
}