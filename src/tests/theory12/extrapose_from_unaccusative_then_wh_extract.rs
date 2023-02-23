use crate::{prelude::*, deriv::LexicalArray};

use crate::tests::{init_logger, make_word, make_empty};

#[test]
/// *I know who a story appeared about.
fn test() {
    std::env::set_var("RUST_LOG", "debug");
    init_logger();

    let who = || {
        make_word(
            fset!("D", "iwh"),
            fvec!("who")
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

    let Q = || {
        make_empty(
            fset!("C", "=T", "uwh"),
            fset!("Q")
        )
    };

    let know = || {
        make_word(
            fset!("V", "=C"),
            fvec!("know")
        )
    };

    let vstar = || {
        make_empty(
            fset!("v*", "=V", "=D"),
            fset!("v*")
        )
    };

    let we = || {
        make_word(
            fset!("D"),
            fvec!("we")
        )
    };

    let Pres = || {
        make_empty(
            fset!("T", "=v*", "EPP"),
            fset!("Pres")
        )
    };

    let C = || {
        make_empty(
            fset!("C", "=T"),
            fset!("C")
        )
    };

    let lex = set!(
        who(), about(), story(), a(), X(), Y(), appeared(), v(), Past(), Q(),
        know(), vstar(), we(), Pres(), C()
    );

    let ug = UniversalGrammar::<BasicTriggers>::new(
        fset!(
            "who", "about", "story", "a", "appeared", "know", "we"
        ),
        fset!(
            "D", "P", "N", "V", "v", "v*", "T", "C", "X", "Y",
            "=D", "=P", "=N", "=V", "=v", "=v*", "=T", "=X",
            "EPP", "wh"
        ),
        fset!(
            "who", "about", "story", "a", "X", "Y", "appeared", "v", "Past",
            "Q", "know", "v*", "we", "Pres", "C"
        )
    );

    let il = ILanguage {
        lex,
        ug,
    };

    let emb_TP = || {
        so!(
            so!(
                so!(
                    so!(lit!(a())),
                    so!(
                        so!(lit!(story())),
                        so!(
                            so!(lit!(about())),
                            so!(lit!(who())),
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
                                            so!(lit!(who())),
                                        ),
                                    ),
                                ),
                            ),
                            so!(
                                so!(lit!(Y())),
                                so!(
                                    so!(
                                        so!(lit!(about())),
                                        so!(lit!(who())),
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
                                                        so!(lit!(who())),
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
                "a", "story", "appeared", "about"
            ) ;
            fset!(
                "who", "about", "story", "a", "X", "Y",
                "appeared", "v", "Past"
            )
        )
    };

    let stages = vec![
        Stage {
            la: LexicalArray::new(set!(
                lit!(who()),
                lit!(about()),
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(appeared()),
                lit!(v()),
                lit!(Past()),
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!()),
        },
        
        //  Select who
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(who()))
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(about())),
                so!(lit!(who()))
            )),
        },
        
        //  Merge (about, who)
        Stage {
            la: LexicalArray::new(set!(
                lit!(story()),
                lit!(a()),
                lit!(X()),
                lit!(Y()),
                lit!(appeared()),
                lit!(v()),
                lit!(Past()),
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(about())),
                    so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(story())),
                so!(
                    so!(lit!(about())),
                    so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(story())),
                    so!(
                        so!(lit!(about())),
                        so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(a())),
                so!(
                    so!(lit!(story())),
                    so!(
                        so!(lit!(about())),
                        so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(a())),
                    so!(
                        so!(lit!(story())),
                        so!(
                            so!(lit!(about())),
                            so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
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
                            so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
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
                                so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
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
                                so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
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
                                    so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(lit!(about())),
                        so!(lit!(who())),
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
                                        so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Y())),
                so!(
                    so!(
                        so!(lit!(about())),
                        so!(lit!(who())),
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
                                        so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(Y())),
                    so!(
                        so!(
                            so!(lit!(about())),
                            so!(lit!(who())),
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
                                            so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
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
                                    so!(lit!(who())),
                                ),
                            ),
                        ),
                    ),
                    so!(
                        so!(lit!(Y())),
                        so!(
                            so!(
                                so!(lit!(about())),
                                so!(lit!(who())),
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
                                                so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
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
                                    so!(lit!(who())),
                                ),
                            ),
                        ),
                    ),
                    so!(
                        so!(lit!(Y())),
                        so!(
                            so!(
                                so!(lit!(about())),
                                so!(lit!(who())),
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
                                                so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
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
                                        so!(lit!(who())),
                                    ),
                                ),
                            ),
                        ),
                        so!(
                            so!(lit!(Y())),
                            so!(
                                so!(
                                    so!(lit!(about())),
                                    so!(lit!(who())),
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
                                                    so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
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
                                        so!(lit!(who())),
                                    ),
                                ),
                            ),
                        ),
                        so!(
                            so!(lit!(Y())),
                            so!(
                                so!(
                                    so!(lit!(about())),
                                    so!(lit!(who())),
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
                                                    so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
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
                                            so!(lit!(who())),
                                        ),
                                    ),
                                ),
                            ),
                            so!(
                                so!(lit!(Y())),
                                so!(
                                    so!(
                                        so!(lit!(about())),
                                        so!(lit!(who())),
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
                                                        so!(lit!(who())),
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
                lit!(Q()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
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
                                so!(lit!(who())),
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
                                                so!(lit!(who())),
                                            ),
                                        ),
                                    ),
                                ),
                                so!(
                                    so!(lit!(Y())),
                                    so!(
                                        so!(
                                            so!(lit!(about())),
                                            so!(lit!(who())),
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
                                                            so!(lit!(who())),
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
        
        //  Select Q
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(lit!(Q())),
                so!(
                    so!(
                        so!(lit!(a())),
                        so!(
                            so!(lit!(story())),
                            so!(
                                so!(lit!(about())),
                                so!(lit!(who())),
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
                                                so!(lit!(who())),
                                            ),
                                        ),
                                    ),
                                ),
                                so!(
                                    so!(lit!(Y())),
                                    so!(
                                        so!(
                                            so!(lit!(about())),
                                            so!(lit!(who())),
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
                                                            so!(lit!(who())),
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
        
        //  Merge (Q, TP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(Q())),
                    so!(
                        so!(
                            so!(lit!(a())),
                            so!(
                                so!(lit!(story())),
                                so!(
                                    so!(lit!(about())),
                                    so!(lit!(who())),
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
                                                    so!(lit!(who())),
                                                ),
                                            ),
                                        ),
                                    ),
                                    so!(
                                        so!(lit!(Y())),
                                        so!(
                                            so!(
                                                so!(lit!(about())),
                                                so!(lit!(who())),
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
                                                                so!(lit!(who())),
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
        
        //  Merge (QP, who)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(who())),
                    so!(
                        so!(lit!(Q())),
                        so!(
                            so!(
                                so!(lit!(a())),
                                so!(
                                    so!(lit!(story())),
                                    so!(
                                        so!(lit!(about())),
                                        so!(lit!(who())),
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
                                                        so!(lit!(who())),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        so!(
                                            so!(lit!(Y())),
                                            so!(
                                                so!(
                                                    so!(lit!(about())),
                                                    so!(lit!(who())),
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
                                                                    so!(lit!(who())),
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
                    ),
                )
            )),
        },
        
        //  Transfer (CP, TP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(know()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(who())),
                    so!(
                        so!(lit!(Q())),
                        emb_TP(),
                    ),
                )
            )),
        },
        
        //  Select know
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(lit!(know())),
                so!(
                    so!(lit!(who())),
                    so!(
                        so!(lit!(Q())),
                        emb_TP(),
                    ),
                )
            )),
        },
        
        //  Merge (know, CP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(know())),
                    so!(
                        so!(lit!(who())),
                        so!(
                            so!(lit!(Q())),
                            emb_TP(),
                        ),
                    ),
                )
            )),
        },
        
        //  Select vstar
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(we()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(lit!(vstar())),
                so!(
                    so!(lit!(know())),
                    so!(
                        so!(lit!(who())),
                        so!(
                            so!(lit!(Q())),
                            emb_TP(),
                        ),
                    ),
                )
            )),
        },
        
        //  Merge (vstar, VP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(we()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(lit!(know())),
                        so!(
                            so!(lit!(who())),
                            so!(
                                so!(lit!(Q())),
                                emb_TP(),
                            ),
                        ),
                    ),
                )
            )),
        },
        
        //  Transfer (vstarP, VP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(we()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(
                            so!(lit!(know())),
                            so!(
                                so!(lit!(who())),
                                so!(
                                    so!(lit!(Q())),
                                    emb_TP(),
                                ),
                            ),
                        ) =>
                        fvec!( "know", "who", "a", "story", "appeared", "about" ) ;
                        fset!( "know", "who", "Q", "about", "story", "a", "X", "Y", "appeared", "v", "Past" )
                    ),
                )
            )),
        },
        
        //  Select we
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(lit!(we())),
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(
                            so!(lit!(know())),
                            so!(
                                so!(lit!(who())),
                                so!(
                                    so!(lit!(Q())),
                                    emb_TP(),
                                ),
                            ),
                        ) =>
                        fvec!( "know", "who", "a", "story", "appeared", "about" ) ;
                        fset!( "know", "who", "Q", "about", "story", "a", "X", "Y", "appeared", "v", "Past" )
                    ),
                )
            )),
        },
        
        //  Merge (v*P, we)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C()),
                lit!(Pres())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(we())),
                    so!(
                        so!(lit!(vstar())),
                        so!(
                            so!(
                                so!(lit!(know())),
                                so!(
                                    so!(lit!(who())),
                                    so!(
                                        so!(lit!(Q())),
                                        emb_TP(),
                                    ),
                                ),
                            ) =>
                            fvec!( "know", "who", "a", "story", "appeared", "about" ) ;
                            fset!( "know", "who", "Q", "about", "story", "a", "X", "Y", "appeared", "v", "Past" )
                        ),
                    ),
                )
            )),
        },
        
        //  Select Pres
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Pres())),
                so!(
                    so!(lit!(we())),
                    so!(
                        so!(lit!(vstar())),
                        so!(
                            so!(
                                so!(lit!(know())),
                                so!(
                                    so!(lit!(who())),
                                    so!(
                                        so!(lit!(Q())),
                                        emb_TP(),
                                    ),
                                ),
                            ) =>
                            fvec!( "know", "who", "a", "story", "appeared", "about" ) ;
                            fset!( "know", "who", "Q", "about", "story", "a", "X", "Y", "appeared", "v", "Past" )
                        ),
                    ),
                )
            )),
        },
        
        //  Merge (Pres, v*P)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(Pres())),
                    so!(
                        so!(lit!(we())),
                        so!(
                            so!(lit!(vstar())),
                            so!(
                                so!(
                                    so!(lit!(know())),
                                    so!(
                                        so!(lit!(who())),
                                        so!(
                                            so!(lit!(Q())),
                                            emb_TP(),
                                        ),
                                    ),
                                ) =>
                                fvec!( "know", "who", "a", "story", "appeared", "about" ) ;
                                fset!( "know", "who", "Q", "about", "story", "a", "X", "Y", "appeared", "v", "Past" )
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
                    so!(lit!(we())),
                    so!(
                        so!(lit!(Pres())),
                        so!(
                            so!(lit!(we())),
                            so!(
                                so!(lit!(vstar())),
                                so!(
                                    so!(
                                        so!(lit!(know())),
                                        so!(
                                            so!(lit!(who())),
                                            so!(
                                                so!(lit!(Q())),
                                                emb_TP(),
                                            ),
                                        ),
                                    ) =>
                                    fvec!( "know", "who", "a", "story", "appeared", "about" ) ;
                                    fset!( "know", "who", "Q", "about", "story", "a", "X", "Y", "appeared", "v", "Past" )
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
                    so!(lit!(we())),
                    so!(
                        so!(lit!(Pres())),
                        so!(
                            so!(lit!(we())),
                            so!(
                                so!(lit!(vstar())),
                                so!(
                                    so!(
                                        so!(lit!(know())),
                                        so!(
                                            so!(lit!(who())),
                                            so!(
                                                so!(lit!(Q())),
                                                emb_TP(),
                                            ),
                                        ),
                                    ) =>
                                    fvec!( "know", "who", "a", "story", "appeared", "about" ) ;
                                    fset!( "know", "who", "Q", "about", "story", "a", "X", "Y", "appeared", "v", "Past" )
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
                        so!(lit!(we())),
                        so!(
                            so!(lit!(Pres())),
                            so!(
                                so!(lit!(we())),
                                so!(
                                    so!(lit!(vstar())),
                                    so!(
                                        so!(
                                            so!(lit!(know())),
                                            so!(
                                                so!(lit!(who())),
                                                so!(
                                                    so!(lit!(Q())),
                                                    emb_TP(),
                                                ),
                                            ),
                                        ) =>
                                        fvec!( "know", "who", "a", "story", "appeared", "about" ) ;
                                        fset!( "know", "who", "Q", "about", "story", "a", "X", "Y", "appeared", "v", "Past" )
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
                            so!(lit!(we())),
                            so!(
                                so!(lit!(Pres())),
                                so!(
                                    so!(lit!(we())),
                                    so!(
                                        so!(lit!(vstar())),
                                        so!(
                                            so!(
                                                so!(lit!(know())),
                                                so!(
                                                    so!(lit!(who())),
                                                    so!(
                                                        so!(lit!(Q())),
                                                        emb_TP(),
                                                    ),
                                                ),
                                            ) =>
                                            fvec!( "know", "who", "a", "story", "appeared", "about" ) ;
                                            fset!( "know", "who", "Q", "about", "story", "a", "X", "Y", "appeared", "v", "Past" )
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ) =>
                    fvec!(
                        "we", "know", "who", "a", "story", "appeared", "about"
                    ) ;
                    fset!(
                        "who", "about", "story", "a", "X", "Y", "appeared", "v", "Past",
                        "Q", "know", "v*", "we", "Pres", "C"
                    )
                )
            )),
        },
    ];

    assert!(is_derivation(&il, &stages));
}