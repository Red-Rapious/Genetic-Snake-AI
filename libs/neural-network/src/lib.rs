extern crate nalgebra as na;
use na::DMatrix;
use rand::Rng;

pub use crate::activation_function::*;

pub mod activation_function;
mod tests;

const ACTIVATION_FUNCTION: ActivationFunctionType = ActivationFunctionType::Sigmoid;
const LAYERS: &[usize; 4] = &[8*3, 18, 18, 4];

#[derive(Debug, PartialEq)]
pub struct NeuralNetwork {
    weights: Vec<na::DMatrix<f32>>,
    biases: Vec<na::DVector<f32>>,
    activation_function: ActivationFunction
}

impl NeuralNetwork {
    pub fn random() -> Self {
        assert!(LAYERS.len() > 1);

        let weights = LAYERS
            .windows(2)
            .map(|shape| 
                DMatrix::from_fn(
                    shape[1],
                    shape[0],
                    |_, _| rand::thread_rng().gen()
                ))
            .collect();

        let biases = LAYERS
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
                    self.activation_function.activation_function_vector(&mut (weight * activation + bias))
                }
            )
    }

    /// Converts a full neural network to a vector of weights and biases.
    pub fn to_weights(&self) -> Vec<f32> {
        let mut weights = vec![]; // TODO change capacity

        /*let _ = self.weights
            .iter()
            .map(|matrix| {
                let _ = matrix
                    .iter()
                    .map(|&coeff| weights.push(coeff));
            });*/

        for matrix in self.weights.iter() {
            for &coeff in matrix.iter() {
                weights.push(coeff);
            }
        }

        /*let _ = self.biases
            .iter()
            .map(|vect| {
                let _ = vect
                    .iter()
                    .map(|&coeff| weights.push(coeff));
            });*/

        for vector in self.biases.iter() {
            for &coeff in vector.iter() {
                weights.push(coeff);
            }
        }

        weights
    }

    /// Initializes a layer from given weights.
    pub fn from_weights(weights: &Vec<f32>) -> Self {
        let mut weights_vect = vec![];
        let mut biases_vect = vec![];
        let mut cursor = 0;

        for shape in LAYERS.windows(2) {
            let mut matrix = Vec::with_capacity(shape[0]*shape[1]);
            for _ in 0..shape[0]*shape[1] {
                matrix.push(weights[cursor]);
                cursor += 1;
            }

            weights_vect.push(na::DMatrix::from_vec(shape[1], shape[0], matrix));
        }

        for &width in LAYERS.iter().skip(1) {
            let mut vector = Vec::with_capacity(width);
            for _ in 0..width {
                vector.push(weights[cursor]);
                cursor += 1;
            }
            biases_vect.push(na::DVector::from_vec(vector));
        }

        assert!(weights.len() == cursor);

        Self { 
            weights: weights_vect, 
            biases: biases_vect, 
            activation_function: ActivationFunction::new(ACTIVATION_FUNCTION)
        }
    }
}