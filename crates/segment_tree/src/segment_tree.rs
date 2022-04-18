use algebraic_structures::monoid::Monoid;
use itertools::Itertools;
use std::{
    fmt::{Debug, Display},
    ops::RangeBounds,
};

/// Generic segment-tree
#[derive(Clone)]
pub struct SegmentTree<M: Monoid> {
    len: usize,
    capacity: usize,
    size: usize,
    height: usize,
    buffer: Vec<M>,
}

impl<M> Debug for SegmentTree<M>
where
    M: Monoid,
    M::T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = (0..self.height())
            .map(|x| {
                ((1 << x) - 1..)
                    .take(1 << x)
                    .map(|x| self.buffer[x].clone().get())
                    .join(" ")
            })
            .join("\n");
        write!(f, "\n{}", s)
    }
}

impl<M: Monoid> From<Vec<M::T>> for SegmentTree<M> {
    /// Complexity: O(n)
    fn from(v: Vec<M::T>) -> Self {
        let len = v.len();
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        let mut buffer = vec![M::identity(); size];

        for (i, e) in v.into_iter().enumerate() {
            buffer[size / 2 + i] = e.into();
        }

        for i in (0..capacity - 1).rev() {
            buffer[i] = M::binary_operation(buffer[i * 2 + 1].clone(), buffer[i * 2 + 2].clone());
        }

        Self {
            len,
            capacity,
            size,
            height,
            buffer,
        }
    }
}

impl<M: Monoid> From<&[M::T]> for SegmentTree<M> {
    fn from(v: &[M::T]) -> Self {
        let len = v.len();
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        let mut buffer = vec![M::identity(); size];

        for (i, e) in v.iter().enumerate() {
            buffer[size / 2 + i] = e.clone().into();
        }

        for i in (0..capacity - 1).rev() {
            buffer[i] = M::binary_operation(buffer[i * 2 + 1].clone(), buffer[i * 2 + 2].clone());
        }

        Self {
            len,
            capacity,
            size,
            height,
            buffer,
        }
    }
}

impl<M: Monoid> SegmentTree<M> {
    /// Returns the size of its buffer
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the length of the original array, NOT size of its buffer
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the height of the tree
    pub fn height(&self) -> usize {
        self.height
    }

    /// Create a new empty SegmentTree with given length
    pub fn new(len: usize) -> Self {
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        Self {
            len,
            capacity,
            size,
            height,
            buffer: vec![M::identity(); size],
        }
    }

    /// Returns ref of original array sliced from its buffer
    pub fn raw_leaves(&self) -> &[M] {
        &self.buffer[self.capacity - 1..self.size]
    }

    /// Returns a value of i-th leaf
    ///
    /// Complexity: O(1)
    pub fn get(&self, i: usize) -> M::T {
        self.buffer[self.capacity - 1 + i].clone().get()
    }

    /// Returns a folded value of leaves in `range`
    ///
    /// Complexity: O(log n)
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> M::T {
        let (from, to) = util::expand_range_bound(&range, 0, self.len());
        debug_assert!(from < to);

        let mut from = from + self.capacity - 1;
        let mut to = to + self.capacity - 1;
        let mut res = M::identity();

        while from < to {
            if from & 1 == 0 {
                res = M::binary_operation(res.clone(), self.buffer[from].clone());
                from += 1;
            }
            if to & 1 == 0 {
                to -= 1;
                res = M::binary_operation(res.clone(), self.buffer[to].clone());
            }
            from = (from - 1) >> 1;
            to = (to - 1) >> 1;
        }

        res.get()
    }

    /// Returns a folded value of all leaves
    ///
    /// Complexity: O(1)
    /// This can be more efficient than calling `self.get_range(..)`
    pub fn get_all(&self) -> M::T {
        self.buffer[0].clone().get()
    }

    /// Update one value at index `i` with `new_value`
    ///
    /// Complexity: O(log n)
    pub fn update(&mut self, i: usize, new_value: M::T) {
        let mut cur = self.capacity - 1 + i;
        self.buffer[cur] = new_value.into();
        while cur != 0 {
            cur = (cur - 1) >> 1;
            self.buffer[cur] = M::binary_operation(
                self.buffer[cur * 2 + 1].clone(),
                self.buffer[cur * 2 + 2].clone(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentTree;
    use algebraic_structures::{monoid::MSum, semigroup::SemiGroup};
    use itertools::Itertools;
    use modint::ModInt998244353;
    use rand::Rng;

    fn verify() {
        let mut rng = rand::thread_rng();

        let n = 1000usize;
        let a = (0..n)
            .map(|_| ModInt998244353::new(rng.gen_range(0..n)))
            .collect_vec();

        let mut st = SegmentTree::<MSum<ModInt998244353>>::from(a);

        for _ in 0..n {
            if rng.gen_bool(0.8) {
                // query
                let mut from = rng.gen_range(0..n);
                let mut to = rng.gen_range(0..n);

                if from > to {
                    std::mem::swap(&mut from, &mut to);
                }

                assert_eq!(
                    st.get_range(from..=to).get(),
                    st.raw_leaves()[from..=to]
                        .iter()
                        .copied()
                        .map(|x| x.get())
                        .sum::<ModInt998244353>()
                        .get()
                );
            } else {
                // update
                let i = rng.gen_range(0..n);
                let new_value = rng.gen_range(0..n);
                st.update(i, new_value.into());
            }
        }
    }

    #[test]
    fn run_varify() {
        for _ in 0..10000 {
            verify();
        }
    }
}
