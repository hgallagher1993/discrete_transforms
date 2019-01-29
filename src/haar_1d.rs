use std::f64;

pub fn forward(input: &Vec<f64>) -> Vec<f64> {
    let mut output: Vec<f64> = input.clone();
    let mut output_temp = vec![0.0; 8];

    let mut length = input.len() / 2;

    for _ in 0..3 {
        for k in 0..length {
            let average = (output[k * 2] + output[k * 2 + 1]) / 2.0;
            let difference = output[k * 2] - average;

            output_temp[k] = average;
            output_temp[k + length] = difference;
        }

        for x in 0..8 {
            output[x] = output_temp[x];
        }

        length /= 2;
    }

    output
}

pub fn inverse(input: &Vec<f64>) -> Vec<f64> {
    let mut output = input.clone();
    let mut length = 2;

    for _ in 0..3 {
        let mut output_temp = Vec::new();
        let mut temp: Vec<f64> = output.iter().take(length).cloned().collect();

        for k in 0..length / 2 {
            let (mut s, mut d) =  temp.split_at(length / 2);

            let sk = s[k] + d[k];
            let dk = s[k] - d[k];

            output_temp.push(sk);
            output_temp.push(dk);
        }

        for y in 0..output_temp.len() {
            output[y] = output_temp[y];
        }

        length *= 2;
    }

    output
}

#[test]
fn haar_1d_forward_test() {
    let input = vec![6.0, 12.0, 15.0, 15.0, 14.0, 12.0, 120.0, 116.0];
    let expected = vec![38.75, -26.75, -3.0, -52.5, -3.0, 0.0, 1.0, 2.0];

    let forward = forward(&input);

    assert_eq!(forward, expected);
}

#[test]
fn haar_1d_inverse_test() {
    let input = vec![38.75, -26.75, -3.0, -52.5, -3.0, 0.0, 1.0, 2.0];
    let expected = vec![6.0, 12.0, 15.0, 15.0, 14.0, 12.0, 120.0, 116.0];

    let inverse = inverse(&input);

    assert_eq!(expected, inverse);
}
