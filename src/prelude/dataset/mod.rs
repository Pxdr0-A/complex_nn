/*

 This module provides basic support for contructing a dataset.
 It can then be used for upcoming neural network input.

 In here I was not extra thoughtfull on efficiency and memory safety.

 */

use std::collections::HashMap;

use crate::math::Matrix;

// dataset for classification problems
#[derive(Debug)]
pub struct TabularDataset<H, B, T> {
    header: Vec<H>,
    body: Matrix<B>,
    target: Vec<T>,
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

        TabularDataset {
            header,
            body,
            target
        }
    }

    pub fn add_row(&mut self, row: &mut Vec<B>, target_val: T) {

        self.body.add_row(row);
        self.target.push(target_val);
    }

    pub fn row(&self, i: &usize) -> (&[B], &T) {

        (self.body.row(i), &self.target[*i])
    }
}

// specific implementation for a synthetic clustering dataset
// this is going to be the most common configuration
// and maybe is it going to be the accepted configuration
// head - string
// body - f64
// target - f64
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

    pub fn to_csv(&self, path: &str) {
        
        let mut contents: String = "".to_string();
        
        let mut formated_header: Vec<&str> = Vec::new();
        for s in self.header.iter() {
            formated_header.push(s.as_str());
        }

        contents.push_str(formated_header.join(",").as_str());
        contents.push_str("\n");

        let mut formated_bulk: Vec<String> = Vec::new();
        let mut current_row: (&[f64], &f64);
        for i in 0..self.body.shape[0] {
            current_row = self.row(&i);
            for f in current_row.0 {
                // add body of the row to content Vec (values of the features)
                formated_bulk.push(f.to_string());
            }

            // add target of the row to content Vec
            formated_bulk.push(current_row.1.to_string());

            // add auxiliar content Vec to contents to create a csv row
            contents.push_str(formated_bulk.join(",").as_str());
            contents.push_str("\n");

            // clear auxiliar content Vec
            formated_bulk.clear();
        }

        crate::utils::file_ops::write_csv(path, contents)
            .expect("Failed to export TabularDataset to csv format.");

    }

    pub fn from_csv(file: &str) -> TabularDataset<String, f64, f64>{
        
        let content = crate::utils::file_ops::read_csv(file).expect(
            "Failed to read existing csv file."
        );

        let mut rows_split = content.split("\n");
        let mut current_row;

        // H = String
        let header: Vec<String> = rows_split
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.to_string())
            .collect();

        let n_rows = rows_split.clone().count();
        let mut body: Matrix<f64> = Matrix::new([n_rows, header.len() - 1]);
        let mut target: Vec<f64> = Vec::with_capacity(n_rows - 1);
        // parse them to f64
        for _ in 0..n_rows-1  {
            current_row = rows_split.next();
            
            target.push(
                current_row
                    .unwrap()
                    .split(",")
                    .map(|x| x.parse::<f64>().expect("Invalid datatype in dataset."))
                    .last()
                    .unwrap()
            );
            body.add_row(
                &mut current_row
                    .unwrap()
                    .split(",")
                    .map(|x| x.parse::<f64>().expect("Invalid datatype in dataset."))
                    .take(header.len() - 1)
                    .collect()
            );
        }

        TabularDataset {
            header,
            body,
            target
        }
    }
}
