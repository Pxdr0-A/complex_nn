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
        let mut m: Matrix<i32> = Matrix::new([4, 4]);

        m.add_row(&mut vec![1; 3]);
        println!("{:?}", m);
        m.add_row(&mut vec![4; 3]);
        println!("{:?}", m);
        m.add_row(&mut vec![5; 3]);
        println!("{:?}", m);

        println!("{:?}", m.elm(&1, &2));

        let mut l: Matrix<f64> = Matrix::new([3, 3]);

        l.add_row(&mut vec![1.4, 2.34]);
        println!("{:?}", l);
        l.add_row(&mut vec![3.22, 4.21]);
        println!("{:?}", l);

        println!("{:?}", l.elm(&1, &0));

        println!("{:?}", m.row(&2));
        println!("{:?}", l.row(&1));
        println!("{:?}", m.column(&0));
        println!("{:?}", l.column(&0));

        m.add_col(&mut vec![1, 3, 8]);
        l.add_col(&mut vec![1.4, 6.45]);
        println!("{:?}", m);
        println!("{:?}", l);
    }
}