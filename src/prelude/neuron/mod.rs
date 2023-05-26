use std::f64::consts::PI;
use crate::math::Cf64;
use crate::math::random;
use crate::math::backpropagation;

#[derive(Debug)]
pub struct Neuron {
    id: usize,
    weights: Vec<Cf64>,
    activation: ActivationFunction
}

#[derive(Debug)]
pub enum ActivationFunction {
    SIGMOID,
    TANH,
    RELU
}

impl Neuron {
    pub fn new(id: usize, inputs: usize, activation: ActivationFunction) -> Neuron {
        let weights: Vec<Cf64> = vec![Cf64::new(0.0, 0.0); inputs];

        Neuron { id, weights, activation }
    }

    pub fn shuffle(&mut self, mut seed: u128, scale: &f64) {
        let mut rnd_val1: f64 = 0.0;
        let mut rnd_val2: f64 = 0.0;
        self.weights = self.weights.iter().map(|_| {
            rnd_val1 = random::lcg(&mut seed);
            rnd_val2 = random::lcg(&mut seed);

            Cf64::new(
                rnd_val1 * scale,
                2.0 * PI * rnd_val2 - PI
            )
        }).collect();
    }

    pub fn signal(&self, input: &[f64]) -> f64 {
        // method 1 for feed forward
        // check if input length is in agreement with neuron inputs
        assert_eq!(self.weights.len(), input.len(),
                   "Input length must match the number of neuron inputs."
        );

        let mut output = Cf64::new(0.0, 0.0);
        for i in 0..input.len() {
            // consider parallelism
            output = Cf64::add(
                &output,
                &Cf64::mul(
                    &self.weights[i],
                    &Cf64::new(
                          input[i],
                          if input[i].is_sign_positive() { 0.0 } else { PI })
            ));
        }

        // build a macro out of this section
        // hopefully to incorporate different approaches to decompose the complex output
        self.activation.activate(&output)

    }
}

impl ActivationFunction {
    fn activate(&self, output: &Cf64) -> f64 {
        match self {
            ActivationFunction::SIGMOID => {
                let active = |x: f64| -> f64 {
                    1.0 / (1.0 + (-x).exp())
                };

                active(output.q * output.p.cos())
            },
            ActivationFunction::TANH => {
                let active = |x: f64| -> f64 {
                    x.tanh()
                };

                active(output.q * output.p.cos())
            },
            ActivationFunction::RELU => {
                let active = |x: f64| -> f64 {
                    if x.is_sign_positive() { x } else { 0.0 }
                };

                active(output.q * output.p.cos())
            }
        }
    }
}