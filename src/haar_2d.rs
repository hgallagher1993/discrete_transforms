use std::f64;
use itertools::Itertools;
use haar_1d;

fn forward(input: &Vec<f64>) -> Vec<f64> {
    let mut output: Vec<f64> = Vec::new();
    let mut output_temp: Vec<f64> = Vec::new();
    let mut forward_input = Vec::new();

    for chunks in &input.iter().chunks(8) {
        forward_input = haar_1d::forward(&chunks.map(|x| *x).collect_vec());

        output.extend(forward_input.iter().cloned());
    }

    forward_input.clear();

    for x in 0..8 {
        for y in 0..8 {
            forward_input.push(output[x + 8 * y]);
        }

        output_temp.extend(haar_1d::forward(&forward_input).iter().cloned());

        forward_input.clear();
    }

    for x in 0..8 {
        for y in 0..8 {
            output[x + 8 * y] = output_temp[x * 8 + y];
        }
    }

    output
}

#[test]
fn haar_2d_forward_test() {
    let input = vec![139.0, 144.0, 149.0, 153.0, 155.0, 155.0, 155.0, 155.0,
                     144.0, 151.0, 153.0, 156.0, 159.0, 156.0, 156.0, 156.0,
                     150.0, 155.0, 160.0, 163.0, 158.0, 156.0, 156.0, 156.0,
                     159.0, 161.0, 162.0, 160.0, 160.0, 159.0, 159.0, 159.0,
                     159.0, 160.0, 161.0, 162.0, 162.0, 155.0, 155.0, 155.0,
                     161.0, 161.0, 161.0, 161.0, 160.0, 157.0, 157.0, 157.0,
                     162.0, 162.0, 161.0, 163.0, 162.0, 157.0, 157.0, 157.0,
                     162.0, 162.0, 161.0, 161.0, 163.0, 158.0, 158.0, 158.0];

    let expected = vec![157.453125,  0.015625, -1.71875,  0.8125, -1.25,  -0.6875,  1.625,  0.0,
                         -2.171875, -1.609375, -1.59375, -0.4375, -1.125, -0.3125,  -0.875, 0.0,
                         -3.03125,	-2.03125,  -0.8125,	  0.0,	  -0.625, -0.75,	0.0,    0.0,
                         -0.625,     0.125,	   -0.375,	  0.0,	  -0.125,  0.125,	0.0,    0.0,
                         -1.625,	-0.75,     -0.625,   -0.375,   0.5,	  -0.25,   -0.75,   0.0,
                         -1.5625,	-0.1875,   -2.0,	  0.125,  -0.75,  -1.25,	0.25,   0.0,
                         -0.375,	 0.125,	   -0.5,	  0.5,	  -0.25,  -0.25,	1.0,    0.0,
                         -0.125,     0.375,	   -0.25,	  0.0,	   0.0,	  -0.5,		0.0,    0.0];

    let forward = forward(&input);

    assert_eq!(forward, expected);
}
