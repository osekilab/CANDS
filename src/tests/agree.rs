use std::collections::HashMap;

use crate::deriv::LexicalArray;
use crate::tests::init_logger;
use crate::{f, fset, fvec, set};
use crate::prelude::*;



/// [ v* [ [ D N1 ] [ V [ D N2 ] ] ]], where v* Agrees with N2.
/// N1 is sg, N2 is pl.  v* gets pl after Agree.
#[test]
fn test5() {
    std::env::set_var("RUST_LOG", "debug");
    init_logger();

    let make_vstar = || {
        LexicalItem::new(
            fset!("v*"),
            set!(
                synf!("v*"),
                synf!("=V"),
                synf!(false; "Person"; _),
                synf!(false; "Number"; _),
                synf!(false; "Case"; "acc")
            ),
            fvec!(""),
            None
        )
    };

    let make_vstar_agreed = || {
        LexicalItem::new(
            fset!("v*"),
            set!(
                synf!("v*"),
                synf!("=V"),
                synf!(false; "Person"; "3"),
                synf!(false; "Number"; "pl"),
                synf!(false; "Case"; "acc")
            ),
            fvec!(""),
            None
        )
    };

    let make_student = || {
        LexicalItem::new(
            fset!("student"),
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "sg"),
                synf!(false; "Case"; _)
            ),
            fvec!("student"),
            None
        )
    };

    let make_letters = || {
        LexicalItem::new(
            fset!("letters"),
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "pl"),
                synf!(false; "Case"; _)
            ),
            fvec!("letters"),
            None
        )
    };

    let make_letters_agreed = || {
        LexicalItem::new(
            fset!("letters"),
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "pl"),
                synf!(false; "Case"; "acc")
            ),
            fvec!("letters"),
            None
        )
    };

    let lex = set!(
        make_vstar(),
        li!("sent"; "V", "=D1", "=D2"; "sent"),
        li!("the"; "D1", "=N"; "the"),  // Hacks! but C&S doesn't allow true ditransitive verbs
        li!("the"; "D2", "=N"; "the"),
        make_student(),
        make_letters()
    );

    let ug = UniversalGrammar::<BasicTriggers>::new(
        fset!("sent", "the", "student", "letters", ""),
        set!(
            synf!("N"), synf!("D1"), synf!("D2"), synf!("V"), synf!("v*"), synf!("=N"), synf!("=D1"), synf!("=D2"), synf!("=V"),
            synf!("Person"), synf!("Number"), synf!("Case") // lil' sloppy
        ),
        fset!("v*", "sent", "the", "student", "letters")
    );

    let il = ILanguage {
        lex,
        ug,
        realize_map: HashMap::new()
    };

    let stages = vec![
        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1),
                lit!(li!("the"; "D1", "=N"; "the"), 1),
                lit!(make_student(), 1),
                lit!(li!("the"; "D2", "=N"; "the"), 2),
                lit!(make_letters(), 2)
            )),
            w: Workspace::new(set!()),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1),
                lit!(li!("the"; "D1", "=N"; "the"), 1),
                lit!(make_student(), 1),
                lit!(make_letters(), 2)
            )),
            w: Workspace::new(set!(
                so!(lit!(li!("the"; "D2", "=N"; "the"), 2))
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1),
                lit!(li!("the"; "D1", "=N"; "the"), 1),
                lit!(make_student(), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                so!(lit!(make_letters(), 2))
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1),
                lit!(li!("the"; "D1", "=N"; "the"), 1),
                lit!(make_student(), 1)
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                    so!(lit!(make_letters(), 2)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1),
                lit!(make_student(), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                so!(
                    so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                    so!(lit!(make_letters(), 2)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                so!(lit!(make_student(), 1)),
                so!(
                    so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                    so!(lit!(make_letters(), 2)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                    so!(lit!(make_student(), 1)),
                ),
                so!(
                    so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                    so!(lit!(make_letters(), 2)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                so!(
                    so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                    so!(lit!(make_student(), 1)),
                ),
                so!(
                    so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                    so!(lit!(make_letters(), 2)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1)
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                    so!(
                        so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                        so!(lit!(make_letters(), 2)),
                    ),
                ),
                so!(
                    so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                    so!(lit!(make_student(), 1)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1)
            )),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                        so!(lit!(make_student(), 1)),
                    ),
                    so!(
                        so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                        so!(
                            so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                            so!(lit!(make_letters(), 2)),
                        ),
                    ),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(lit!(make_vstar(), 1)),
                so!(
                    so!(
                        so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                        so!(lit!(make_student(), 1)),
                    ),
                    so!(
                        so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                        so!(
                            so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                            so!(lit!(make_letters(), 2)),
                        ),
                    ),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(make_vstar(), 1)),
                    so!(
                        so!(
                            so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                            so!(lit!(make_student(), 1)),
                        ),
                        so!(
                            so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                            so!(
                                so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                                so!(lit!(make_letters(), 2)),
                            ),
                        ),
                    ),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(make_vstar_agreed(), 1)),
                    so!(
                        so!(
                            so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                            so!(lit!(make_student(), 1)),
                        ),
                        so!(
                            so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                            so!(
                                so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                                so!(lit!(make_letters_agreed(), 2)),
                            ),
                        ),
                    ),
                )
            )),
        },
    ];

    assert!(is_derivation(&il, &stages));
}



