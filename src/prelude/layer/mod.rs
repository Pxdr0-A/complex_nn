use crate::prelude::neuron::ActivationFunction;
use crate::prelude::neuron::Neuron;

#[derive(Debug)]
pub struct Layer {
    pub id: usize,
    pub units: Vec<Neuron>,
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

    pub fn signal(&self, input: &[f64]) -> Vec<f64> {

        // error handling will be at a higher level (in the Network)
        let mut output: Vec<f64> = Vec::with_capacity(self.units.len());
        
        if self.hidden {
            // hidden layer logic
            for neuron in &self.units[..] {
                output.push(neuron.signal(&input[..]));
            }
        } else {
            // input layer logic
            let mut init: usize = 0;
            let mut end: usize;
            for neuron in &self.units[..] {
                end = init + neuron.weights.len();
                
                output.push(neuron.signal(
                    &input[init..end]
                ));

                init = end;
            }        
        }

        output
    }

    pub fn next_id(&self) -> usize {

        self.id + 1
    }

    pub fn get_units(&self) -> usize {

        self.units.len()
    }
}