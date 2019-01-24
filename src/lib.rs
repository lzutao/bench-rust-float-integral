#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    fn is_square_number(num: usize) -> bool {
        (num as f64).sqrt().fract() == 0.0
    }

    fn is_square_number_cast(num: usize) -> bool {
        let sqrt = (num as f64).sqrt() as usize;
        (sqrt * sqrt) == num
    }

    use rand::prelude::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_fract(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            for _ in 2..20_000 {
                let num = rng.gen::<u32>() as usize;
                black_box(is_square_number(num));
            }
        })
    }

    #[bench]
    fn bench_cast(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            for _ in 2..20_000 {
                let num = rng.gen::<u32>() as usize;
                black_box(is_square_number_cast(num));
            }
        })
    }
}
