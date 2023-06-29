mod error_handling;

use crate::prelude::layer::Layer;
use crate::prelude::neuron::ActivationFunction;

#[derive(Debug)]
pub struct ConventionalNetwork {
    pub layers: Vec<Layer>,
    scale: f64
}

// considering fully connected network
impl ConventionalNetwork {
    // just the input layer
    pub fn new(
        n_units: usize,
        activation: ActivationFunction,
        input_length: usize,
        seed: u128,
        scale: &f64
    ) -> ConventionalNetwork {
        let mut layers: Vec<Layer> = Vec::new();
        layers.push(
            Layer::new(
                1,
                n_units,
                false,
                activation,
                error_handling::input_test(input_length, n_units)
                    .expect("Input must be a multiple different from 1 of the number o units."),
                seed,
                &scale
            )
        );

        ConventionalNetwork { layers, scale: *scale }
    }

    pub fn add(
        &mut self,
        n_units: usize,
        activation: ActivationFunction,
        seed: u128
    ) {
        
        let last_layer = self.layers
            .last()
            .expect("Network must be initialized first.");
        self.layers.push(
            Layer::new(
                last_layer.next_id(),
                n_units,
                true,
                activation,
                last_layer.get_units(),
                seed,
                &self.scale
            )
        );
    }

    pub fn foward(&self, input: &[f64]) -> Vec<f64> {
        let input_layer = &self.layers[0];
        // build a macro with this
        let _neuron_input_len = error_handling::input_test(
            input.len(), input_layer.units.len()
        ).expect("Input must be a multiple different from 1 of the number o units.");

        let mut output: Vec<f64> = Vec::from(input);
        for layer in &self.layers[..] {
            output = layer.signal(&output);
        }

        output
    }
    
}