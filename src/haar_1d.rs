use std::f64;

pub struct Haar1D {
    input: Vec<f64>
}

impl Haar1D {
    pub fn new() -> Haar1D {
        Haar1D {
            input: vec![]
        }
    }

    pub fn set_input(&mut self, input: Vec<f64>) -> &mut Self {
        self.input = input;

        self
    }

    pub fn forward(&mut self) -> Vec<f64> {
        let mut output: Vec<f64> = self.input.clone();
        let mut output_temp = vec![0.0; 8];

        let mut length = self.input.len() / 2;

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

    pub fn inverse(&mut self) -> Vec<f64> {
        let mut output = self.input.clone();
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
}
