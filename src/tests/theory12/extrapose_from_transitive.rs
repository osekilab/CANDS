use crate::{prelude::*, deriv::LexicalArray};

use crate::tests::{init_logger, make_word, make_empty};

#[test]
/// *A story bothered me about Mary.
fn test() {
    std::env::set_var("RUST_LOG", "debug");
    init_logger();

    let mary = || {
        make_word(
            fset!("D"),
            fvec!("Mary")
        )
    };

    let about = || {
        make_word(
            fset!("P", "=D"),
            fvec!("about")
        )
    };

    let story = || {
        make_word(
            fset!("N", "=P"),
            fvec!("story")
        )
    };

    let a = || {
        make_word(
            fset!("D", "=N"),
            fvec!("a")
        )
    };

    let X = || {
        make_empty(
            fset!("X", "=v*", "=P"),
            fset!("X")
        )
    };

    let Y = || {
        make_empty(
            fset!("v*", "=X", "=v*"),
            fset!("Y")
        )
    };

    let me = || {
        make_word(
            fset!("D"),
            fvec!("me")
        )
    };

    let bothered = || {
        make_word(
            fset!("V", "=D"),
            fvec!("bothered")
        )
    };

    let vstar = || {
        make_empty(
            fset!("v*", "=V", "=D"),
            fset!("v*")
        )
    };

    let Past = || {
        make_empty(
            fset!("T", "=v*", "EPP"),
            fset!("Past")
        )
    };

    let C = || {
        make_empty(
            fset!("C", "=T"),
            fset!("C")
        )
    };

    let lex = set!(
        mary(),     story(),    about(),    a(),        X(),        Y(),
        me(),       bothered(), vstar(),    Past(),     C()
    );

    let ug = UniversalGrammar::<BasicTriggers>::new(
        fset!(
            "Mary", "about", "story", "a", "me", "bothered"
        ),
        fset!(
            "D", "P", "N", "V", "v*", "T", "C", "X", "Y",
            "=D", "=P", "=N", "=V", "=v*", "=T", "=X",
            "EPP"
        ),
        fset!(
            "Mary", "about", "story", "a", "X", "Y", "me", "bothered", "v*",
            "Past", "C"
        )
    );

    let il = ILanguage {
        lex,
        ug,
    };

    let stages = vec![
        Stage {
            la: LexicalArray::new(set!(
                lit!(mary()),
                lit!(about()),
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(me()),
                lit!(bothered()),
                lit!(vstar()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!()),
        },

        //  Select me
        Stage {
            la: LexicalArray::new(set!(
                lit!(mary()),
                lit!(about()),
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(bothered()),
                lit!(vstar()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(me()))
            )),
        },

        //  Select bothered
        Stage {
            la: LexicalArray::new(set!(
                lit!(mary()),
                lit!(about()),
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(vstar()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(bothered())),
                so!(lit!(me()))
            )),
        },

        //  Merge (bothered, me)
        Stage {
            la: LexicalArray::new(set!(
                lit!(mary()),
                lit!(about()),
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(vstar()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(bothered())),
                    so!(lit!(me())),
                )
            )),
        },

        //  Select vstar
        Stage {
            la: LexicalArray::new(set!(
                lit!(mary()),
                lit!(about()),
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(vstar())),
                so!(
                    so!(lit!(bothered())),
                    so!(lit!(me())),
                )
            )),
        },

        //  Merge (vstar, VP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(mary()),
                lit!(about()),
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(lit!(bothered())),
                        so!(lit!(me())),
                    ),
                )
            )),
        },

        //  Select Mary
        Stage {
            la: LexicalArray::new(set!(
                lit!(about()),
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(mary())),
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(lit!(bothered())),
                        so!(lit!(me())),
                    ),
                )
            )),
        },

        //  Select about
        Stage {
            la: LexicalArray::new(set!(
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(about())),
                so!(lit!(mary())),
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(lit!(bothered())),
                        so!(lit!(me())),
                    ),
                )
            )),
        },

        //  Merge (about, Mary)
        Stage {
            la: LexicalArray::new(set!(
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(about())),
                    so!(lit!(mary())),
                ),
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(lit!(bothered())),
                        so!(lit!(me())),
                    ),
                )
            )),
        },

        //  Select story
        Stage {
            la: LexicalArray::new(set!(
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(story())),
                so!(
                    so!(lit!(about())),
                    so!(lit!(mary())),
                ),
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(lit!(bothered())),
                        so!(lit!(me())),
                    ),
                )
            )),
        },

        //  Merge (story, PP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(story())),
                    so!(
                        so!(lit!(about())),
                        so!(lit!(mary())),
                    ),
                ),
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(lit!(bothered())),
                        so!(lit!(me())),
                    ),
                )
            )),
        },

        //  Select a
        Stage {
            la: LexicalArray::new(set!(
                lit!(X()),
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(a())),
                so!(
                    so!(lit!(story())),
                    so!(
                        so!(lit!(about())),
                        so!(lit!(mary())),
                    ),
                ),
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(lit!(bothered())),
                        so!(lit!(me())),
                    ),
                )
            )),
        },

        //  Merge (a, NP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(X()),
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(a())),
                    so!(
                        so!(lit!(story())),
                        so!(
                            so!(lit!(about())),
                            so!(lit!(mary())),
                        ),
                    ),
                ),
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(lit!(bothered())),
                        so!(lit!(me())),
                    ),
                )
            )),
        },

        //  Merge (v*P, DP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(X()),
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(lit!(a())),
                        so!(
                            so!(lit!(story())),
                            so!(
                                so!(lit!(about())),
                                so!(lit!(mary())),
                            ),
                        ),
                    ),
                    so!(
                        so!(lit!(vstar())),
                        so!(
                            so!(lit!(bothered())),
                            so!(lit!(me())),
                        ),
                    ),
                )
            )),
        },

        //  Select X
        Stage {
            la: LexicalArray::new(set!(
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(X())),
                so!(
                    so!(
                        so!(lit!(a())),
                        so!(
                            so!(lit!(story())),
                            so!(
                                so!(lit!(about())),
                                so!(lit!(mary())),
                            ),
                        ),
                    ),
                    so!(
                        so!(lit!(vstar())),
                        so!(
                            so!(lit!(bothered())),
                            so!(lit!(me())),
                        ),
                    ),
                )
            )),
        },

        //  Merge (X, v*P)
        Stage {
            la: LexicalArray::new(set!(
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(X())),
                    so!(
                        so!(
                            so!(lit!(a())),
                            so!(
                                so!(lit!(story())),
                                so!(
                                    so!(lit!(about())),
                                    so!(lit!(mary())),
                                ),
                            ),
                        ),
                        so!(
                            so!(lit!(vstar())),
                            so!(
                                so!(lit!(bothered())),
                                so!(lit!(me())),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Merge (XP, PP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(Y()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(lit!(about())),
                        so!(lit!(mary())),
                    ),
                    so!(
                        so!(lit!(X())),
                        so!(
                            so!(
                                so!(lit!(a())),
                                so!(
                                    so!(lit!(story())),
                                    so!(
                                        so!(lit!(about())),
                                        so!(lit!(mary())),
                                    ),
                                ),
                            ),
                            so!(
                                so!(lit!(vstar())),
                                so!(
                                    so!(lit!(bothered())),
                                    so!(lit!(me())),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Select Y
        Stage {
            la: LexicalArray::new(set!(
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Y())),
                so!(
                    so!(
                        so!(lit!(about())),
                        so!(lit!(mary())),
                    ),
                    so!(
                        so!(lit!(X())),
                        so!(
                            so!(
                                so!(lit!(a())),
                                so!(
                                    so!(lit!(story())),
                                    so!(
                                        so!(lit!(about())),
                                        so!(lit!(mary())),
                                    ),
                                ),
                            ),
                            so!(
                                so!(lit!(vstar())),
                                so!(
                                    so!(lit!(bothered())),
                                    so!(lit!(me())),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Merge (Y, XP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(Y())),
                    so!(
                        so!(
                            so!(lit!(about())),
                            so!(lit!(mary())),
                        ),
                        so!(
                            so!(lit!(X())),
                            so!(
                                so!(
                                    so!(lit!(a())),
                                    so!(
                                        so!(lit!(story())),
                                        so!(
                                            so!(lit!(about())),
                                            so!(lit!(mary())),
                                        ),
                                    ),
                                ),
                                so!(
                                    so!(lit!(vstar())),
                                    so!(
                                        so!(lit!(bothered())),
                                        so!(lit!(me())),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Merge (YP, v*P)
        Stage {
            la: LexicalArray::new(set!(
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(
                            so!(lit!(a())),
                            so!(
                                so!(lit!(story())),
                                so!(
                                    so!(lit!(about())),
                                    so!(lit!(mary())),
                                ),
                            ),
                        ),
                        so!(
                            so!(lit!(vstar())),
                            so!(
                                so!(lit!(bothered())),
                                so!(lit!(me())),
                            ),
                        ),
                    ),
                    so!(
                        so!(lit!(Y())),
                        so!(
                            so!(
                                so!(lit!(about())),
                                so!(lit!(mary())),
                            ),
                            so!(
                                so!(lit!(X())),
                                so!(
                                    so!(
                                        so!(lit!(a())),
                                        so!(
                                            so!(lit!(story())),
                                            so!(
                                                so!(lit!(about())),
                                                so!(lit!(mary())),
                                            ),
                                        ),
                                    ),
                                    so!(
                                        so!(lit!(vstar())),
                                        so!(
                                            so!(lit!(bothered())),
                                            so!(lit!(me())),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Select Past
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Past())),
                so!(
                    so!(
                        so!(
                            so!(lit!(a())),
                            so!(
                                so!(lit!(story())),
                                so!(
                                    so!(lit!(about())),
                                    so!(lit!(mary())),
                                ),
                            ),
                        ),
                        so!(
                            so!(lit!(vstar())),
                            so!(
                                so!(lit!(bothered())),
                                so!(lit!(me())),
                            ),
                        ),
                    ),
                    so!(
                        so!(lit!(Y())),
                        so!(
                            so!(
                                so!(lit!(about())),
                                so!(lit!(mary())),
                            ),
                            so!(
                                so!(lit!(X())),
                                so!(
                                    so!(
                                        so!(lit!(a())),
                                        so!(
                                            so!(lit!(story())),
                                            so!(
                                                so!(lit!(about())),
                                                so!(lit!(mary())),
                                            ),
                                        ),
                                    ),
                                    so!(
                                        so!(lit!(vstar())),
                                        so!(
                                            so!(lit!(bothered())),
                                            so!(lit!(me())),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Merge (Past, YP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(Past())),
                    so!(
                        so!(
                            so!(
                                so!(lit!(a())),
                                so!(
                                    so!(lit!(story())),
                                    so!(
                                        so!(lit!(about())),
                                        so!(lit!(mary())),
                                    ),
                                ),
                            ),
                            so!(
                                so!(lit!(vstar())),
                                so!(
                                    so!(lit!(bothered())),
                                    so!(lit!(me())),
                                ),
                            ),
                        ),
                        so!(
                            so!(lit!(Y())),
                            so!(
                                so!(
                                    so!(lit!(about())),
                                    so!(lit!(mary())),
                                ),
                                so!(
                                    so!(lit!(X())),
                                    so!(
                                        so!(
                                            so!(lit!(a())),
                                            so!(
                                                so!(lit!(story())),
                                                so!(
                                                    so!(lit!(about())),
                                                    so!(lit!(mary())),
                                                ),
                                            ),
                                        ),
                                        so!(
                                            so!(lit!(vstar())),
                                            so!(
                                                so!(lit!(bothered())),
                                                so!(lit!(me())),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Merge (TP, DP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(lit!(a())),
                        so!(
                            so!(lit!(story())),
                            so!(
                                so!(lit!(about())),
                                so!(lit!(mary())),
                            ),
                        ),
                    ),
                    so!(
                        so!(lit!(Past())),
                        so!(
                            so!(
                                so!(
                                    so!(lit!(a())),
                                    so!(
                                        so!(lit!(story())),
                                        so!(
                                            so!(lit!(about())),
                                            so!(lit!(mary())),
                                        ),
                                    ),
                                ),
                                so!(
                                    so!(lit!(vstar())),
                                    so!(
                                        so!(lit!(bothered())),
                                        so!(lit!(me())),
                                    ),
                                ),
                            ),
                            so!(
                                so!(lit!(Y())),
                                so!(
                                    so!(
                                        so!(lit!(about())),
                                        so!(lit!(mary())),
                                    ),
                                    so!(
                                        so!(lit!(X())),
                                        so!(
                                            so!(
                                                so!(lit!(a())),
                                                so!(
                                                    so!(lit!(story())),
                                                    so!(
                                                        so!(lit!(about())),
                                                        so!(lit!(mary())),
                                                    ),
                                                ),
                                            ),
                                            so!(
                                                so!(lit!(vstar())),
                                                so!(
                                                    so!(lit!(bothered())),
                                                    so!(lit!(me())),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Select C
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(lit!(C())),
                so!(
                    so!(
                        so!(lit!(a())),
                        so!(
                            so!(lit!(story())),
                            so!(
                                so!(lit!(about())),
                                so!(lit!(mary())),
                            ),
                        ),
                    ),
                    so!(
                        so!(lit!(Past())),
                        so!(
                            so!(
                                so!(
                                    so!(lit!(a())),
                                    so!(
                                        so!(lit!(story())),
                                        so!(
                                            so!(lit!(about())),
                                            so!(lit!(mary())),
                                        ),
                                    ),
                                ),
                                so!(
                                    so!(lit!(vstar())),
                                    so!(
                                        so!(lit!(bothered())),
                                        so!(lit!(me())),
                                    ),
                                ),
                            ),
                            so!(
                                so!(lit!(Y())),
                                so!(
                                    so!(
                                        so!(lit!(about())),
                                        so!(lit!(mary())),
                                    ),
                                    so!(
                                        so!(lit!(X())),
                                        so!(
                                            so!(
                                                so!(lit!(a())),
                                                so!(
                                                    so!(lit!(story())),
                                                    so!(
                                                        so!(lit!(about())),
                                                        so!(lit!(mary())),
                                                    ),
                                                ),
                                            ),
                                            so!(
                                                so!(lit!(vstar())),
                                                so!(
                                                    so!(lit!(bothered())),
                                                    so!(lit!(me())),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Merge (C, TP)
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(C())),
                    so!(
                        so!(
                            so!(lit!(a())),
                            so!(
                                so!(lit!(story())),
                                so!(
                                    so!(lit!(about())),
                                    so!(lit!(mary())),
                                ),
                            ),
                        ),
                        so!(
                            so!(lit!(Past())),
                            so!(
                                so!(
                                    so!(
                                        so!(lit!(a())),
                                        so!(
                                            so!(lit!(story())),
                                            so!(
                                                so!(lit!(about())),
                                                so!(lit!(mary())),
                                            ),
                                        ),
                                    ),
                                    so!(
                                        so!(lit!(vstar())),
                                        so!(
                                            so!(lit!(bothered())),
                                            so!(lit!(me())),
                                        ),
                                    ),
                                ),
                                so!(
                                    so!(lit!(Y())),
                                    so!(
                                        so!(
                                            so!(lit!(about())),
                                            so!(lit!(mary())),
                                        ),
                                        so!(
                                            so!(lit!(X())),
                                            so!(
                                                so!(
                                                    so!(lit!(a())),
                                                    so!(
                                                        so!(lit!(story())),
                                                        so!(
                                                            so!(lit!(about())),
                                                            so!(lit!(mary())),
                                                        ),
                                                    ),
                                                ),
                                                so!(
                                                    so!(lit!(vstar())),
                                                    so!(
                                                        so!(lit!(bothered())),
                                                        so!(lit!(me())),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Transfer (CP, CP)
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(lit!(C())),
                        so!(
                            so!(
                                so!(lit!(a())),
                                so!(
                                    so!(lit!(story())),
                                    so!(
                                        so!(lit!(about())),
                                        so!(lit!(mary())),
                                    ),
                                ),
                            ),
                            so!(
                                so!(lit!(Past())),
                                so!(
                                    so!(
                                        so!(
                                            so!(lit!(a())),
                                            so!(
                                                so!(lit!(story())),
                                                so!(
                                                    so!(lit!(about())),
                                                    so!(lit!(mary())),
                                                ),
                                            ),
                                        ),
                                        so!(
                                            so!(lit!(vstar())),
                                            so!(
                                                so!(lit!(bothered())),
                                                so!(lit!(me())),
                                            ),
                                        ),
                                    ),
                                    so!(
                                        so!(lit!(Y())),
                                        so!(
                                            so!(
                                                so!(lit!(about())),
                                                so!(lit!(mary())),
                                            ),
                                            so!(
                                                so!(lit!(X())),
                                                so!(
                                                    so!(
                                                        so!(lit!(a())),
                                                        so!(
                                                            so!(lit!(story())),
                                                            so!(
                                                                so!(lit!(about())),
                                                                so!(lit!(mary())),
                                                            ),
                                                        ),
                                                    ),
                                                    so!(
                                                        so!(lit!(vstar())),
                                                        so!(
                                                            so!(lit!(bothered())),
                                                            so!(lit!(me())),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ) =>
                    fvec!(
                        "a", "story", "bothered", "me", "about", "Mary"
                    ) ;
                    fset!(
                        "Mary", "about", "story", "a", "X", "Y", "me", "bothered", "v*",
                        "Past", "C"
                    )
                )
            )),
        },
    ];

    assert!(is_derivation(&il, &stages));
}