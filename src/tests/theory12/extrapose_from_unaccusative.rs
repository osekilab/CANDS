use crate::{prelude::*, deriv::LexicalArray};

use crate::tests::{init_logger, make_word, make_empty};

#[test]
/// A story appeared about Mary.
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
            fset!("X", "=V", "=P"),
            fset!("X")
        )
    };

    let Y = || {
        make_empty(
            fset!("V", "=X", "=V"),
            fset!("Y")
        )
    };

    let appeared = || {
        make_word(
            fset!("V", "=D"),
            fvec!("appeared")
        )
    };

    let v = || {
        make_empty(
            fset!("v", "=V"),
            fset!("v")
        )
    };

    let Past = || {
        make_empty(
            fset!("T", "=v", "EPP"),
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
        appeared(), v(),    Past(),     C()
    );

    let ug = UniversalGrammar::<BasicTriggers>::new(
        fset!(
            "Mary", "about", "story", "a", "appeared"
        ),
        fset!(
            "D", "P", "N", "V", "v", "T", "C", "X", "Y",
            "=D", "=P", "=N", "=V", "=v", "=T", "=X",
            "EPP"
        ),
        fset!(
            "Mary", "about", "story", "a", "X", "Y", "appeared", "v",
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
                lit!(appeared()),
                lit!(v()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!()),
        },
        
        //  Select Mary
        Stage {
            la: LexicalArray::new(set!(
                lit!(about()),
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(appeared()),
                lit!(v()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(mary()))
            )),
        },
        
        //  Select about
        Stage {
            la: LexicalArray::new(set!(
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(appeared()),
                lit!(v()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(about())),
                so!(lit!(mary()))
            )),
        },
        
        //  Merge (about, Mary)
        Stage {
            la: LexicalArray::new(set!(
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(appeared()),
                lit!(v()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(about())),
                    so!(lit!(mary())),
                )
            )),
        },
        
        //  Select story
        Stage {
            la: LexicalArray::new(set!(
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(appeared()),
                lit!(v()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(story())),
                so!(
                    so!(lit!(about())),
                    so!(lit!(mary())),
                )
            )),
        },
        
        //  Merge (story, PP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(appeared()),
                lit!(v()),
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
                )
            )),
        },
        
        //  Select a
        Stage {
            la: LexicalArray::new(set!(
                lit!(X()),
                lit!(Y()),
                lit!(appeared()),
                lit!(v()),
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
                )
            )),
        },
        
        //  Merge (a, NP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(X()),
                lit!(Y()),
                lit!(appeared()),
                lit!(v()),
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
                )
            )),
        },
        
        //  Select appeared
        Stage {
            la: LexicalArray::new(set!(
                lit!(X()),
                lit!(Y()),
                lit!(v()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(appeared())),
                so!(
                    so!(lit!(a())),
                    so!(
                        so!(lit!(story())),
                        so!(
                            so!(lit!(about())),
                            so!(lit!(mary())),
                        ),
                    ),
                )
            )),
        },
        
        //  Merge (appeared, DP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(X()),
                lit!(Y()),
                lit!(v()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(appeared())),
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
                )
            )),
        },
        
        //  Select X
        Stage {
            la: LexicalArray::new(set!(
                lit!(Y()),
                lit!(v()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(X())),
                so!(
                    so!(lit!(appeared())),
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
                )
            )),
        },
        
        //  Merge (X, VP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(Y()),
                lit!(v()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(X())),
                    so!(
                        so!(lit!(appeared())),
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
                    ),
                )
            )),
        },
        
        //  Merge (XP, PP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(Y()),
                lit!(v()),
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
                            so!(lit!(appeared())),
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
                        ),
                    ),
                )
            )),
        },
        
        //  Select Y
        Stage {
            la: LexicalArray::new(set!(
                lit!(v()),
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
                            so!(lit!(appeared())),
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
                        ),
                    ),
                )
            )),
        },
        
        //  Merge (Y, XP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(v()),
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
                                so!(lit!(appeared())),
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
                            ),
                        ),
                    ),
                )
            )),
        },
        
        //  Merge (YP, VP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(v()),
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(lit!(appeared())),
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
                                    so!(lit!(appeared())),
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
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },
        
        //  Select v
        Stage {
            la: LexicalArray::new(set!(
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(v())),
                so!(
                    so!(
                        so!(lit!(appeared())),
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
                                    so!(lit!(appeared())),
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
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },
        
        //  Merge (v, YP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(Past()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(v())),
                    so!(
                        so!(
                            so!(lit!(appeared())),
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
                                        so!(lit!(appeared())),
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
                    so!(lit!(v())),
                    so!(
                        so!(
                            so!(lit!(appeared())),
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
                                        so!(lit!(appeared())),
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
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },
        
        //  Merge (Past, vP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(Past())),
                    so!(
                        so!(lit!(v())),
                        so!(
                            so!(
                                so!(lit!(appeared())),
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
                                            so!(lit!(appeared())),
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
                            so!(lit!(v())),
                            so!(
                                so!(
                                    so!(lit!(appeared())),
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
                                                so!(lit!(appeared())),
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
                            so!(lit!(v())),
                            so!(
                                so!(
                                    so!(lit!(appeared())),
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
                                                so!(lit!(appeared())),
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
                                so!(lit!(v())),
                                so!(
                                    so!(
                                        so!(lit!(appeared())),
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
                                                    so!(lit!(appeared())),
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
                                    so!(lit!(v())),
                                    so!(
                                        so!(
                                            so!(lit!(appeared())),
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
                                                        so!(lit!(appeared())),
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
                        "a", "story", "appeared", "about", "Mary"
                    ) ;
                    fset!(
                        "Mary", "about", "story", "a", "X", "Y", "appeared",
                        "v", "Past", "C"
                    )
                )
            )),
        },
    ];

    assert!(is_derivation(&il, &stages));
}