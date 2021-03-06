//! Wrapper implementation

use itertools::Itertools as _;
use std::collections::{BTreeSet, BinaryHeap, HashSet, VecDeque};
use std::fmt::Display;

/// Wrapper to visualize items
///
/// Numeric primitives, char, &str, String and their references
/// will visualized in the same way as println! does.
///
/// Iterable Containers (array, Vec, VecDeque, HashSet, BTreeSet, BinayHeap)
/// and tuples with items less than six
/// will visualized in space-separated.
pub struct VisWrapper<T>(pub T);

macro_rules! impl_display_for_wrapped_iterables {
    ($t:ty => $($iterable:tt),*) => {
        impl<const N: usize> Display for VisWrapper<[$t; N]> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.iter().map(|x| VisWrapper(x)).join(" ").fmt(f)
            }
        }
        impl Display for VisWrapper<&[$t]> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.iter().map(|x| VisWrapper(x)).join(" ").fmt(f)
            }
        }
        impl<const N: usize> Display for VisWrapper<[&$t; N]> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.iter().map(|&x| VisWrapper(x)).join(" ").fmt(f)
            }
        }
        impl Display for VisWrapper<&[&$t]> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.iter().map(|&x| VisWrapper(x)).join(" ").fmt(f)
            }
        }
        $(
            impl Display for VisWrapper<$iterable<$t>> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.iter().map(|x| VisWrapper(x)).join(" ").fmt(f)
                }
            }
            impl Display for VisWrapper<&$iterable<$t>> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.iter().map(|x| VisWrapper(x)).join(" ").fmt(f)
                }
            }
            impl Display for VisWrapper<$iterable<&$t>> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.iter().map(|&x| VisWrapper(x)).join(" ").fmt(f)
                }
            }
            impl Display for VisWrapper<&$iterable<&$t>> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.iter().map(|&x| VisWrapper(x)).join(" ").fmt(f)
                }
            }
        )*
    };
}

macro_rules! impl_display_for_wrapped_primitives {
    ($($t:ty),*) => {
        $(
            impl Display for VisWrapper<$t> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }
            impl Display for VisWrapper<&$t> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl_display_for_wrapped_iterables!($t => Vec, VecDeque, HashSet, BTreeSet, BinaryHeap);
        )*
    };
}

impl_display_for_wrapped_primitives! {
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64,
    char, &str, String
}

impl<T> Display for VisWrapper<(T,)>
where
    T: Clone,
    VisWrapper<T>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", VisWrapper(self.0 .0.clone()),)
    }
}

impl<T, U> Display for VisWrapper<(T, U)>
where
    T: Clone,
    U: Clone,
    VisWrapper<T>: Display,
    VisWrapper<U>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            VisWrapper(self.0 .0.clone()),
            VisWrapper(self.0 .1.clone())
        )
    }
}

impl<T, U, V> Display for VisWrapper<(T, U, V)>
where
    T: Clone,
    U: Clone,
    V: Clone,
    VisWrapper<T>: Display,
    VisWrapper<U>: Display,
    VisWrapper<V>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            VisWrapper(self.0 .0.clone()),
            VisWrapper(self.0 .1.clone()),
            VisWrapper(self.0 .2.clone())
        )
    }
}

impl<T, U, V, W> Display for VisWrapper<(T, U, V, W)>
where
    T: Clone,
    U: Clone,
    V: Clone,
    W: Clone,
    VisWrapper<T>: Display,
    VisWrapper<U>: Display,
    VisWrapper<V>: Display,
    VisWrapper<W>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            VisWrapper(self.0 .0.clone()),
            VisWrapper(self.0 .1.clone()),
            VisWrapper(self.0 .2.clone()),
            VisWrapper(self.0 .3.clone())
        )
    }
}

impl<T, U, V, W, X> Display for VisWrapper<(T, U, V, W, X)>
where
    T: Clone,
    U: Clone,
    V: Clone,
    W: Clone,
    X: Clone,
    VisWrapper<T>: Display,
    VisWrapper<U>: Display,
    VisWrapper<V>: Display,
    VisWrapper<W>: Display,
    VisWrapper<X>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            VisWrapper(self.0 .0.clone()),
            VisWrapper(self.0 .1.clone()),
            VisWrapper(self.0 .2.clone()),
            VisWrapper(self.0 .3.clone()),
            VisWrapper(self.0 .4.clone()),
        )
    }
}