/// [ v* [ [ D1 N1 ] [ V [ D2 N2 ] ] ]], where v* Agrees with N1.
/// N1 is sg, N2 is pl.  v* gets sg after Agree.
#[test]
fn test4() {
    std::env::set_var("RUST_LOG", "debug");
    init_logger();

    let make_vstar = || {
        LexicalItem::new(
            fset!("v*"),
            set!(
                synf!("v*"),
                synf!("=V"),
                synf!(false; "Person"; _),
                synf!(false; "Number"; _),
                synf!(false; "Case"; "acc")
            ),
            fvec!(""),
            None
        )
    };

    let make_vstar_agreed = || {
        LexicalItem::new(
            fset!("v*"),
            set!(
                synf!("v*"),
                synf!("=V"),
                synf!(false; "Person"; "3"),
                synf!(false; "Number"; "sg"),
                synf!(false; "Case"; "acc")
            ),
            fvec!(""),
            None
        )
    };

    let make_student = || {
        LexicalItem::new(
            fset!("student"),
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "sg"),
                synf!(false; "Case"; _)
            ),
            fvec!("student"),
            None
        )
    };

    let make_student_agreed = || {
        LexicalItem::new(
            fset!("student"),
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "sg"),
                synf!(false; "Case"; "acc")
            ),
            fvec!("student"),
            None
        )
    };

    let make_letters = || {
        LexicalItem::new(
            fset!("letters"),
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "pl"),
                synf!(false; "Case"; _)
            ),
            fvec!("letters"),
            None
        )
    };

    let lex = set!(
        make_vstar(),
        li!("sent"; "V", "=D1", "=D2"; "sent"),
        li!("the"; "D1", "=N"; "the"),  // Hacks! but C&S doesn't allow true ditransitive verbs
        li!("the"; "D2", "=N"; "the"),
        make_student(),
        make_letters()
    );

    let ug = UniversalGrammar::<BasicTriggers>::new(
        fset!("sent", "the", "student", "letters", ""),
        set!(
            synf!("N"), synf!("D1"), synf!("D2"), synf!("V"), synf!("v*"), synf!("=N"), synf!("=D1"), synf!("=D2"), synf!("=V"),
            synf!("Person"), synf!("Number"), synf!("Case") // lil' sloppy
        ),
        fset!("v*", "sent", "the", "student", "letters")
    );

    let il = ILanguage {
        lex,
        ug,
        realize_map: HashMap::new()
    };

    let stages = vec![
        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1),
                lit!(li!("the"; "D1", "=N"; "the"), 1),
                lit!(make_student(), 1),
                lit!(li!("the"; "D2", "=N"; "the"), 2),
                lit!(make_letters(), 2)
            )),
            w: Workspace::new(set!()),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1),
                lit!(li!("the"; "D1", "=N"; "the"), 1),
                lit!(make_student(), 1),
                lit!(make_letters(), 2)
            )),
            w: Workspace::new(set!(
                so!(lit!(li!("the"; "D2", "=N"; "the"), 2))
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1),
                lit!(li!("the"; "D1", "=N"; "the"), 1),
                lit!(make_student(), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                so!(lit!(make_letters(), 2))
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1),
                lit!(li!("the"; "D1", "=N"; "the"), 1),
                lit!(make_student(), 1)
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                    so!(lit!(make_letters(), 2)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1),
                lit!(make_student(), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                so!(
                    so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                    so!(lit!(make_letters(), 2)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                so!(lit!(make_student(), 1)),
                so!(
                    so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                    so!(lit!(make_letters(), 2)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                    so!(lit!(make_student(), 1)),
                ),
                so!(
                    so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                    so!(lit!(make_letters(), 2)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                so!(
                    so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                    so!(lit!(make_student(), 1)),
                ),
                so!(
                    so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                    so!(lit!(make_letters(), 2)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1)
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                    so!(
                        so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                        so!(lit!(make_letters(), 2)),
                    ),
                ),
                so!(
                    so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                    so!(lit!(make_student(), 1)),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1)
            )),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                        so!(lit!(make_student(), 1)),
                    ),
                    so!(
                        so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                        so!(
                            so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                            so!(lit!(make_letters(), 2)),
                        ),
                    ),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(lit!(make_vstar(), 1)),
                so!(
                    so!(
                        so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                        so!(lit!(make_student(), 1)),
                    ),
                    so!(
                        so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                        so!(
                            so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                            so!(lit!(make_letters(), 2)),
                        ),
                    ),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(make_vstar(), 1)),
                    so!(
                        so!(
                            so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                            so!(lit!(make_student(), 1)),
                        ),
                        so!(
                            so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                            so!(
                                so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                                so!(lit!(make_letters(), 2)),
                            ),
                        ),
                    ),
                )
            )),
        },

        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(make_vstar_agreed(), 1)),
                    so!(
                        so!(
                            so!(lit!(li!("the"; "D1", "=N"; "the"), 1)),
                            so!(lit!(make_student_agreed(), 1)),
                        ),
                        so!(
                            so!(lit!(li!("sent"; "V", "=D1", "=D2"; "sent"), 1)),
                            so!(
                                so!(lit!(li!("the"; "D2", "=N"; "the"), 2)),
                                so!(lit!(make_letters(), 2)),
                            ),
                        ),
                    ),
                )
            )),
        },
    ];

    assert!(is_derivation(&il, &stages));
}



