#[cfg(test)]
extern crate acme_tensor as tensor;

use tensor::Tensor;

#[test]
fn test_tensor() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = Tensor::<f64>::ones(shape);

    assert_eq!(a[&[1, 1]], b[&[0, 1]]);
}
