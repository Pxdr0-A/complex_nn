use std::fs;
use std::collections::HashMap;

use crate::math::Matrix;

// dataset for classification problems
#[derive(Debug)]
pub struct TabularDataset<H, B, T> {
    header: Vec<H>,
    body: Matrix<B>,
    target: Vec<T>,
    file: Option<String>,
    batch: usize
}

/* Future datasets to consider:

- RegressionDataset
- SequenceDataset (can also support text)
- TextDataset (corpus)

Consider creating a trait Dataset.

*/

impl<H, B, T> TabularDataset<H, B, T> {
    pub fn new(capacity: [usize; 2]) -> TabularDataset<H, B, T> {

        let header: Vec<H> = Vec::with_capacity(capacity[1]);
        let body: Matrix<B> = Matrix::new(capacity);
        let target: Vec<T> = Vec::with_capacity(capacity[0]);
        let file = None;
        let batch: usize = 0;

        TabularDataset {
            header,
            body,
            target,
            file,
            batch
        }
    }

    pub fn from_csv(file: String, batch_size: usize) -> std::io::Result<()>  {

        let file1 = fs::read_to_string("./resources/dataset.csv")?;

        println!("{:?}", file1);

        Ok(())
    }

    pub fn add_row(&mut self, row: &mut Vec<B>, target_val: T) {

        self.body.add_row(row);
        self.target.push(target_val);
    }

    pub fn row(&self, i: &usize) -> (&[B], &T) {

        (self.body.row(i), &self.target[*i])
    }
}

impl TabularDataset<String, f64, f64> {
    pub fn sample(
        shape: [usize; 2], 
        n_classes: usize, 
        seed: u128
    ) -> TabularDataset<String, f64, f64> {

        let mut dataset = TabularDataset::new(shape);
        for u in 0..shape[1] {
            dataset.header.push(format!("feature {}", u).to_string());
        }
        dataset.header.push("target".to_string());

        // build cluster centers
        let mut centers: HashMap<String, Vec<f64>> = HashMap::new();

        crate::utils::misc::build_random_centers(&mut centers, &shape, n_classes, seed + 2348);
        crate::utils::misc::add_random_points(&mut dataset, &centers, &shape, n_classes, seed);

        dataset
    }
}
