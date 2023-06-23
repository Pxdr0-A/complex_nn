pub mod prelude;
pub mod math;
mod utils;

#[cfg(test)]
mod tests {
    use crate::prelude::dataset::TabularDataset;
    use crate::prelude::network::ConventionalNetwork;
    use crate::prelude::neuron::ActivationFunction;

    use super::*;

    #[test]
    fn synthetic_dataset() {
        const DIM: usize = 10;
        const SHAPE: [usize; 2] = [5000, DIM];
        const CLASSES: usize = 20;
        const SEED: u128 = 187;

        let dataset: TabularDataset<String, f64, f64>;
        dataset = prelude::dataset::TabularDataset::sample(SHAPE, CLASSES, SEED);

        dataset.to_csv("./resources/dataset.csv");
    }

    #[test]
    fn load_dataset() {
        let file = "./resources/dataset.csv";
        let _dataset = TabularDataset::from_csv(file);
    }

    #[test]
    fn construct_network() {
        let mut network = ConventionalNetwork::new(
            8, 
            ActivationFunction::SIGMOID, 
            16, 
            2134, 
            &1.0
        );

        network.add(
            4, 
            ActivationFunction::TANH, 
            9857
        );

        network.add(
            2, 
            ActivationFunction::RELU, 
            347
        );

        network.close();

    }
}