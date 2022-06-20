use std::collections::BTreeSet;



/// Type definition for convenience.
pub type Set<T> = BTreeSet<T>;



/// Macro to generate a set.
// #[macro_export]
macro_rules! set {
    () => { Set::new() };
    ($($e:expr),*) => {
        {
            let mut myset = Set::new();
            $(myset.insert($e);)*
            myset
        }
    };
}

pub(crate) use set; // https://stackoverflow.com/a/31749071



#[cfg(test)]
mod tests {
    /// Tests for the `set!` macro.
    mod set {
        use crate::prelude::*;

        /// Make an empty set.
        #[test]
        fn empty_usize_set() {
            let set1: Set<usize> = set!();
            let set2: Set<usize> = Set::new();
            assert_eq!(set1, set2);
        }
        
        /// Make a set with three distinct elements.
        #[test]
        fn three_elems() {
            let set1: Set<usize> = set!(12, 13, 14);
            let mut set2: Set<usize> = Set::new();
        
            set2.insert(12);
            set2.insert(13);
            set2.insert(14);
        
            assert_eq!(set1, set2);
        }
        
        /// Make a set with duplicates. The resulting set should not contain
        /// duplicates.
        #[test]
        fn dedup() {
            let set1: Set<usize> = set!(12, 13, 13);
            let mut set2: Set<usize> = Set::new();

            set2.insert(12);
            set2.insert(13);

            assert_eq!(set1, set2);
        }
    }
}