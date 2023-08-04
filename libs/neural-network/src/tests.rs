#[cfg(test)]
use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    mod neural_network {
        use super::*;

        mod initialisation {
            use super::*;

            /*fn test(layers: Vec<usize>) {
                let nn = NeuralNetwork::random(&layers);

                for i in 0..layers.len()-2 {
                    assert_eq!(nn.weights[i].shape(), (layers[i+1], layers[i]));
                }
            }

            #[test]
            fn small() {
                test(vec![1, 2, 3, 4]);
            }

            #[test]
            fn medium() {
                test(vec![10, 20, 30, 40]);
            }

            #[test]
            fn big() {
                test(vec![10, 20, 30, 40, 30, 20, 10, 20, 30, 40]);
            }

            #[test]
            fn huge() {
                test((0..100).collect());
            }*/

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

        mod weights {
            use super::*;
    
            #[test]
            fn to_and_from_weights() {
                let nn = NeuralNetwork::random();
    
                let weights = nn.to_weights();
                let nn_bis = NeuralNetwork::from_weights(&weights);
    
                assert_eq!(nn, nn_bis);
            }
        }
    }
}