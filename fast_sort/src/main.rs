#![feature(test)]

extern crate test;
extern crate rand;

#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use rand;
    use rand::{thread_rng, Rng};
    use rand::distributions::{Standard, Distribution};

    fn gen_random_list<A>(n: usize) -> Vec<A> 
        where Standard: Distribution<A> {
        (0..n).map(|_| rand::random()).collect()
    }

    fn gen_shuffled_list(n: i32) -> Vec<i32> {
        let mut list = Vec::with_capacity(n as usize);
        for x in 0..n {
            list.push(x)
        }

        thread_rng().shuffle(&mut list);

        list
    }

    fn is_sorted<A>(xs: &[A]) -> bool where A: Ord {
       xs.iter().zip(xs.iter().next()).all(|(x, y)| x < y)
    }

    #[bench]
    fn stable_default_sort_random_i32(b: &mut Bencher) {
        let list = gen_random_list::<i32>(1000);
        b.iter(|| {
            list.clone().sort()
        })
    }

    #[bench]
    fn unstable_default_sort_random_i32(b: &mut Bencher) {
        let list = gen_random_list::<i32>(1000);
        b.iter(|| {
            list.clone().sort_unstable()
        })
    }
}
