use std::f64::consts::PI;
use crate::math::Cf64;
use crate::math::random;

#[derive(Debug)]
pub struct Neuron {
    id: usize,
    weights: Vec<Cf64>,
    activation: ActivationFunction
}

#[derive(Debug)]
pub enum ActivationFunction {
    RELU,
    SIGMOID,
    TANH
}

impl Neuron {
    pub fn new(id: usize, inputs: usize, activation: ActivationFunction) -> Neuron {
        let weights: Vec<Cf64> = vec![Cf64::new(0.0, 0.0); inputs];

        Neuron { id, weights, activation }
    }

    pub fn shuffle(&mut self, seed: u128, scale: f64) {
        let mut next_val = seed;
        let mut rnd_val1: f64 = 0.0;
        let mut rnd_val2: f64 = 0.0;
        self.weights.iter_mut().map(|x| {
            (next_val, rnd_val1) = random::lcg(next_val);
            (next_val, rnd_val2) = random::lcg(next_val);

            x.q = rnd_val1 * scale;
            x.p = 2.0 * PI * rnd_val2 - PI;
        }).collect::<Vec<_>>();
    }

    pub fn signal(&self, input: Vec<f64>) {

    }
}