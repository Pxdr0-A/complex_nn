pub mod prelude;
pub mod math;

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::math::Cf64;
    use crate::prelude::network::{ConventionalNetwork, Network};
    use crate::prelude::neuron::ActivationFunction;
    use super::*;

    #[test]
    fn it_works() {
        let scale: f64 = 1.0;
        let mut net: ConventionalNetwork = prelude::network::Network::new(
            2,
            ActivationFunction::SIGMOID,
            4,
            38983,
            &scale
        );

        println!("{:?}", net);

        net.add(
            6,
            ActivationFunction::TANH,
            53898
        );

        println!("{:?}", net);
    }
}