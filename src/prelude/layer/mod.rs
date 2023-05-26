use super::neuron::Neuron;

pub struct Layer {
    id: usize,
    units: Vec<Neuron>,
    hidden:bool
}