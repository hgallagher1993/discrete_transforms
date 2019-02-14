extern crate discrete_transforms;

use discrete_transforms::dct_1d::*;

#[test]
fn dct_1d_forward_test() {
    let input = vec![139.0, 144.0, 149.0, 153.0, 155.0, 155.0, 155.0, 155.0];
    let expected = vec![426.03183566489486, -14.281166131964753, -7.423865308491784,
                        -2.080834053102315, -0.3535533905932624, -0.4669834002898554,
                        -0.3690851952774956, -0.018709851025652924];

    let forward = forward(&input);

    assert_eq!(forward, expected);
}

#[test]
fn dct_1d_inverse_test() {
    let input = vec![426.03183566489486, -14.281166131964753, -7.423865308491784,
                     -2.080834053102315, -0.3535533905932624, -0.4669834002898554,
                     -0.3690851952774956, -0.018709851025652924];
    let expected = vec![139.0, 144.0, 149.0, 153.0, 155.0, 155.0, 155.0, 155.0];

    let mut inverse = inverse(&input);

    for x in 0..8 {
        inverse[x] = inverse[x].round();
    }

    assert_eq!(inverse, expected);
}