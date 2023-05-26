pub mod prelude;
pub mod math;

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::math::Cf64;
    use super::*;

    #[test]
    fn it_works() {
        let mut n1 = prelude::neuron::Neuron::new(
            1,
            2,
            prelude::neuron::ActivationFunction::SIGMOID
        );

        let mut n2 = prelude::neuron::Neuron::new(
            2,
            2,
            prelude::neuron::ActivationFunction::TANH
        );

        let mut n3 = prelude::neuron::Neuron::new(
            3,
            2,
            prelude::neuron::ActivationFunction::RELU
        );
        println!("Neuron 1: {:?}", n1);
        println!("Neuron 2: {:?}", n2);
        println!("Neuron 3: {:?}", n3);

        n1.shuffle(457869485u128, &1.0);
        n2.shuffle(903923847u128, &1.0);
        n3.shuffle(559872342u128, &1.0);
        println!("Neuron 1: {:?}", n1);
        println!("Neuron 2: {:?}", n2);
        println!("Neuron 3: {:?}", n3);

        let output1 = n1.signal(&[0.1, 0.2]);
        let output2 = n2.signal(&[0.34, 1.977]);
        let output3 = n3.signal(&[-0.84, 1.577]);
        println!("Neuron 1 output {:?}", output1);
        println!("Neuron 2 output {:?}", output2);
        println!("Neuron 3 output {:?}", output3);
    }
}