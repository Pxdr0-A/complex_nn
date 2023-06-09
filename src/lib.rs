pub mod prelude;
pub mod math;
mod utils;

#[cfg(test)]
mod tests {
    use crate::prelude::dataset::TabularDataset;

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
}