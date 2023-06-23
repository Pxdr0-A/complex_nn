use crate::prelude::layer::Layer;
use crate::prelude::neuron::ActivationFunction;

#[derive(Debug)]
pub struct ConventionalNetwork {
    layers: Vec<Layer>,
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
                if (input_length / n_units) * n_units == input_length {
                    if input_length / n_units >= 2 {
                        input_length / n_units
                    } else {
                        panic!("Single input Neurons will bypass complex properties.");
                    }
                } else {
                    panic!("The number of units must be a multiple of the input length.");
                },
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

    pub fn close(&mut self) {
        let last_layer = self.layers
            .last_mut()
            .expect("Network must be initialized first.");

        last_layer.switch_hidden()
    }
    
}