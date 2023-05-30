/* Contents of prelude module

- user is able to design his own network
--- choose complex method
--- build his arch: conventional or sequence based networks

- user is able to train, test and validate his own network
--- provided a dataset struct (can be pre-preprocessed from an external file or simulated)
------ tabular dataset (numeric)
------ image dataset
------ text dataset
--- provided a .csv file

- user is able to evaluate and fine tune his own network

- ambition: user is able to obtain explanations for its results

 */

pub mod neuron;
pub mod layer;
pub mod network;

pub mod dataset;
