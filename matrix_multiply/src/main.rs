#![feature(test)]

extern crate test;
extern crate rand;
extern crate num_traits;

#[macro_use]
extern crate ndarray;

use ndarray::{Array2, Data};
use num_traits::identities::{Zero, One};

fn multiply<A>(x1: &Array2<A>, x2: &Array2<A>) -> Array2<A> 
    where A: Clone + Zero + One {
    x1.dot(x2)
}

#[cfg(test)]
mod tests {
    use super::multiply;
    use super::test::Bencher;
    use ndarray::{Array, Array2, ShapeBuilder};
    use rand;

    #[test]
    fn identity() {
        let I = array![[1., 0.], [0., 1.]];
        assert_eq!(I, multiply(&I, &I));
    }

    #[bench]
    fn bench_default_1000(b: &mut Bencher) {
        let x1 = Array2::from_shape_fn((100, 100), |(_, _)| rand::random::<f64>());
        let x2 = Array2::from_shape_fn((100, 100), |(_, _)| rand::random::<f64>());

        b.iter(|| x1.dot(&x2))
    }
}
