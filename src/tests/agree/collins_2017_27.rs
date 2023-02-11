use crate::{tests::{init_logger, make_word, make_empty}};
use crate::prelude::*;

/// Collins (2017)
/// (27) The man falls.
#[test]
fn test() {
    std::env::set_var("RUST_LOG", "debug");
    init_logger();

    //  Set up word makers
    
    let the = || {
        make_word(
            synfset!("D", "=N"),
            fvec!("the")
        )
    };

    let Person_3 = |i| synf!(i; "Person"; "3");
    let Number_sg = |i| synf!(i; "Number"; "sg");
    let Case_nom = |i| synf!(i; "Case"; "nom");

    let man = || {
        make_word(
            set!(
                synf!("N"),
                Person_3(true),
                Number_sg(true),
                Case_nom(false)
            ),
            fvec!("man")
        )
    };

    let man_agreed = || {
        so!(
            so!(lit!(man()));
            Case_nom(true)
        )
    };

    let falls = || {
        make_word(
            synfset!("V", "=D"),
            fvec!("falls")
        )
    };

    let v = || {
        make_empty(
            synfset!("v", "=V"),
            fset!("v")
        )
    };

    let Pres = || {
        make_empty(
            set!(
                synf!("T"),
                synf!("=v"),
                synf!("EPP"),
                Person_3(false),
                Number_sg(false),
                Case_nom(true)
            ),
            fset!("Pres")
        )
    };

    let Pres_agreed = || {
        so!(
            so!(
                so!(lit!(Pres()));
                Person_3(true)
            );
            Number_sg(true)
        )
    };

    let C = || {
        make_empty(
            synfset!("C", "=T"),
            fset!("C")
        )
    };

    //  Set up I-language

    let lex = set!(
        man(), the(), falls(), v(), Pres(), C()
    );

    let ug = UniversalGrammar::<BasicTriggers>::new(
        fset!("man", "the", "falls"),
        set!(
            synf!("N"), synf!("D"), synf!("V"), synf!("v"), synf!("T"), synf!("C"),
            synf!("=N"), synf!("=D"), synf!("=V"), synf!("=v"), synf!("=T"),
            synf!("EPP"),
            synf!("Person"), synf!("Number"), synf!("Case") // lil' sloppy
        ),
        fset!("man", "the", "falls", "v", "Pres", "C")
    );

    let il = ILanguage { lex, ug };

    let stages = vec![
        Stage {
            la: LexicalArray::new(set!(
                lit!(man()),
                lit!(the()),
                lit!(falls()),
                lit!(v()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!()),
        },

        //  Select man
        Stage {
            la: LexicalArray::new(set!(
                lit!(the()),
                lit!(falls()),
                lit!(v()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(man()))
            )),
        },

        //  Select Pres
        Stage {
            la: LexicalArray::new(set!(
                lit!(the()),
                lit!(falls()),
                lit!(v()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Pres())),
                so!(lit!(man()))
            )),
        },

        //  Merge (iCase, man)
        Stage {
            la: LexicalArray::new(set!(
                lit!(the()),
                lit!(falls()),
                lit!(v()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Pres())),
                man_agreed()
            )),
        },

        //  Select the
        Stage {
            la: LexicalArray::new(set!(
                lit!(falls()),
                lit!(v()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Pres())),
                so!(lit!(the())),
                man_agreed()
            )),
        },

        //  Merge (the, man')
        Stage {
            la: LexicalArray::new(set!(
                lit!(falls()),
                lit!(v()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Pres())),
                so!(
                    so!(lit!(the())),
                    man_agreed(),
                )
            )),
        },

        //  Select V
        Stage {
            la: LexicalArray::new(set!(
                lit!(v()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Pres())),
                so!(lit!(falls())),
                so!(
                    so!(lit!(the())),
                    man_agreed(),
                )
            )),
        },

        //  Merge (V, DP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(v()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Pres())),
                so!(
                    so!(lit!(falls())),
                    so!(
                        so!(lit!(the())),
                        man_agreed(),
                    ),
                )
            )),
        },

        //  Select v
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Pres())),
                so!(lit!(v())),
                so!(
                    so!(lit!(falls())),
                    so!(
                        so!(lit!(the())),
                        man_agreed(),
                    ),
                )
            )),
        },

        //  Merge (v, VP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(Pres())),
                so!(
                    so!(lit!(v())),
                    so!(
                        so!(lit!(falls())),
                        so!(
                            so!(lit!(the())),
                            man_agreed(),
                        ),
                    ),
                )
            )),
        },

        //  Merge (Pres', vP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(Pres())),
                    so!(
                        so!(lit!(v())),
                        so!(
                            so!(lit!(falls())),
                            so!(
                                so!(lit!(the())),
                                man_agreed(),
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
                        so!(lit!(the())),
                        man_agreed(),
                    ),
                    so!(
                        so!(lit!(Pres())),
                        so!(
                            so!(lit!(v())),
                            so!(
                                so!(lit!(falls())),
                                so!(
                                    so!(lit!(the())),
                                    man_agreed(),
                                ),
                            ),
                        ),
                    ),
                )
            )),
        },

        //  Merge (TP, iPerson)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(
                            so!(lit!(the())),
                            man_agreed(),
                        ),
                        so!(
                            so!(lit!(Pres())),
                            so!(
                                so!(lit!(v())),
                                so!(
                                    so!(lit!(falls())),
                                    so!(
                                        so!(lit!(the())),
                                        man_agreed(),
                                    ),
                                ),
                            ),
                        ),
                    );
                    Person_3(true)
                )
            )),
        },

        //  Merge (TP, iNumber)
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(
                            so!(
                                so!(lit!(the())),
                                man_agreed(),
                            ),
                            so!(
                                so!(lit!(Pres())),
                                so!(
                                    so!(lit!(v())),
                                    so!(
                                        so!(lit!(falls())),
                                        so!(
                                            so!(lit!(the())),
                                            man_agreed(),
                                        ),
                                    ),
                                ),
                            ),
                        );
                        Person_3(true)
                    );
                    Number_sg(true)
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
                        so!(
                            so!(
                                so!(lit!(the())),
                                man_agreed(),
                            ),
                            so!(
                                so!(lit!(Pres())),
                                so!(
                                    so!(lit!(v())),
                                    so!(
                                        so!(lit!(falls())),
                                        so!(
                                            so!(lit!(the())),
                                            man_agreed(),
                                        ),
                                    ),
                                ),
                            ),
                        );
                        Person_3(true)
                    );
                    Number_sg(true)
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
                            so!(
                                so!(
                                    so!(lit!(the())),
                                    man_agreed(),
                                ),
                                so!(
                                    so!(lit!(Pres())),
                                    so!(
                                        so!(lit!(v())),
                                        so!(
                                            so!(lit!(falls())),
                                            so!(
                                                so!(lit!(the())),
                                                man_agreed(),
                                            ),
                                        ),
                                    ),
                                ),
                            );
                            Person_3(true)
                        );
                        Number_sg(true)
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
                                so!(
                                    so!(
                                        so!(lit!(the())),
                                        man_agreed(),
                                    ),
                                    so!(
                                        so!(lit!(Pres())),
                                        so!(
                                            so!(lit!(v())),
                                            so!(
                                                so!(lit!(falls())),
                                                so!(
                                                    so!(lit!(the())),
                                                    man_agreed(),
                                                ),
                                            ),
                                        ),
                                    ),
                                );
                                Person_3(true)
                            );
                            Number_sg(true)
                        ),
                    ) =>
                    fvec!("the", "man", "falls") ;
                    fset!("man", "the", "falls", "v", "Pres", "C")
                )
            )),
        },
    ];

    assert!(is_derivation(&il, &stages));
}