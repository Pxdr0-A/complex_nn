pub mod prelude;
pub mod math;
mod utils;

#[cfg(test)]
mod tests {
    use crate::prelude::dataset::TabularDataset;

    use super::*;

    #[test]
    fn it_works() {
        println!("Started focus testing.");

        let dataset: TabularDataset<String, f64, f64>;
        dataset = prelude::dataset::TabularDataset::sample([500, 2], 10, 11327);

        let (x, y, target) = utils::draw::arrange_points(&dataset, &[500, 2]);
        utils::draw::scatter_template(x, y, target);
    }
}