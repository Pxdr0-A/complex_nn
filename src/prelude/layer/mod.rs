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
        seed_increment: u128,
        scale: &f64
    ) -> Layer {
        // inputs on each neuron are considered in spite of input length
        // error handling is done prior to this
        let mut units: Vec<Neuron> = Vec::with_capacity(n_units);
        let mut neuron;
        for u in 0..n_units {
            neuron = Neuron::new(
                u + 1,
                inputs,
                // every neuron has its own activation
                activation.clone()
            );
            neuron.shuffle(
                ((u as u128) + 1) * seed_increment,
                scale
            );

            units.push(neuron);
        }

        Layer { id, units, hidden }
    }

    pub fn next_id(&self) -> usize {
        self.id + 1
    }

    pub fn get_units(&self) -> usize {
        self.units.len()
    }
}