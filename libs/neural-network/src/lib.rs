extern crate nalgebra as na;
use na::DMatrix;
use rand::Rng;

pub use crate::activation_function::*;

pub mod activation_function;
mod tests;

pub struct NeuralNetwork {
    weights: Vec<na::DMatrix<f32>>,
    biases: Vec<na::DVector<f32>>,
    activation_function: ActivationFunction
}

impl NeuralNetwork {
    pub fn random(layers: &Vec<usize>, activation_function: ActivationFunction) -> Self {
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
            activation_function
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