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

        coefficient = coefficient * 0.25 * cu;

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

        output.push(output_temp.round());
    }

    output
}

#[test]
fn dct_1d_forward_test() {
    let input = vec![139.0, 144.0, 149.0, 153.0, 155.0, 155.0, 155.0, 155.0];
    let expected = vec![213.01591783244743, -7.140583065982376,
                        -3.711932654245892, -1.0404170265511574,
                        -0.1767766952966312, -0.2334917001449277,
                        -0.1845425976387478, -0.009354925512826462];

    let forward = forward(&input);

    assert_eq!(forward, expected);
}

#[test]
fn dct_1d_inverse_test() {
    let input = vec![213.01591783244743, -7.140583065982376,
                        -3.711932654245892, -1.0404170265511574,
                        -0.1767766952966312, -0.2334917001449277,
                        -0.1845425976387478, -0.009354925512826462];
    let expected = vec![139.0, 144.0, 149.0, 153.0, 155.0, 155.0, 155.0, 155.0];

    let inverse = inverse(&input);

    assert_eq!(inverse, expected);
}