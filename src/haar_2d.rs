use haar_1d::*;
use util::{TransformDirection, TransformDirections};
use itertools::Itertools;

#[derive(Debug)]
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
}

impl TransformDirections for Haar2D {
    fn forward(&self) -> Vec<f64> {
        let input = self.input.clone();

        let direction = TransformDirection::Forward;

        let output = discrete_transform!(Haar1D, input, direction);

        output
    }

    fn inverse(&self) -> Vec<f64> {
        let input = self.input.clone();

        let direction = TransformDirection::Inverse;

        let output = discrete_transform!(Haar1D, input, direction);

        output
    }
}