/// [ v* [ V object ] ], where v* Agrees with the object.
/// v* has EPP, so the object moves to [Spec; v*].
#[test]
fn test3() {
    std::env::set_var("RUST_LOG", "debug");
    init_logger();

    let make_vstar = || {
        LexicalItem::new(
            fset!("v*"),
            set!(
                synf!("v*"),
                synf!("=V"),
                synf!("EPP"),
                synf!(false; "Person"; _),
                synf!(false; "Number"; _),
                synf!(false; "Case"; "acc")
            ),
            fvec!(""),
            None
        )
    };

    let make_vstar_agreed = || {
        LexicalItem::new(
            fset!("v*"),
            set!(
                synf!("v*"),
                synf!("=V"),
                synf!("EPP"),
                synf!(false; "Person"; "3"),
                synf!(false; "Number"; "sg"),
                synf!(false; "Case"; "acc")
            ),
            fvec!(""),
            None
        )
    };

    let make_vases = || {
        LexicalItem::new(
            fset!("vases"),
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "sg"),
                synf!(false; "Case"; _)
            ),
            fvec!("vases"),
            None
        )
    };

    let make_vases_agreed = || {
        LexicalItem::new(
            fset!("vases"),
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "sg"),
                synf!(false; "Case"; "acc")
            ),
            fvec!("vases"),
            None
        )
    };

    let lex = set!(
        make_vstar(),
        li!("broke"; "V", "=N"; "broke"),
        make_vases()
    );

    let ug = UniversalGrammar::<BasicTriggers>::new(
        fset!("broke", "vases", ""),
        set!(
            synf!("N"), synf!("V"), synf!("v*"), synf!("=N"), synf!("=V"), synf!("EPP"),
            synf!("Person"), synf!("Number"), synf!("Case") // lil' sloppy
        ),
        fset!("v*", "broke", "vases")
    );

    let il = ILanguage {
        lex,
        ug,
        realize_map: HashMap::new()
    };

    let stages = vec![
        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("broke"; "V", "=N"; "broke"), 1),
                lit!(make_vases(), 1)
            )),
            w: Workspace::new(set!())
        },
        
        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("broke"; "V", "=N"; "broke"), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(make_vases(), 1))
            ))
        },
        
        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(make_vases(), 1)),
                so!(lit!(li!("broke"; "V", "=N"; "broke"), 1))
            ))
        },
        
        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1)
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(li!("broke"; "V", "=N"; "broke"), 1)),
                    so!(lit!(make_vases(), 1)),
                )
            ))
        },
        
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(lit!(make_vstar(), 1)),
                so!(
                    so!(lit!(li!("broke"; "V", "=N"; "broke"), 1)),
                    so!(lit!(make_vases(), 1)),
                )
            ))
        },
        
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(make_vstar(), 1)),
                    so!(
                        so!(lit!(li!("broke"; "V", "=N"; "broke"), 1)),
                        so!(lit!(make_vases(), 1)),
                    ),
                )
            ))
        },
        
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(make_vases_agreed(), 1)),
                    so!(
                        so!(lit!(make_vstar_agreed(), 1)),
                        so!(
                            so!(lit!(li!("broke"; "V", "=N"; "broke"), 1)),
                            so!(lit!(make_vases_agreed(), 1)),
                        ),
                    ),
                )
            ))
        },
    ];

    assert!(is_derivation(&il, &stages));
}



