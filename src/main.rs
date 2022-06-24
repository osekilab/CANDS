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



mod utils;
mod feature;
mod ops;
mod deriv;
mod prelude;



use prelude::*;

use std::marker::PhantomData;



fn main() {
    env_logger::init();

    let mut lex = Set::new();
    lex.insert( li!( ;; "John" ) );
    lex.insert( li!( ;; "see" ) );

    let ug = UniversalGrammar {
        phon_f:     fset!("John", "see"),
        syn_f:      fset!(),
        sem_f:      fset!(),
        select:     PhantomData::<BasicSelect>::default(),
        merge:      PhantomData::<TokenBasedMerge>::default(),
        transfer:   PhantomData::<BasicTransfer>::default(),
    };

    let il = ILanguage { lex, ug };

    let s1 = Stage {
        la: set!(
            lit!( li!( ;;; "John" ), 2 ),
            lit!( li!( ;;; "see" ), 1 ),
            lit!( li!( ;;; "John" ), 1 )
        ),
        w:  w!()
    };

    let s2 = Stage {
        la: set!(
            lit!( li!( ;;; "John" ), 2 ),
            lit!( li!( ;;; "see" ), 1 )
        ),
        w:  w!(
            so!( lit!( li!( ;;; "John" ), 1 ) )
        )
    };

    let s3 = Stage {
        la: set!(
            lit!( li!( ;;; "John" ), 2 )
        ),
        w:  w!(
            so!( lit!( li!( ;;; "John" ), 1 ) ),
            so!( lit!( li!( ;;; "see" ), 1 ) )
        )
    };

    let s4 = Stage {
        la: set!(
            lit!( li!( ;;; "John" ), 2 )
        ),
        w:  w!(
            so!( 
                so!( lit!( li!( ;;; "John" ), 1 ) ),
                so!( lit!( li!( ;;; "see" ), 1 ) ),
            )
        )
    };

    let s5 = Stage {
        la: set!(),
        w:  w!(
            so!( lit!( li!( ;;; "John" ), 2 ) ),
            so!( 
                so!( lit!( li!( ;;; "John" ), 1 ) ),
                so!( lit!( li!( ;;; "see" ), 1 ) ),
            )
        )
    };

    let s6 = Stage {
        la: set!(),
        w:  w!(
            so!(
                so!( lit!( li!( ;;; "John" ), 2 ) ),
                so!( 
                    so!( lit!( li!( ;;; "John" ), 1 ) ),
                    so!( lit!( li!( ;;; "see" ), 1 ) ),
                ),
            )
        )
    };

    let stages = vec![
        s1, s2, s3, s4, s5, s6
    ];

    eprintln!("Derivation? {}",
        is_derivation(&il, &stages)
    );

    eprintln!("\n\n\n");

    for (k, stage) in stages.iter().enumerate() {
        eprintln!("========================================");
        eprintln!("Stage {}:", k+1);
        eprintln!("Lexical array: {{ {} }}",
            stage.la.iter()
                .map(|li| format!("{}", li))
                .reduce(|a, b| format!("{}, {}", a, b))
                .unwrap_or_else(|| format!(""))
        );
        eprintln!("Workspace: {{\n{}\n}}",
            stage.w.iter()
                .map(|so| format!("{}", so))
                .reduce(|a, b| format!("{},\n{}", a, b))
                .unwrap_or_else(|| format!(""))
        );
    }

    eprintln!("\n\n\n");
}

#[cfg(test)]
mod tests;