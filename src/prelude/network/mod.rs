use crate::prelude::layer::Layer;
use crate::prelude::neuron::ActivationFunction;

// considering fully connected network
#[derive(Debug)]
pub struct ConventionalNetwork {
    layers: Vec<Layer>,
    scale: f64
}
pub trait Network {
    fn new(
        n_units: usize,
        activation: ActivationFunction,
        input_length: usize,
        seed: u128,
        scale: &f64
    ) -> Self;

    fn add(
        &mut self,
        n_units: usize,
        activation: ActivationFunction,
        seed: u128
    );

    // fit function will call backpropagation and receive dataset
}

impl Network for ConventionalNetwork {
    // just the input layer
    fn new(
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

    fn add(
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
}