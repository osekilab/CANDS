use std::collections::BTreeSet;

use std::sync::{ Mutex };

use once_cell::sync::{ Lazy };


/// Type definition for convenience.
pub type Set<T> = BTreeSet<T>;



/// Macro to generate a set.
#[macro_export]
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



/// Stack depth counter for logging macros.
pub static LOG_STACK_DEPTH: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

/// Increase the stack depth counter.
/// 
/// You should call this at the beginning of a function where one of the logging macros are used.
macro_rules! inc {
    () => { *crate::utils::LOG_STACK_DEPTH.lock().unwrap() += 1; };
}

pub(crate) use inc;

/// Decrease the stack depth counter.
/// 
/// You should call this at the end of a function where one of the logging macros are used.
macro_rules! dec {
    () => { *crate::utils::LOG_STACK_DEPTH.lock().unwrap() -= 1; };
}

pub(crate) use dec;

/// Wrapper around the `log::trace` macro.
macro_rules! my_trace {
    ( $($e:expr),*) => {
        let s = format!($($e),*);
        for line in s.lines() {
            log::trace!("{} {}", std::iter::repeat(">").take(*crate::utils::LOG_STACK_DEPTH.lock().unwrap()).collect::<String>(), line);
        }
    };
}

/// Wrapper around the `log::debug` macro.
macro_rules! my_debug {
    ( $($e:expr),*) => {
        let s = format!($($e),*);
        for line in s.lines() {
            log::debug!("{} {}", std::iter::repeat(">").take(*crate::utils::LOG_STACK_DEPTH.lock().unwrap()).collect::<String>(), line);
        }
    };
}

/// Wrapper around the `log::info` macro.
macro_rules! my_info {
    ($($e:expr),*) => {
        let s = format!($($e),*);
        for line in s.lines() {
            log::info!("{} {}", std::iter::repeat(">").take(*crate::utils::LOG_STACK_DEPTH.lock().unwrap()).collect::<String>(), line);
        }
    };
}

/// Wrapper around the `log::error` macro.
macro_rules! my_error {
    ($($e:expr),*) => {
        let s = format!($($e),*);
        for line in s.lines() {
            log::error!("{} {}", std::iter::repeat(">").take(*crate::utils::LOG_STACK_DEPTH.lock().unwrap()).collect::<String>(), line);
        }
    };
}

pub(crate) use my_trace;
pub(crate) use my_debug;
pub(crate) use my_info;
pub(crate) use my_error;



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