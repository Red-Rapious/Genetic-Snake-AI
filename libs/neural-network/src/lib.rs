extern crate nalgebra as na;
use na::DMatrix;
use rand::Rng;

use crate::activation_function::*;

pub mod activation_function;
mod tests;

pub struct NeuralNetwork {
    weights: Vec<na::DMatrix<f32>>,
    biases: Vec<na::DVector<f32>>,
    activation_function: ActivationFunction
}

impl NeuralNetwork {
    fn random(layers: &Vec<usize>, activation_function: ActivationFunction) -> Self {
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
                |activation, (w, b)| 
                { 
                    dbg!(activation.len());
                    dbg!(w.shape());
                    dbg!(b.shape());
                    self.activation_function.activation_function_vector(&mut (w * activation + b))
                }
            )
    }
}