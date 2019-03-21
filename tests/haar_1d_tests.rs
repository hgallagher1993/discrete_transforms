extern crate discrete_transforms;

use discrete_transforms::haar_1d::*;

#[test]
fn haar_1d_forward_test() {
    let input = vec![6.0, 12.0, 15.0, 15.0, 14.0, 12.0, 120.0, 116.0];
    let expected = vec![38.75, -26.75, -3.0, -52.5, -3.0, 0.0, 1.0, 2.0];

    let mut haar_1d = Haar1D::new();

    haar_1d.set_input(input);

    let forward = haar_1d.forward();

    assert_eq!(forward, expected);
}

#[test]
fn haar_1d_inverse_test() {
    let input = vec![38.75, -26.75, -3.0, -52.5, -3.0, 0.0, 1.0, 2.0];
    let expected = vec![6.0, 12.0, 15.0, 15.0, 14.0, 12.0, 120.0, 116.0];

    let mut haar_1d = Haar1D::new();

    haar_1d.set_input(input);

    let inverse = haar_1d.inverse();

    assert_eq!(expected, inverse);
}