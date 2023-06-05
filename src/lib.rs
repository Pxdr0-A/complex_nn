pub mod prelude;
pub mod math;
mod utils;

#[cfg(test)]
mod tests {
    use crate::prelude::dataset::TabularDataset;

    use super::*;

    #[test]
    fn synthetic_dataset() {
        const DIM: usize = 2;
        const SHAPE: [usize; 2] = [5000, DIM];
        const CLASSES: usize = 50;
        const SEED: u128 = 187;

        let dataset: TabularDataset<String, f64, f64>;
        dataset = prelude::dataset::TabularDataset::sample(SHAPE, CLASSES, SEED);

        let (x, y, target) = utils::draw::arrange_points(&dataset, &SHAPE);
        utils::draw::scatter_template(x, y, target);
    }
}