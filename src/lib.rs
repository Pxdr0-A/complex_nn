pub mod prelude;
pub mod math;

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::math::{Cf64, Matrix};
    use crate::prelude::network::{ConventionalNetwork, Network};
    use crate::prelude::neuron::ActivationFunction;
    use super::*;

    #[test]
    fn it_works() {
        let mut m: Matrix<i32> = math::Matrix::new([3, 3]);

        m.add_row(&mut vec![1; 3]);
        println!("{:?}", m);
        m.add_row(&mut vec![4; 3]);
        println!("{:?}", m);

        let mut l: Matrix<f64> = math::Matrix::new([2, 2]);

        l.add_row(&mut vec![1.4, 2.34]);
        println!("{:?}", l);
        l.add_row(&mut vec![3.225, 4.21]);
        println!("{:?}", l);

        println!("{:?}", l.elm(&1, &0));
    }
}