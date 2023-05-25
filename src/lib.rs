pub mod prelude;
pub mod math;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut n1 = prelude::neuron::Neuron::new(
            1,
            5,
            prelude::neuron::ActivationFunction::SIGMOID
        );

        let mut n2 = prelude::neuron::Neuron::new(
            2,
            7,
            prelude::neuron::ActivationFunction::TANH
        );

        println!("Neuron 1: {:?}", n1);
        println!("Neuron 2: {:?}", n2);

        n1.shuffle(12039485u128, 1.0);
        n2.shuffle(1039u128, 10.0);

        println!("Neuron 1: {:?}", n1);
        println!("Neuron 2: {:?}", n2);
    }
}
