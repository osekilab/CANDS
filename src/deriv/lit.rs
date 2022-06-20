use crate::deriv::li::LexicalItem;

use std::fmt;



/// Lexical item token.
/// 
/// From Definition 5 in C&S 2016, p. 45.
/// 
/// >A *lexical item token* is a pair $\\langle \\textrm{LI}, k \\rangle$ where $\\textrm{LI}$ is a lexical item and $k$ is an integer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LexicalItemToken {
    pub li: LexicalItem,
    pub k: usize,
}



impl LexicalItemToken {
    pub fn new(
        li: LexicalItem,
        k: usize
    ) -> Self {
        LexicalItemToken { li, k }
    }
}



impl fmt::Display for LexicalItemToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.li, self.k)
    }
}



/// Macro to generate a lexical item token from a lexical item and an index.
/// 
/// # Example
/// 
/// ```
/// lit!(li!(; "n", "EPP"; "-voice"; "book"), 1)
/// ```
macro_rules! lit {
    ($li:expr, $k:expr) => {
        LexicalItemToken::new($li, $k)
    };
}

pub(crate) use lit;



#[cfg(test)]
mod tests {
    /// Tests for the `lit!` macro.
    mod lit {
        use crate::prelude::*;

        /// Make a lexical item with no semantics features, the syntactic feature
        /// "N", the phonological feature "Alex" and no shorthand. Make a lexical
        /// item token from this lexical item, with the index 12345.
        #[test]
        fn alex() {
            let li1 = li!(; "N"; "Alex");
            let lit1 = lit!(li1, 12345);

            let li2 = LexicalItem::new(
                fset!(),
                fset!("N"),
                fset!("Alex"),
                None,
            );
            let lit2 = LexicalItemToken::new(li2, 12345);

            assert_eq!(lit1, lit2);
        }

        /// Make a lexical item with no semantics features, the syntactic feature
        /// "N", the phonological feature "Alex" and the shorthand "Alex". Make a
        /// lexical item token from this lexical item, with the index 12345.
        #[test]
        fn alex_with_shorthand() {
            let li1 = li!(; "N"; "Alex"; "Alex");
            let lit1 = lit!(li1, 12345);

            let li2 = LexicalItem::new(
                fset!(),
                fset!("N"),
                fset!("Alex"),
                Some(format!("Alex"))
            );
            let lit2 = LexicalItemToken::new(li2, 12345);

            assert_eq!(lit1, lit2);
        }

        /// Make a lexical item with no semantics features, the syntactic feature
        /// "N", the phonological feature "Alex" and no shorthand. Make a lexical
        /// item token from this lexical item, with the index 12345. Check if the
        /// `std::fmt::Display` trait implementation for this lexical item is
        /// correct.
        #[test]
        fn display_alex() {
            let li = li!(; "N"; "Alex");
            let lit = lit!(li, 12345);

            let disp = format!("{}", lit);

            assert_eq!(
                disp,
                "{ sem: {}; syn: {N}; phon: {Alex} }12345"
            );
        }


        /// Make a lexical item with no semantics features, the syntactic feature
        /// "N", the phonological feature "Alex" and the shorthand "Alex". Make a
        /// lexical item token from this lexical item, with the index 12345. Check
        /// if the `std::fmt::Display` trait implementation for this lexical item
        /// is correct.
        #[test]
        fn display_alex_with_shorthand() {
            let li = li!(; "N"; "Alex"; "Alex");
            let lit = lit!(li, 12345);

            let disp = format!("{}", lit);

            assert_eq!(
                disp,
                "Alex12345"
            );
        }
    }
}