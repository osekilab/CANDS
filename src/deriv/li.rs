use crate::utils::{ Set };
use crate::feature::{ Feature, SyntacticFeature };

use std::fmt;



/// Lexical item.
/// 
/// From Definition 2 in C&S 2016, p. 44.
/// 
/// >A *lexical item* is a triple: $\\textrm{LI} = \langle \\textrm{SEM}, \\textrm{SYN}, \\textrm{PHON} \rangle$ where $\\textrm{SEM}$ and $\\textrm{SYN}$ are finite sets such that $\\textrm{SEM} \\subseteq \\textrm{SEM-F}$, $\\textrm{SYN} \\subseteq \\textrm{SYN-F}$, and $\\textrm{PHON} \\in \\textrm{PHON-F}$*.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LexicalItem {
    /// Semantic features.
    pub sem: Set<Feature>,

    /// Syntactic features.
    pub syn: Set<SyntacticFeature>,

    /// Phonological features.
    pub phon: Vec<Feature>,

    /// Optional shorthand.
    pub shorthand: Option<String>,
}



impl LexicalItem {
    pub fn new(
        sem: Set<Feature>,
        syn: Set<SyntacticFeature>,
        phon: Vec<Feature>,
        shorthand: Option<String>,
    ) -> Self {
        Self { sem, syn, phon, shorthand }
    }
}



impl fmt::Display for LexicalItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.shorthand {
            None => {
                write!(f, "{{ sem: {{")?;

                let mut semit = self.sem.iter();
                if let Some(semf) = semit.next() {
                    write!(f, "{}", semf)?;
                }
                semit.try_fold((), |_, semf| write!(f, ", {}", semf))?;

                write!(f, "}}; syn: {{")?;

                let mut synit = self.syn.iter();
                if let Some(synf) = synit.next() {
                    write!(f, "{}", synf)?;
                }
                synit.try_fold((), |_, synf| write!(f, ", {}", synf))?;

                write!(f, "}}; phon: {{")?;

                let mut phonit = self.phon.iter();
                if let Some(phonf) = phonit.next() {
                    write!(f, "{}", phonf)?;
                }
                phonit.try_fold((), |_, phonf| write!(f, ", {}", phonf))?;

                write!(f, "}} }}")
            },

            Some(ref shorthand) => {
                write!(f, "{}", shorthand)
            },
        }
    }
}



/// Macro to generate a lexical item.
/// 
/// You can optionally provide a shorthand.
/// 
/// # Example
/// 
/// Here's an example without a shorthand:
/// 
/// ```
/// li!(; "n", "EPP"; "-voice")
/// ```
/// 
/// And one with:
/// 
/// ```
/// li!(; "n", "EPP"; "-voice"; "book")
/// ```
#[macro_export]
macro_rules! li {
    ($($sem:expr),*; $($syn:expr),*; $($phon:expr),*) => {
        LexicalItem::new(
            fset!($($sem),*),
            synfset!($($syn),*),
            fvec!($($phon),*),
            None
        )
    };

    ($($sem:expr),*; $($syn:expr),*; $($phon:expr),*; $shorthand:expr) => {
        LexicalItem::new(
            fset!($($sem),*),
            synfset!($($syn),*),
            fvec!($($phon),*),
            Some(String::from($shorthand))
        )
    };

    ($shorthand:expr) => {
        LexicalItem::new(
            fset!(),
            synfset!(),
            fvec!(),
            Some(String::from($shorthand))
        )
    };
}

pub(crate) use li;



#[cfg(test)]
mod tests {
    /// Tests for the `li!` macro.
    mod li {
        use crate::prelude::*;



        /// Make a lexical item with no semantics features, the syntactic feature
        /// "N", the phonological feature "Alex" and no shorthand.
        #[test]
        fn alex() {
            let li1 = li!(; "N"; "Alex");

            let li2 = LexicalItem {
                sem:        fset!(),
                syn:        synfset!("N"),
                phon:       fvec!("Alex"),
                shorthand:  None,
            };

            assert_eq!(li1, li2);
        }



        /// Make a lexical item with no semantics features, the syntactic feature
        /// "N", the phonological feature "Alex" and the shorthand "Alex".
        #[test]
        fn alex_with_shorthand() {
            let li1 = li!(; "N"; "Alex"; "Alex");

            let li2 = LexicalItem {
                sem:        fset!(),
                syn:        synfset!("N"),
                phon:       fvec!("Alex"),
                shorthand:  Some(format!("Alex")),
            };

            assert_eq!(li1, li2);
        }



        /// Make a lexical item with no features, and the shorthand "Alex".
        #[test]
        fn empty_with_shorthand() {
            let li1 = li!("Alex");

            let li2 = LexicalItem {
                sem:        fset!(),
                syn:        synfset!(),
                phon:       fvec!(),
                shorthand:  Some(format!("Alex"))
            };

            assert_eq!(li1, li2);
        }



        /// Make a lexical item with no semantics features, the syntactic feature
        /// "N", the phonological feature "Alex" and no shorthand. Check if the
        /// `std::fmt::Display` trait implementation for this lexical item is
        /// correct.
        #[test]
        fn display_alex() {
            let li = li!(; "N"; "Alex");

            let disp = format!("{}", li);

            assert_eq!(
                disp,
                "{ sem: {}; syn: {N}; phon: {Alex} }"
            );
        }



        /// Make a lexical item with no semantics features, the syntactic feature
        /// "N", the phonological feature "Alex" and the shorthand "Alex". Check if
        /// the `std::fmt::Display` trait implementation for this lexical item is
        /// correct.
        #[test]
        fn display_alex_with_shorthand() {
            let li = li!(; "N"; "Alex"; "Alex");

            let disp = format!("{}", li);

            assert_eq!(
                disp,
                "Alex"
            );
        }
    }
}