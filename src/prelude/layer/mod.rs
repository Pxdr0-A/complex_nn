use crate::prelude::neuron::ActivationFunction;
use crate::prelude::neuron::Neuron;

#[derive(Debug)]
pub struct Layer {
    id: usize,
    units: Vec<Neuron>,
    hidden: bool
}

impl Layer {
    // to create the input layer
    pub fn new(
        id: usize,
        n_units: usize,
        hidden: bool,
        activation: ActivationFunction,
        inputs: usize,
        scale: &f64) -> Layer {
        // inputs on each neuron are considered in spite of input length
        // error handling is done prior to this
        let mut units: Vec<Neuron> = Vec::with_capacity(n_units);
        let mut neuron;
        let seed_increment: u128 = 999;
        for u in 0..n_units {
            neuron = Neuron::new(
                u,
                inputs,
                // we need to clone because every neuron has its own activation
                activation.clone()
            );
            neuron.shuffle(
                (u as u128) + seed_increment,
                scale
            );

            units.push(neuron);
        }

        Layer { id, units, hidden }
    }
}