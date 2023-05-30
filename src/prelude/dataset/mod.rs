use crate::math::Matrix;

pub struct TabularDataset<T, C> {
    body: Matrix<T>,
    target: Vec<C>,
    file: Option<String>
}
// batch size will be included in fit function

