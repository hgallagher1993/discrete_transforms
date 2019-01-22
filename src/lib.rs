extern crate num_traits;
extern crate stream_dct;

use std::f64;
use num_traits::float::Float;

fn dct(input: Vec<Vec<u8>>) -> Vec<f64> {
    let mut total_vec: Vec<f64> = Vec::new();
    let mut q;
    let mut s;
    let mut total;
    let pi = f64::consts::PI;

    for v in 0..8 {
        for u in 0..8 {
            let mut z = 0.0;

            for y in 0..8 {
                for x in 0..8 {
                    s = input[y][x] as f64;

                    q = s * (v as f64 * pi / 16.0 * (2.0 * y as f64 + 1.0)).cos()
                          * (u as f64 * pi / 16.0 * (2.0 * x as f64 + 1.0)).cos();

                    z += q;

                }
            }

            let cu = if u == 0 {
                1.0 / 2.0.sqrt()
            }
            else {
                1.0
            };

            let cv = if v == 0 {
                1.0 / 2.0.sqrt()
            }
            else {
                1.0
            };

            total = z * 0.25 * cu * cv;

            total_vec.push(total);
        }
    }

    total_vec
}

pub fn i_dct(input: Vec<f64>) -> Vec<u8> {
    let mut total_vec: Vec<u8> = Vec::new();
    let mut v_pi;
    let mut u_pi;
    let mut twice_y;
    let mut twice_x;

    for y in 0..8 {
        for x in 0..8 {
            let mut pixel_value: f64 = 0.0;
            twice_x = 2.0 * x as f64 + 1 as f64;
            twice_y = 2.0 * y as f64 + 1 as f64;

            for v in 0..8 {
                for u in 0..8 {
                    u_pi = u as f64 * f64::consts::PI / 16.0;
                    v_pi = v as f64 * f64::consts::PI / 16.0;

                    let cu = if u == 0 {
                        1.0 / 2.0.sqrt()
                    }
                    else {
                        1.0
                    };

                    let cv = if v == 0 {
                        1.0 / 2.0.sqrt()
                    }
                    else {
                        1.0
                    };

                    pixel_value += cu * cv * (v_pi * twice_y).cos() * (u_pi * twice_x).cos() * input[v * 8 + u];
                }
            }

            pixel_value = (pixel_value / 4.0 * 100.0).round() / 100.0;

            total_vec.push(pixel_value as u8);
        }
    }

    total_vec
}


#[test]
fn dct_forward() {
    let input = vec![vec![139, 144, 149, 153, 155, 155, 155, 155],
                     vec![144, 151, 153, 156, 159, 156, 156, 156],
                     vec![150, 155, 160, 163, 158, 156, 156, 156],
                     vec![159, 161, 162, 160, 160, 159, 159, 159],
                     vec![159, 160, 161, 162, 162, 155, 155, 155],
                     vec![161, 161, 161, 161, 160, 157, 157, 157],
                     vec![162, 162, 161, 163, 162, 157, 157, 157],
                     vec![162, 162, 161, 161, 163, 158, 158, 158]];

    let expected = vec![1259.62, -1.03, -12.08, -5.2, 2.13, -1.67, -2.71, 1.32,
                        -22.59, -17.48, -6.24, -3.16, -2.86, -0.07, 0.43, -1.19,
                        -10.95, -9.26,	-1.58, 1.53, 0.2, -0.94, -0.57,	-0.06,
                        -7.08, -1.91, 0.22, 1.45, 0.9,	-0.08, -0.04, 0.33,
                        -0.63, -0.84, 1.47, 1.56, -0.12, -0.66, 0.61, 1.28,
                        1.75, -0.2, 1.62, -0.34, -0.78, 1.48, 1.04, -0.99,
                        -1.28, -0.36, -0.32, -1.46, -0.49,	1.73, 1.08,	-0.76,
                        -2.6, 1.55, -3.76, -1.84, 1.87, 1.21, -0.57, -0.45];

    let forward = dct(input);

    let mut forward_rounded = Vec::new();

    for y in 0..64 {
        forward_rounded.push((forward[y] * 100.0).round() / 100.0);
    }

    assert_eq!(forward_rounded, expected);
}

#[test]
fn dct_inverse() {
    let input = vec![vec![139, 144, 149, 153, 155, 155, 155, 155],
                     vec![144, 151, 153, 156, 159, 156, 156, 156],
                     vec![150, 155, 160, 163, 158, 156, 156, 156],
                     vec![159, 161, 162, 160, 160, 159, 159, 159],
                     vec![159, 160, 161, 162, 162, 155, 155, 155],
                     vec![161, 161, 161, 161, 160, 157, 157, 157],
                     vec![162, 162, 161, 163, 162, 157, 157, 157],
                     vec![162, 162, 161, 161, 163, 158, 158, 158]];

    let expected = vec![139, 144, 149, 153, 155, 155, 155, 155,
                        144, 151, 153, 156, 159, 156, 156, 156,
                        150, 155, 160, 163, 158, 156, 156, 156,
                        159, 161, 162, 160, 160, 159, 159, 159,
                        159, 160, 161, 162, 162, 155, 155, 155,
                        161, 161, 161, 161, 160, 157, 157, 157,
                        162, 162, 161, 163, 162, 157, 157, 157,
                        162, 162, 161, 161, 163, 158, 158, 158];

    let forward = dct(input);

    let inverse = i_dct(forward);

    assert_eq!(inverse, expected)
}
