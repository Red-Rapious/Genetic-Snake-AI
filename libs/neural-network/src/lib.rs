extern crate nalgebra as na;
use na::DMatrix;
use rand::Rng;

pub use crate::activation_function::*;

pub mod activation_function;
mod tests;

const ACTIVATION_FUNCTION: ActivationFunctionType = ActivationFunctionType::Sigmoid;


#[derive(Debug, PartialEq)]
pub struct NeuralNetwork {
    weights: Vec<na::DMatrix<f32>>,
    biases: Vec<na::DVector<f32>>,
    activation_function: ActivationFunction
}

impl NeuralNetwork {
    pub fn random(layers: &Vec<usize>) -> Self {
        assert!(layers.len() > 1);

        let weights = layers
            .windows(2)
            .map(|shape| 
                DMatrix::from_fn(
                    shape[1],
                    shape[0],
                    |_, _| rand::thread_rng().gen()
                ))
            .collect();

        let biases = layers
                .windows(2)
                .map(|shape| na::DVector::from_fn(
                    shape[1],
                    |_, _| rand::thread_rng().gen()
                ))
                .collect();

        Self {
            weights,
            biases,
            activation_function: ActivationFunction::new(ACTIVATION_FUNCTION)
        }
    }

    pub fn feed_forward(&self, input: na::DVector<f32>) -> na::DVector<f32> {
        self.weights
            .iter()
            .zip(&self.biases)
            .fold(input, 
                |activation, (weight, bias)| 
                { 
                    dbg!(activation.len());
                    dbg!(weight.shape());
                    dbg!(bias.shape());
                    self.activation_function.activation_function_vector(&mut (weight * activation + bias))
                }
            )
    }

    /// Initializes a layer from given weights.
    pub fn from_weights(weights: &Vec<f32>) -> Self {
        todo!()
    }

    pub fn to_weights(&self) -> Vec<f32> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod weights {
        use super::*;

        #[test]
        fn to_and_from_weights() {
            let nn = NeuralNetwork::random(&vec![3, 4, 5]);

            let weights = nn.to_weights();
            let nn_bis = NeuralNetwork::from_weights(&weights);

            assert_eq!(nn, nn_bis);
        }
    }
}