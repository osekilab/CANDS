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
mod labels;



use prelude::*;

use std::marker::PhantomData;



fn main() {
    env_logger::init();



    //  Lexical item shorthands
    let me = || li!( "me'" ; "D", "1", "sg", "acc"; "me"; "me");
    let HELP = || li!( "help'" ; "V", "=D"; "HELP"; "HELP");
    let strong_light_verb = || li!( ; "v*", "=V", "=D", "acc"; "v*"; "v*");
    let she = || li!( "she'" ; "D", "3", "sg", "nom"; "she"; "she");
    let PAST = || li!( ; "T", "=v*", "epp", "nom"; "PAST"; "PAST");
    let nullcomp = || li!( ; "C", "=T"; "C"; "C");



    //  Lexicon
    let mut lex = set!(
        me(), HELP(), strong_light_verb(), she(), PAST(), nullcomp()
    );

    let ug = UniversalGrammar {
        phon_f:     fset!("me", "HELP", "v*", "she", "PAST", "C"),
        syn_f:      fset!("D", "V", "v*", "T", "C", "=D", "=V", "=v*", "=T", "1", "3", "sg", "nom", "acc", "epp"),
        sem_f:      fset!("me'", "help'", "she'"),
    };

    let il = ILanguage { lex, ug };



    let object = || lit!(me(), 1);
    let root = || lit!(HELP(), 1);
    let light_verb = ||lit!(strong_light_verb(), 1);
    let subject = || lit!(she(), 1);
    let tense = || lit!(PAST(), 1);
    let comp = || lit!(nullcomp(), 1);



    // let VP = || so!( root(), object(), );
    // let vbar = || so!( light_verb(), VP(), );
    // let vP = || so!( subject(), vbar(), );
    // let Tbar = || so!( tense(), vP(), );
    // let TP = || so! ( subject(), Tbar(), );
    // let CP = || so! ( comp(), TP(), );



    //  First stage.
    let s1 = Stage {
        la: set!(
            object(), root(), light_verb(), subject(), tense(), comp()
        ),
        w:  w!()
    };

    //  Select the object.
    let s2 = Stage {
        la: set!(
            root(), light_verb(), subject(), tense(), comp()
        ),
        w:  w!(
            so!(object())
        )
    };

    //  Select the root.
    let s3 = Stage {
        la: set!(
            light_verb(), subject(), tense(), comp()
        ),
        w:  w!(
            so!(root()),
            so!(object())
        )
    };

    //  Merge root with object.
    let s4 = Stage {
        la: set!(
            light_verb(), subject(), tense(), comp()
        ),
        w:  w!(
            so!(
                so!(root()),    //  root must come before object because triggered merge is not symmetric!
                so!(object()),
            )
        )
    };

    //  Select light verb.
    let s5 = Stage {
        la: set!(
            subject(), tense(), comp()
        ),
        w:  w!(
            so!(light_verb()),
            so!(
                so!(root()),
                so!(object()),
            )
        )
    };

    //  Merge light verb with VP.
    let s6 = Stage {
        la: set!(
            subject(), tense(), comp()
        ),
        w:  w!(
            so!(
                so!(light_verb()),
                so!(
                    so!(root()),
                    so!(object()),
                ),
            )
        )
    };

    //  Select subject.
    let s7 = Stage {
        la: set!(
            tense(), comp()
        ),
        w:  w!(
            so!(subject()),
            so!(
                so!(light_verb()),
                so!(
                    so!(root()),
                    so!(object()),
                ),
            )
        )
    };

    //  Merge vP with subject.
    let s8 = Stage {
        la: set!(
            tense(), comp()
        ),
        w:  w!(
            so!(
                so!(
                    so!(light_verb()),
                    so!(
                        so!(root()),
                        so!(object()),
                    ),
                ),
                so!(subject()),
            )
        )
    };

    //  Cyclic-Transfer vP.
    let s9 = Stage {
        la: set!(
            tense(), comp()
        ),
        w:  w!(
            so!(
                so!(
                    so!(light_verb()),
                    so!(
                        so!(
                            so!(root()),
                            so!(object()),
                        )
                        => fvec![ "HELP", "me" ]; fset!( "help'", "me'" )
                    ),
                ),
                so!(subject()),
            )
        )
    };

    //  Select T.
    let s10 = Stage {
        la: set!(
            comp()
        ),
        w:  w!(
            so!(tense()),
            so!(
                so!(
                    so!(light_verb()),
                    so!(
                        so!(
                            so!(root()),
                            so!(object()),
                        )
                        => fvec![ "HELP", "me" ]; fset!( "help'", "me'" )
                    ),
                ),
                so!(subject()),
            )
        )
    };

    //  Merge T with vP.
    let s11 = Stage {
        la: set!(
            comp()
        ),
        w:  w!(
            so!(
                so!(tense()),
                so!(
                    so!(
                        so!(light_verb()),
                        so!(
                            so!(
                                so!(root()),
                                so!(object()),
                            )
                            => fvec![ "HELP", "me" ]; fset!( "help'", "me'" )
                        ),
                    ),
                    so!(subject()),
                ),
            )
        )
    };

    //  Move subject to [Spec; TP].
    let s12 = Stage {
        la: set!(
            comp()
        ),
        w:  w!(
            so!(
                so!(
                    so!(tense()),
                    so!(
                        so!(
                            so!(light_verb()),
                            so!(
                                so!(
                                    so!(root()),
                                    so!(object()),
                                )
                                => fvec![ "HELP", "me" ]; fset!( "help'", "me'" )
                            ),
                        ),
                        so!(subject()),
                    ),
                ),
                so!(subject()),
            )
        )
    };

    //  Select C.
    let s13 = Stage {
        la: set!(),
        w:  w!(
            so!(comp()),
            so!(
                so!(
                    so!(tense()),
                    so!(
                        so!(
                            so!(light_verb()),
                            so!(
                                so!(
                                    so!(root()),
                                    so!(object()),
                                )
                                => fvec![ "HELP", "me" ]; fset!( "help'", "me'" )
                            ),
                        ),
                        so!(subject()),
                    ),
                ),
                so!(subject()),
            )
        )
    };

    //  Merge C with TP.
    let s14 = Stage {
        la: set!(),
        w:  w!(
            so!(
                so!(comp()),
                so!(
                    so!(
                        so!(tense()),
                        so!(
                            so!(
                                so!(light_verb()),
                                so!(
                                    so!(
                                        so!(root()),
                                        so!(object()),
                                    )
                                    => fvec![ "HELP", "me" ]; fset!( "help'", "me'" )
                                ),
                            ),
                            so!(subject()),
                        ),
                    ),
                    so!(subject()),
                ),
            )
        )
    };

    //  Transfer CP.
    let s15 = Stage {
        la: set!(),
        w:  w!(
            so!(
                so!(
                    so!(comp()),
                    so!(
                        so!(
                            so!(tense()),
                            so!(
                                so!(
                                    so!(light_verb()),
                                    so!(
                                        so!(
                                            so!(root()),
                                            so!(object()),
                                        )
                                        => fvec![ "HELP", "me" ]; fset!( "help'", "me'" )
                                    ),
                                ),
                                so!(subject()),
                            ),
                        ),
                        so!(subject()),
                    ),
                )
                => fvec![ "C", "she", "PAST", "v*", "HELP", "me" ]; fset!( "she'", "help'", "me'" )
            )
        )
    };

    let stages = vec![
        s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15
    ];

    eprintln!("Derivation? {}",
        is_derivation::<BasicTriggers>(&il, &stages)
    );

    eprintln!("\n\n\n");

    for (k, stage) in stages.iter().enumerate() {
        eprintln!("========================================");
        eprintln!("Stage {}:", k+1);
        eprintln!("Lexical array: {{\n{}\n}}",
            stage.la.iter()
                .map(|li| format!("    {}", li))
                .reduce(|a, b| format!("{},\n{}", a, b))
                .unwrap_or_else(|| format!(""))
        );
        eprintln!("Workspace: {}", stage.w);
    }

    eprintln!("\n\n\n");
}

#[cfg(test)]
mod tests;