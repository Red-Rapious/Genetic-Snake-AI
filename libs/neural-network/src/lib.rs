extern crate nalgebra as na;
use na::DMatrix;
use rand_distr::{Normal, Distribution};

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
    /// Initilises a neural network with random weights and biases. 
    /// Each weight and bias' follows a normal distribution.
    pub fn random() -> Self {
        assert!(LAYERS.len() > 1);
        // Normal distribution sampler
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut rng = rand::thread_rng();

        // Initialise random weights matrices with proper dimensions
        let weights = LAYERS
            .windows(2)
            .map(|shape| 
                DMatrix::from_fn(
                    shape[1],
                    shape[0],
                    |_, _| normal.sample(&mut rng)
                ))
            .collect();

        // Initilise random biases vectors with proper width
        let biases = LAYERS
                .windows(2)
                .map(|shape| na::DVector::from_fn(
                    shape[1],
                    |_, _| normal.sample(&mut rng)
                ))
                .collect();

        Self {
            weights,
            biases,
            activation_function: ActivationFunction::new(ACTIVATION_FUNCTION)
        }
    }

    /// Given an input vector of proper dimension, computes the output of the neural network.
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

    /// Converts a full neural network to a genome of weights and biases.
    pub fn to_genome(&self) -> Vec<f32> {
        let mut genome = Vec::with_capacity(868);

        // Add weights to the genome
        for matrix in self.weights.iter() {
            for &coeff in matrix.iter() {
                genome.push(coeff);
            }
        }

        // Add biases to the genome
        for vector in self.biases.iter() {
            for &coeff in vector.iter() {
                genome.push(coeff);
            }
        }

        genome
    }

    /// Initializes a layer from given genome.
    pub fn from_genome(genome: &Vec<f32>) -> Self {
        let mut weights_vect = vec![];
        let mut biases_vect = vec![];
        let mut cursor = 0;

        for shape in LAYERS.windows(2) {
            let mut matrix = Vec::with_capacity(shape[0]*shape[1]);
            for _ in 0..shape[0]*shape[1] {
                matrix.push(genome[cursor]);
                cursor += 1;
            }

            weights_vect.push(na::DMatrix::from_vec(shape[1], shape[0], matrix));
        }

        for &width in LAYERS.iter().skip(1) {
            let mut vector = Vec::with_capacity(width);
            for _ in 0..width {
                vector.push(genome[cursor]);
                cursor += 1;
            }
            biases_vect.push(na::DVector::from_vec(vector));
        }

        assert!(genome.len() == cursor);

        Self { 
            weights: weights_vect, 
            biases: biases_vect, 
            activation_function: ActivationFunction::new(ACTIVATION_FUNCTION)
        }
    }
}