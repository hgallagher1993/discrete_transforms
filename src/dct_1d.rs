use std::f64;
use num_traits::float::Float;
use util::TransformDirections;

#[derive(Debug)]
pub struct Dct1D {
    input: Vec<f64>
}

impl Dct1D {
    pub fn new() -> Dct1D {
        Dct1D {
            input: vec![]
        }
    }

    pub fn set_input(&mut self, input: Vec<f64>) -> &mut Self {
        self.input = input;

        self
    }
}

impl TransformDirections for Dct1D {
    fn forward(&self) -> Vec<f64> {
        let pi = f64::consts::PI;
        let mut output: Vec<f64> = Vec::new();

        for u in 0..8 {
            let mut coefficient: f64 = 0.0;

            for x in 0..8 {
                coefficient += self.input[x] * (u as f64 * pi / 16.0 * (2.0 * x as f64 + 1.0)).cos();
            }

            let cu = if u == 0 {
                1.0 / 2.0.sqrt()
            }
            else {
                1.0
            };

            coefficient = coefficient / 2.0 * cu;

            output.push(coefficient);
        }

        output
    }

    fn inverse(&self) -> Vec<f64> {
        let pi = f64::consts::PI;
        let mut output: Vec<f64> = Vec::new();

        for x in 0..8 {
            let mut output_temp: f64 = 0.0;

            for u in 0..8 {
                let cu = if u == 0 {
                    1.0 / 2.0.sqrt()
                }
                else {
                    1.0
                };

                output_temp += (u as f64 * pi / 16.0 * (2.0 * x as f64 + 1.0)).cos() * self.input[u] * cu;
            }

            output_temp /= 2.0;

            output.push(output_temp);
        }

        output
    }
}