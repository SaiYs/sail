use crate::StaticModInt;
use num_traits::{Inv, One};

#[derive(Debug, Clone)]
pub struct CombTable<const M: u64> {
    factorials: Box<[StaticModInt<M>]>,
    factorials_inversed: Box<[StaticModInt<M>]>,
}

impl<const M: u64> CombTable<M> {
    pub fn new(n: usize) -> Self {
        let mut cur = StaticModInt::<M>::one();
        let mut factorials = vec![cur];
        for i in 1..=n {
            cur *= i;
            factorials.push(cur);
        }

        let mut factorial_inversed = vec![factorials.last().unwrap().inv()];
        for i in (1..=n).rev() {
            factorial_inversed.push(*factorial_inversed.last().unwrap() * i);
        }
        factorial_inversed.reverse();

        Self {
            factorials: factorials.into_boxed_slice(),
            factorials_inversed: factorial_inversed.into_boxed_slice(),
        }
    }

    pub fn factorial(&self, k: usize) -> StaticModInt<M> {
        self.factorials[k]
    }

    pub fn factorial_inversed(&self, k: usize) -> StaticModInt<M> {
        self.factorials_inversed[k]
    }

    pub fn permutation(&self, n: usize, k: usize) -> StaticModInt<M> {
        self.factorial(n) * self.factorial_inversed(n - k)
    }

    pub fn combination(&self, n: usize, k: usize) -> StaticModInt<M> {
        self.permutation(n, k) * self.factorial_inversed(k)
    }

    pub fn binomial(&self, n: usize, k: usize) -> StaticModInt<M> {
        self.combination(n, k)
    }
}
