#[cfg(test)]
use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    mod neural_network {
        use super::*;

        mod initialisation {
            use super::*;

            #[test]
            fn test() {
                let _ = NeuralNetwork::random();
            }
        }

        mod feed_forward {
            use super::*;

            #[test]
            fn sigmoid() {
                let mut nn = NeuralNetwork::random();
                nn.activation_function = ActivationFunction::sigmoid();
                let input = na::DVector::from([1.0; 8*3].to_vec());

                nn.feed_forward(input);
            }

            #[test]
            fn relu() {
                let mut nn = NeuralNetwork::random();
                nn.activation_function = ActivationFunction::ReLU();
                let input = na::DVector::from([1.0; 8*3].to_vec());

                nn.feed_forward(input);
            }
        }

        mod genome {
            use super::*;
    
            #[test]
            fn to_and_from_genome() {
                let nn = NeuralNetwork::random();
    
                let genome = nn.to_genome();
                let nn_bis = NeuralNetwork::from_genome(&genome);
    
                assert_eq!(nn, nn_bis);
            }
        }
    }
}