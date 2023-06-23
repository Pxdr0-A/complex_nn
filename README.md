# Complex Neural Networks
A library for complex neural network alternatives built with Rust


# Details on usage
- This library does not preprocess data. TabularDataset struct is provide as an interface.
    - Header: String
    - Body: f64
    - Target: f64
- Precision can not be manipulated. Data feed into the network is always f64.