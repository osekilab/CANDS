//! This is an implementation of Collins and Stabler 2016: "A Formalization of Minimalist Syntax", *Syntax* 19:1, pp. 43--78.
//!
//! # Implementations of C&S 2016
//!
//! The goal is to match each definition in the article with its
//! implementation.
//!
//! ## Preliminary definitions
//!
//! 1.  UG
//!
//!     See [`UniversalGrammar`].
//! 2.  lexical item
//!
//!     See [`LexicalItem`].
//! 3.  lexicon
//!
//!     See [`Lexicon`].
//! 4.  I-language
//!
//!     See [`ILanguage`].
//! 5.  lexical item token
//!
//!     See [`LexicalItemToken`].
//! 6.  lexical array
//!
//!     See [`LexicalArray`].
//! 7.  syntactic object (??)
//!
//!     See [`SyntacticObject`].
//! 8.  immediate contaiment (of syntactic objects)
//!
//!     See [`SyntacticObject::immediately_contains`] and [`Workspace::immediately_contains`].
//! 9.  containment (of syntactic objects)
//!
//!     See [`SyntacticObject::contains`] and [`Workspace::contains`].
//! 
//! ## Workspaces, Select and Merge
//! 
//! 10. stage, workspace
//!
//!     See [`Stage`] and [`Workspace`].
//! 11. root
//!
//!     See [`SyntacticObject::is_root`].
//! 12. Select
//!
//!     See [`BasicSelect::select`].
//! 13. Merge
//!
//!     See [`TokenBasedMerge::merge`].
//! 14. derivation
//!
//!     See [`is_derivation`].
//!
//!     (internal vs. external Merge?)
//! 15. derivability from lexicon
//!
//!     This definition does not have an implementation on its own.
//! 
//! ## Occurrences
//! 
//! 16. position and path
//! 
//!     See [`Occurrence::check`].
//! 17. occurrence
//! 
//!     See [`Occurrence`].
//! 18. immediate containment (for occurrences)
//! 
//!     See [`Occurrence::immediately_contains`] (and [`Occurrence::contains`]).
//! 19. sisterhood (for syntactic objects)
//! 
//!     See [`SyntacticObject::sisters_with`].
//! 20. sisterhood (for occurrences)
//! 
//!     See [`Occurrence::sisters_with`].
//! 21. c-command and asymmetric c-command (for syntactic objects)
//! 
//!     See [`SyntacticObject::c_commands`] and [`SyntacticObject::asymmetrically_c_commands`].
//! 22. c-command (for occurrences)
//! 
//!     See [`Occurrence::c_commands`] (and [`Occurrence::asymmetrically_c_commands`]).
//! 
//! ## General properties of derivations
//! 
//! 23. derivability
//!
//!     This definition does not have an implementation on its own.
//! 24. binary branching
//! 
//!     See [`SyntacticObject::is_binary_branching`].



pub mod utils;
pub mod feature;
pub mod ops;
pub mod deriv;
pub mod prelude;
pub mod labels;
// mod cli;



use std::path::{ PathBuf };

// use clap::Parser;



// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     #[clap(value_parser)]
//     file_path: Option<PathBuf>,
// }



// fn main() {
//     std::env::set_var("RUST_LOG", "info");

//     env_logger::Builder::from_default_env()
//         .format_target(false)
//         .init();

//     let args = Args::parse();

//     match args.file_path {
//         Some(ref file_path) => cli::run_file(file_path),
//         None => cli::run_stdin(),
//     };
// }

#[cfg(test)]
mod tests;