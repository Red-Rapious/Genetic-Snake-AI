extern crate nalgebra as na;
use na::DMatrix;
use rand::Rng;

use crate::activation_function::*;

pub mod activation_function;

pub struct NeuralNetwork {
    layers: Vec<usize>,
    weights: Vec<na::DMatrix<f32>>,
    biases: Vec<na::DVector<f32>>,
    activation_function: ActivationFunction
}

impl NeuralNetwork {
    fn random(layers: Vec<usize>, activation_function: ActivationFunction) -> Self {
        assert!(layers.len() > 1);

        let weights = layers
            .windows(2)
            .map(|dimensions| 
                DMatrix::from_fn(
                    dimensions[0],
                    dimensions[1],
                    |_, _| rand::thread_rng().gen()
                ))
            .collect();

        let biases = layers
                .iter()
                .map(|dimension| na::DVector::from_fn(
                    *dimension,
                    |_, _| rand::thread_rng().gen()
                ))
                .collect();

        Self {
            layers,
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
                |activation, (w, b)| self.activation_function.activation_function_vector(&mut (w * activation + b))
            )
    }
}