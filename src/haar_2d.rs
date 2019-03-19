use haar_1d::*;
use itertools::Itertools;

pub struct Haar2D {
    input: Vec<f64>
}

impl Haar2D {
    pub fn new() -> Haar2D {
        Haar2D {
            input: vec![]
        }
    }

    pub fn set_input(&mut self, input: Vec<f64>) -> &mut Self{
        self.input = input;

        self
    }

    pub fn forward(&self) -> Vec<f64> {
        let mut haar_1d = Haar1d::new();
        let mut output = Vec::new();
        let mut output_temp = Vec::new();
        let mut forward_input = Vec::new();

        for chunks in &self.input.iter().chunks(8) {
            haar_1d.set_input(chunks.map(|x| *x).collect_vec());

            forward_input = haar_1d.forward();

            output.extend(forward_input.iter().cloned());
        }

        forward_input.clear();

        for x in 0..8 {
            for y in 0..8 {
                forward_input.push(output[x + 8 * y]);
            }

            haar_1d.set_input(forward_input.clone());

            output_temp.extend(haar_1d.forward());

            forward_input.clear();
        }

        for x in 0..8 {
            for y in 0..8 {
                output[x + 8 * y] = output_temp[x * 8 + y];
            }
        }

        output
    }

    pub fn inverse(&self) -> Vec<f64> {
        let mut haar_1d = Haar1d::new();
        let mut output = Vec::new();
        let mut output_temp = Vec::new();
        let mut forward_input = Vec::new();

        for chunks in &self.input.iter().chunks(8) {
            haar_1d.set_input(chunks.map(|x| *x).collect_vec());

            forward_input = haar_1d.inverse();

            output.extend(forward_input.iter().cloned());
        }

        forward_input.clear();

        for x in 0..8 {
            for y in 0..8 {
                forward_input.push(output[x + 8 * y]);
            }

            haar_1d.set_input(forward_input.clone());

            output_temp.extend(haar_1d.inverse());

            forward_input.clear();
        }

        for x in 0..8 {
            for y in 0..8 {
                output[x + 8 * y] = output_temp[x * 8 + y];
            }
        }

        output
    }
}