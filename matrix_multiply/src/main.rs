#![feature(test, rust_2018_preview, stdsimd)]
extern crate faster;
extern crate test;
extern crate rand;
extern crate num_traits;
extern crate ndarray;

use ndarray::{Array2};
use num_traits::identities::{Zero, One};
use faster::*;

fn smarter_multiply<A>(a1: &Array2<A>, a2: &Array2<A>) -> Array2<A> 
    where A: Copy + Zero + One {

    let m = a1.shape()[0];
    let n = a2.shape()[1];
    assert_eq!(a1.shape()[1], a2.shape()[0]);
    let p = a1.shape()[1];
    let mut out_v: Vec<A> = vec![A::zero(); p * p];

    for i in 0..m {
        let r = a1.row(i);
        for j in 0..n {
            let c = a2.column(j);
            let mut acc = A::zero();
            for x in 0..p {
                acc = acc + r[x] * c[x];
            }
            out_v[i * p +j] = acc;
        }
    }

    Array2::from_shape_vec((p, p), out_v).unwrap()
}

fn naive_multiply<A>(a1: &Array2<A>, a2: &Array2<A>) -> Array2<A> 
    where A: Copy + Zero + One {

    let m = a1.shape()[0];
    let n = a2.shape()[1];
    assert_eq!(a1.shape()[1], a2.shape()[0]);
    let p = a1.shape()[1];
    let mut out = Array2::zeros((m, n));

    for i in 0..m {
        let r = a1.row(i);
        for j in 0..n {
            let c = a2.column(j);
            let mut acc = A::zero();
            for x in 0..p {
                acc = acc + r[x] * c[x];
            }
            out[[i, j]] = acc
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::naive_multiply;
    use super::smarter_multiply;
    use super::test::Bencher;
    use ndarray::{Array2};
    use rand;

    fn random_matrix<A>(dim: (usize, usize)) -> Array2<A> 
        where rand::distributions::Standard: rand::distributions::Distribution<A>
    {
        Array2::from_shape_fn(dim, |(_, _)| rand::random::<A>())
    }

    #[test]
    fn identity() {
        let i: Array2<i32> = Array2::eye(4);
        assert_eq!(i, smarter_multiply(&i, &i));
    }

    #[test]
    fn random_float_100_square() {
        let x: Array2<f64> = random_matrix((100, 100));
        let y = random_matrix((100, 100));

        assert_eq!(x.dot(&y), smarter_multiply(&x, &y))
    }

    #[bench]
    fn bench_naive_1000(b: &mut Bencher) {
        let x: Array2<f64> = random_matrix((100, 100));
        let y = random_matrix((100, 100));

        b.iter(|| naive_multiply(&x, &y))
    }

    #[bench]
    fn bench_smarter_1000(b: &mut Bencher) {
        let x: Array2<f64> = random_matrix((100, 100));
        let y = random_matrix((100, 100));

        b.iter(|| smarter_multiply(&x, &y))
    }

    #[bench]
    fn bench_default_1000(b: &mut Bencher) {
        let x: Array2<f64> = random_matrix((100, 100));
        let y = random_matrix((100, 100));

        b.iter(|| x.dot(&y))
    }
}
