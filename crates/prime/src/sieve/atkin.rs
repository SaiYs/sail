use super::PrimeSieve;

const WHEEL: &[usize] = &[1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59];
const CANDEDATE_A: &[usize] = &[1, 13, 17, 29, 37, 41, 49, 53];
const CANDIDATE_B: &[usize] = &[7, 19, 31, 43];
const CANDIDATE_C: &[usize] = &[11, 23, 47, 59];

#[derive(Debug, Clone)]
pub struct SieveOfAtkin {
    limit: usize,
    is_prime: Vec<bool>,
    primes: Vec<usize>,
}

impl SieveOfAtkin {
    pub fn new(limit: usize) -> Self {
        let mut is_prime = vec![false; limit + 1];
        is_prime[2] = true;
        is_prime[3] = true;
        is_prime[5] = true;

        let cap = if limit > 1 {
            let x = limit as f64;
            (x / x.ln() * (1.0 + 1.2762 / x.ln())) as usize
        } else {
            0
        };
        let mut primes = Vec::with_capacity(cap);
        primes.extend_from_slice(&[2, 3, 5]);

        for x in 1.. {
            if 4 * x * x > limit {
                break;
            }
            for y in (1..).step_by(2) {
                let n = 4 * x * x + y * y;
                if n > limit {
                    break;
                }

                if CANDEDATE_A.contains(&(n % 60)) {
                    is_prime[n] = !is_prime[n];
                }
            }
        }

        for x in (1..).step_by(2) {
            if 3 * x * x > limit {
                break;
            }
            for y in (2..).step_by(2) {
                let n = 3 * x * x + y * y;
                if n > limit {
                    break;
                }

                if CANDIDATE_B.contains(&(n % 60)) {
                    is_prime[n] = !is_prime[n];
                }
            }
        }

        for x in 2.. {
            if 3 * x * x - (x - 1) * (x - 1) > limit {
                break;
            }
            for y in (1..=x - 1).rev().step_by(2) {
                let n = 3 * x * x - y * y;
                if n > limit {
                    break;
                }

                if CANDIDATE_C.contains(&(n % 60)) {
                    is_prime[n] = !is_prime[n];
                }
            }
        }

        'a: for w in 0.. {
            for &x in WHEEL {
                let n = 60 * w + x;
                if n < 7 {
                    continue;
                }
                if n * n > limit {
                    break 'a;
                }

                if is_prime[n] {
                    'b: for w in 0.. {
                        for &x in WHEEL {
                            let c = n * n * (60 * w + x);
                            if c > limit {
                                break 'b;
                            }

                            is_prime[c] = false;
                        }
                    }
                }
            }
        }

        for (i, _) in is_prime
            .iter()
            .enumerate()
            .take(limit + 1)
            .skip(7)
            .filter(|&(_, &e)| e)
        {
            primes.push(i);
        }

        Self {
            limit,
            is_prime,
            primes,
        }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn is_prime(&self, n: usize) -> bool {
        self.is_prime[n]
    }

    pub fn primes(&self) -> &[usize] {
        &self.primes
    }
}

impl PrimeSieve for SieveOfAtkin {
    fn limit(&self) -> usize {
        self.limit()
    }

    fn is_prime(&self, n: usize) -> bool {
        self.is_prime(n)
    }

    fn primes(&self) -> &[usize] {
        self.primes()
    }
}
