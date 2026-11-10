pub use alloc::collections::*;

pub mod hashmap;
mod random;

pub use hashmap as hash_map;
pub use hashmap::HashMap;
pub use random::{DefaultHasher, RandomState};