/// [ v* [ V object ] ], where v* Agrees with the object.
#[test]
fn test2() {
    std::env::set_var("RUST_LOG", "debug");
    init_logger();

    let make_vstar = || {
        LexicalItem::new(
            fset!("v*"),
            set!(
                synf!("v*"),
                synf!("=V"),
                synf!(false; "Person"; _),
                synf!(false; "Number"; _),
                synf!(false; "Case"; "acc")
            ),
            fvec!(""),
            None
        )
    };

    let make_vstar_agreed = || {
        LexicalItem::new(
            fset!("v*"),
            set!(
                synf!("v*"),
                synf!("=V"),
                synf!(false; "Person"; "3"),
                synf!(false; "Number"; "sg"),
                synf!(false; "Case"; "acc")
            ),
            fvec!(""),
            None
        )
    };

    let make_vases = || {
        LexicalItem::new(
            fset!("vases"),
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "sg"),
                synf!(false; "Case"; _)
            ),
            fvec!("vases"),
            None
        )
    };

    let make_vases_agreed = || {
        LexicalItem::new(
            fset!("vases"),
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "sg"),
                synf!(false; "Case"; "acc")
            ),
            fvec!("vases"),
            None
        )
    };

    let lex = set!(
        make_vstar(),
        li!("broke"; "V", "=N"; "broke"),
        make_vases()
    );

    let ug = UniversalGrammar::<BasicTriggers>::new(
        fset!("broke", "vases", ""),
        set!(
            synf!("N"), synf!("V"), synf!("v*"), synf!("=N"), synf!("=V"),
            synf!("Person"), synf!("Number"), synf!("Case") // lil' sloppy
        ),
        fset!("v*", "broke", "vases")
    );

    let il = ILanguage {
        lex,
        ug,
        realize_map: HashMap::new()
    };

    let stages = vec![
        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("broke"; "V", "=N"; "broke"), 1),
                lit!(make_vases(), 1)
            )),
            w: Workspace::new(set!())
        },
        
        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1),
                lit!(li!("broke"; "V", "=N"; "broke"), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(make_vases(), 1))
            ))
        },
        
        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1)
            )),
            w: Workspace::new(set!(
                so!(lit!(make_vases(), 1)),
                so!(lit!(li!("broke"; "V", "=N"; "broke"), 1))
            ))
        },
        
        Stage {
            la: LexicalArray::new(set!(
                lit!(make_vstar(), 1)
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(li!("broke"; "V", "=N"; "broke"), 1)),
                    so!(lit!(make_vases(), 1)),
                )
            ))
        },
        
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(lit!(make_vstar(), 1)),
                so!(
                    so!(lit!(li!("broke"; "V", "=N"; "broke"), 1)),
                    so!(lit!(make_vases(), 1)),
                )
            ))
        },
        
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(make_vstar(), 1)),
                    so!(
                        so!(lit!(li!("broke"; "V", "=N"; "broke"), 1)),
                        so!(lit!(make_vases(), 1)),
                    ),
                )
            ))
        },
        
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(make_vstar_agreed(), 1)),
                    so!(
                        so!(lit!(li!("broke"; "V", "=N"; "broke"), 1)),
                        so!(lit!(make_vases_agreed(), 1)),
                    ),
                )
            ))
        },
    ];

    assert!(is_derivation(&il, &stages));
}