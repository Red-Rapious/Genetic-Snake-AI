use crate::*;
use std::fmt;

/// An enumeration of all available activation functions, like `Sigmoid` or `ReLU`.
#[derive(Debug, PartialEq)]
pub enum ActivationFunctionType {
    Sigmoid,
    ReLU
}

impl fmt::Display for ActivationFunctionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            ActivationFunctionType::ReLU => "ReLU (Rectification function)",
            ActivationFunctionType::Sigmoid => "Sigmoid (Logistic function)"
        })
    }
}

/// A structure providing the good activation function depending on the given type.
#[derive(Debug, PartialEq)]
pub struct ActivationFunction {
    /// The activation function, which will dictate the main function and its derivative.
    function_type: ActivationFunctionType
}

impl ActivationFunction {
    /// Create a new ActivationFunction of given type.
    pub fn new(function_type: ActivationFunctionType) -> Self {
        ActivationFunction { function_type }
    }

    /// Shortcut for sigmoid
    pub fn sigmoid() -> Self {
        ActivationFunction::new(ActivationFunctionType::Sigmoid)
    }

    /// Shortcut for ReLU
    #[allow(non_snake_case)]
    pub fn ReLU() -> Self {
        ActivationFunction::new(ActivationFunctionType::Sigmoid)
    }

    pub fn activation_function_vector(&self, input: &na::DVector<f32>) -> na::DVector<f32> {
        na::DVector::from_iterator(
            input.len(),
            input.iter().map(|x| self.activation_function(*x))
        )
    }

    /// The main activation function, depends on `function_type`.
    pub fn activation_function(&self, x: f32) -> f32 {
        match self.function_type {
            ActivationFunctionType::Sigmoid => sigmoid(x),
            ActivationFunctionType::ReLU => ReLU(x)
        }
    }

    /// The derivative of the activation function, depends on `function_type`.
    pub fn activation_prime(&self, x: f32) -> f32 {
        match self.function_type {
            ActivationFunctionType::Sigmoid => sigmoid_prime(x),
            ActivationFunctionType::ReLU => ReLU_prime(x)
        }
    }
}

/// Logistic function.
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/// Derivative of sigmoid.
fn sigmoid_prime(x: f32) -> f32 {
    let s = sigmoid(x);
    s * (1.0 - s)
}

/// Rectification function.
#[allow(non_snake_case)]
fn ReLU(x: f32) -> f32 {
    f32::max(x, 0.0)
}

/// Derivative of ReLU
#[allow(non_snake_case)]
fn ReLU_prime(x: f32) -> f32 {
    if x >= 0.0 {
        1.0
    } else {
        0.0
    }
}