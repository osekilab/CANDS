use crate::utils::{ Set };
use crate::deriv::{
    Stage,
    lit::{ LexicalItemToken },
    so::{ SyntacticObject }
};



/// A trait for the Select operation.
pub trait Select {
    /// Perform Select, given a lexical item token and a stage.
    /// 
    /// Return a stage on success, and an error message on failure.
    fn select(a: LexicalItemToken, s: Stage) -> Result<Stage, String>;
}

/// The implementation of Select as in Definition 12 in C&S 2016, p. 47.
/// 
/// See [`BasicSelect::select`] for more information.
pub struct BasicSelect;

impl Select for BasicSelect {
    /// Select.
    /// 
    /// From Definition 12 in C&S 2016, p. 47:
    /// 
    /// >Let $S$ be a stage in a derivation $S = \\langle \\textit{LA}, W \\rangle$.
    /// >
    /// >If lexical token $A \\in \\textit{LA}$, then $\\textrm{Select} (A, S) = \\langle \\textit{LA} - \\{ A \\}, W \\cup \\{ A \\} \\rangle$.
    fn select(a: LexicalItemToken, s: Stage) -> Result<Stage, String> {
        let Stage { mut la, mut w } = s;

        if !la.remove(&a) {
            return Err(
                format!("BasicSelect: error.\nIn the provided stage, the lexical array:\n{:#?}\ndoes not contain the provided lexical item token:\n{:#?}", la, a)
            );
        }

        w.insert(SyntacticObject::LexicalItemToken(a));

        Ok(Stage { la, w })
    }
}

/// A trait for the token-based Merge operation.
pub trait Merge {
    /// Perform Merge, given a pair of syntactic objects.
    /// 
    /// Return a syntactic object on success, and an error message on failure.
    fn merge(a: SyntacticObject, b: SyntacticObject) -> Result<SyntacticObject, String>;
}

/// The implementation of a token-based Merge as in Definition 13 in C&S 2016, p. 47.
/// 
/// See [`TokenBasedMerge::merge`] for more information.
pub struct TokenBasedMerge;

impl Merge for TokenBasedMerge {
    /// Token-based Merge.
    /// 
    /// From Definition 13 in C&S 2016, p. 47:
    /// 
    /// >Given any two distinct syntactic objects $A, B$, $\\textrm{Merge} (A, B) = \\{ A, B \\}$.
    fn merge(a: SyntacticObject, b: SyntacticObject) -> Result<SyntacticObject, String> {
        //  a and b must be distinct!
        if a == b {
            return Err(
                format!("TokenBasedMerge: error.\nThe provided syntactic objects are not distinct.\nSyntactic object 1:\n{:#?}\nSyntactic object 2:\n{:#?}", a, b)
            );
        }

        let mut pair = Set::new();

        pair.insert(a);
        pair.insert(b);

        Ok(SyntacticObject::Set(pair))
    }
}

/// A trait for the Transfer operation.
pub trait Transfer {}

pub struct BasicTransfer {}

impl Transfer for BasicTransfer {}