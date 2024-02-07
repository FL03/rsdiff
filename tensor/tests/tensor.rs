#[cfg(test)]
extern crate acme_tensor as tensor;

use tensor::Tensor;

#[test]
fn test_tensor() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = Tensor::<f64>::zeros(shape);

    assert_ne!(&a, &b);
}
