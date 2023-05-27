pub mod prelude;
pub mod math;

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::math::Cf64;
    use super::*;

    #[test]
    fn it_works() {
        let scale: f64 = 1.0;
        let l = prelude::layer::Layer::new(
            1,
            2,
            false,
            prelude::neuron::ActivationFunction::SIGMOID,
            3,
            &scale
        );

        println!("{:?}", l);
    }
}