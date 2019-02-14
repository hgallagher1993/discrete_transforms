use std::f64;
use num_traits::float::Float;

pub fn forward(input: &Vec<f64>) -> Vec<f64> {
    let pi = f64::consts::PI;
    let mut output = Vec::new();

    for u in 0..8 {
        let mut coefficient = 0.0;

        for x in 0..8 {
            coefficient += input[x] * (u as f64 * pi / 16.0 * (2.0 * x as f64 + 1.0)).cos();
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

pub fn inverse(input: &Vec<f64>) -> Vec<f64> {
    let pi = f64::consts::PI;
    let mut output = Vec::new();

    for x in 0..8 {
        let mut output_temp = 0.0;

        for u in 0..8 {
            let cu = if u == 0 {
                1.0 / 2.0.sqrt()
            }
            else {
                1.0
            };

            output_temp += (u as f64 * pi / 16.0 * (2.0 * x as f64 + 1.0)).cos() * input[u] * cu;
        }

        output_temp /= 2.0;

        output.push(output_temp);
    }

    output
}