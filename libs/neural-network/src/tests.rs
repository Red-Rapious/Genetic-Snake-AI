#[cfg(test)]
use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    mod neural_network {
        use super::*;

        mod initialisation {
            use super::*;

            fn test(layers: Vec<usize>) {
                let nn = NeuralNetwork::random(&layers, ActivationFunction::sigmoid());

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
            }
        }

        mod feed_forward {
            use super::*;

            #[test]
            fn sigmoid() {
                let nn = NeuralNetwork::random(&vec![3, 4, 5, 6], ActivationFunction::sigmoid());
                let input = na::DVector::from(vec![42.0, 21.0, 100.0]);

                nn.feed_forward(input);
            }

            #[test]
            fn relu() {
                let nn = NeuralNetwork::random(&vec![3, 4, 5, 6], ActivationFunction::ReLU());
                let input = na::DVector::from(vec![42.0, 21.0, 100.0]);

                nn.feed_forward(input);
            }
        }
    }
}