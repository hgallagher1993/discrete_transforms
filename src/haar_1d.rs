use std::f64;

pub fn forward(input: &Vec<u8>) -> Vec<f64> {
    let mut output: Vec<f64> = input.iter().map(|x| *x as f64).collect();
    let mut output_temp = vec![0.0; 8];

    let mut length = input.len() / 2;

    for _ in 0..3 {
        for k in 0..length {
            let average = (output[k * 2] + output[k * 2 + 1]) as f64 / 2.0;
            let difference = output[k * 2] as f64 - average;

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

pub fn inverse(input: &Vec<f64>) -> Vec<u8> {
    let mut input = input.clone();
    let mut length = 2;

    for _ in 0..3 {
        let mut output_temp = Vec::new();
        let mut temp: Vec<f64> = input.iter().take(length).cloned().collect();

        for k in 0..length / 2 {
            let (mut s, mut d) =  temp.split_at(length / 2);

            let sk = s[k] + d[k];
            let dk = s[k] - d[k];

            output_temp.push(sk);
            output_temp.push(dk);
        }

        for y in 0..output_temp.len() {
            input[y] = output_temp[y];
        }

        length *= 2;
    }

    let output = input.iter().map(|x| *x as u8).collect();

    output
}

#[test]
fn haar_1d_forward_test() {
    let input = vec![6, 12, 15, 15, 14, 12, 120, 116];
    let expected = vec![38.75, -26.75, -3.0, -52.5, -3.0, 0.0, 1.0, 2.0];

    let forward = forward(&input);

    assert_eq!(forward, expected);
}

#[test]
fn haar_1d_inverse_test() {
    let input = vec![38.75, -26.75, -3.0, -52.5, -3.0, 0.0, 1.0, 2.0];
    let expected = vec![6, 12, 15, 15, 14, 12, 120, 116];

    let inverse = inverse(&input);

    assert_eq!(expected, inverse);
}
