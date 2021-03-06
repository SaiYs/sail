/// # Sail prelude
///
/// Frequency used items
///
/// ```
/// use sail::prelude::*;
/// ```
pub mod prelude {
    pub use crate::consts::*;
    pub use crate::{
        max, min,
        snippet::{
            compare::{MaxAssign, MinAssign},
            counter::Counter,
            distance::{EuclideanDistance, ManhattanDistance},
            index_compression::IndexCompression,
        },
    };
    pub use io::{
        input::marker::Digits,
        input_interactive,
        output::{polar_question, Yn},
        proconio::{
            input, is_stdin_empty,
            marker::{Bytes, Chars, Isize1, Usize1},
        },
        trace, vis,
    };
    pub use modint::{ModInt1000000007, ModInt998244353, StaticModInt as ModInt};
}

pub mod consts;
pub mod snippet;

// re-exported crates
pub use accumulate;
pub use algebraics;
pub use bitset;
pub use fenwick_tree;
pub use graph;
pub use io;
pub use modint;
pub use prime;
pub use rolling_hash;
// pub use sample_generater;
pub use segment_tree;
pub use sparse_table;
// pub use sqrt_decomposition;
// pub use suffix_array;
pub use timer;
