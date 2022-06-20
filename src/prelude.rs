/*
    Why do we need a prelude?
    This crate relies heavily on macros. These macros are syntactic sugar for
    creating objects like `LexicalItem` but in order to use them, the structs
    and types themselves must be in scope, e.g. you need to write
    `use ...::LexicalItem`. So you end up having to import a bunch of the same
    things everywhere just to use these macros. Instead, you can import the
    prelude which re-exports all of these things for you.
*/

//  Why pub(crate) not pub?
//  https://stackoverflow.com/a/41667202

pub(crate) use crate::utils::{ Set, set };
pub(crate) use crate::feature::{ Feature, f, fset };
pub(crate) use crate::ops::{ Select, Merge, Transfer, BasicSelect, TokenBasedMerge, BasicTransfer };
pub(crate) use crate::deriv::{
    UniversalGrammar, ILanguage, Stage, is_derivation, Workspace, w,
    li::{ LexicalItem, li },
    lit::{ LexicalItemToken, lit },
    so::{ SyntacticObject, ContainedSyntacticObjects, so }
};