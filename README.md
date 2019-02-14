`use discrete_transforms::*`

1D DCT 

```
let forward = dct_1d::forward(&Vec<f64>)
let inverse = dct_1d::inverse(&Vec<f64>)
```

1D Haar 

```
let forward = haar_1d::forward(&Vec<f64>)
let inverse = haar_1d::inverse(&Vec<f64>)
```

2D Transforms, create a `Transform` object and add in the transform you want 
```
let dct_forward = Transform::new(input, TransformType::DctForward).transform();
let dct_inverse = Transform::new(input, TransformType::DctInverse).transform();
let haar_forward = Transform::new(input, TransformType::HaarForward).transform();
let haar_inverse = Transform::new(input, TransformType::HaarInverse).transform();
```

# Problems / Things that need fixing:
* 2D Transforms only work on 8x8 blocks, block size should be user defined.
* 1D and 2D Haar transforms default to a step size of 3
* This needs to be read and changes made based on it https://rust-lang-nursery.github.io/api-guidelines/about.html
* And many more problems, no doubt...



This is a learning project as much as anything so things will change a lot :-)
