use std::f64;
use itertools::Itertools;

use haar_1d;
use dct_1d;

pub enum TransformType {
    DctForward,
    DctInverse,
    HaarForward,
    HaarInverse
}

pub struct Transform {
    pub input: Vec<f64>,
    pub transform_type: fn(input: &Vec<f64>) -> Vec<f64>
}

impl Transform {
    pub fn new(input: Vec<f64>, transform_type: TransformType) -> Transform {
        let transform_type = match transform_type {
            TransformType::DctForward => dct_1d::forward,
            TransformType::DctInverse => dct_1d::inverse,
            TransformType::HaarForward => haar_1d::forward,
            TransformType::HaarInverse => haar_1d::inverse
        };

        Transform {
            input,
            transform_type
        }
    }

    pub fn transform(&self) -> Vec<f64> {
        let mut output: Vec<f64> = Vec::new();
        let mut output_temp: Vec<f64> = Vec::new();
        let mut forward_input = Vec::new();

        for chunks in &self.input.iter().chunks(8) {
            forward_input = (self.transform_type)(&chunks.map(|x| *x).collect_vec());

            output.extend(forward_input.iter().cloned());
        }

        forward_input.clear();

        for x in 0..8 {
            for y in 0..8 {
                forward_input.push(output[x + 8 * y]);
            }

            output_temp.extend((self.transform_type)(&forward_input).iter().cloned());

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