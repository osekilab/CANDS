use std::fmt;



/// Features are identified by strings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Feature(String);



impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}



impl Feature {
    pub fn new(s: String) -> Self {
        Feature(s)
    }
}



/// Macro to generate a feature.
///
/// # Example
///
/// ```
/// f!("wh")
/// ```
// #[macro_export]
macro_rules! f {
    ($literal:expr) => {
        Feature::new(String::from($literal))
    };
}

pub(crate) use f;



/// Macro to generate a feature set.
/// 
/// # Example
/// 
/// ```
/// fset!("n", "+voiced", "EPP")
/// ```
// #[macro_export]
macro_rules! fset {
    ($($literal:expr),*) => {
        {
            set!( $(f!($literal)),* )
        }
    };
}

pub(crate) use fset;



#[cfg(test)]
mod tests {
    /// Tests for the `fset!` macro.
    mod fset {
        use crate::prelude::*;

        /// Make an empty fset.
        #[test]
        fn empty_fset() {
            let set1: Set<Feature> = fset!();
            let set2: Set<Feature> = Set::new();
            assert_eq!(set1, set2);
        }

        /// Make a fset with three distinct elements.
        #[test]
        fn three_elems() {
            let set1: Set<Feature> = fset!("alpha", "beta", "gamma");
            let mut set2: Set<Feature> = Set::new();

            set2.insert(f!("alpha"));
            set2.insert(f!("beta"));
            set2.insert(f!("gamma"));

            assert_eq!(set1, set2);
        }
        
        /// Make a fset with duplicates. The resulting fset should not contain
        /// duplicates.
        #[test]
        fn dedup() {
            let set1: Set<Feature> = fset!("alpha", "alpha", "beta");
            let mut set2: Set<Feature> = Set::new();

            set2.insert(f!("alpha"));
            set2.insert(f!("beta"));

            assert_eq!(set1, set2);
        }
    }
}