use std::collections::HashMap;

use crate::deriv::LexicalArray;
use crate::tests::{init_logger, make_word, make_empty};
use crate::{f, fset, fvec, set};
use crate::prelude::*;



/// Chomsky (2001)
/// (4bii)  Several prizes are likely to be awarded.
#[test]
fn test() {
    std::env::set_var("RUST_LOG", "debug");
    init_logger();

    //  Set up word makers

    let prizes = || {
        make_word(
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "pl"),
                synf!(false; "Case"; _)
            ),
            fvec!("prizes")
        )
    };

    let prizes_agreed = || {
        make_word(
            set!(
                synf!("N"),
                synf!(true; "Person"; "3"),
                synf!(true; "Number"; "pl"),
                synf!(false; "Case"; "nom")
            ),
            fvec!("prizes")
        )
    };

    let several = || {
        make_word(
            set!(synf!("Q"), synf!("=N")),
            fvec!("several")
        )
    };

    let D = || {
        make_empty(
            set!(synf!("D"), synf!("=Q")),
            fset!("D")
        )
    };

    let awarded = || {
        make_word(
            set!(synf!("V"), synf!("=D")),
            fvec!("awarded")
        )
    };

    let vpass = || {
        make_empty(
            set!(synf!("v"), synf!("=V")),
            fset!("v")
        )
    };

    let be = || {
        make_word(
            set!(synf!("Aux"), synf!("=v")),
            fvec!("be")
        )
    };

    let def_to = || {
        make_word(
            set!(
                synf!("T"), synf!("=Aux"), synf!(false; "Person"; _),
                synf!("EPP")
            ),
            fvec!("to")
        )
    };

    let def_to_agreed = || {
        make_word(
            set!(
                synf!("T"), synf!("=Aux"), synf!(false; "Person"; "3"),
                synf!("EPP")
            ),
            fvec!("to")
        )
    };

    let likely = || {
        make_word(
            set!(synf!("A"), synf!("=T")),
            fvec!("likely")
        )
    };

    let are = || {
        make_word(
            set!(synf!("Aux"), synf!("=A")),
            fvec!("are")
        )
    };

    let Pres = || {
        make_empty(
            set!(
                synf!("T"), synf!("=Aux"), synf!(false; "Person"; _),
                synf!(false; "Number"; _), synf!(false; "Case"; "nom"),
                synf!("EPP")
            ),
            fset!("Pres")
        )
    };

    let Pres_agreed1 = || {
        make_empty(
            set!(
                synf!("T"), synf!("=Aux"), synf!(false; "Person"; "3"),
                synf!(false; "Number"; _), synf!(false; "Case"; "nom"),
                synf!("EPP")
            ),
            fset!("Pres")
        )
    };

    let Pres_agreed2 = || {
        make_empty(
            set!(
                synf!("T"), synf!("=Aux"), synf!(false; "Person"; "3"),
                synf!(false; "Number"; "pl"), synf!(false; "Case"; "nom"),
                synf!("EPP")
            ),
            fset!("Pres")
        )
    };

    let C = || {
        make_empty(
            set!(synf!("C"), synf!("=T")),
            fset!("C")
        )
    };

    //  Set up I-language

    let lex = set!(
        prizes(),   several(),  D(),        awarded(),  vpass(),    be(),
        def_to(),   likely(),   are(),      Pres(),     C()
    );

    let ug = UniversalGrammar::<BasicTriggers>::new(
        fset!(
            "prizes", "several", "awarded", "be", "to", "likely",
            "are"
        ),
        set!(
            synf!("N"), synf!("Q"), synf!("D"), synf!("V"), synf!("v"),
            synf!("Aux"), synf!("T"), synf!("A"), synf!("C"),
            synf!("=N"), synf!("=Q"), synf!("=D"), synf!("=V"), synf!("=v"),
            synf!("=Aux"), synf!("=T"), synf!("=A"),
            synf!("EPP"),
            synf!("Person"), synf!("Number"), synf!("Case") // lil' sloppy
        ),
        fset!(
            "prizes", "several", "D", "awarded", "v", "be", "to",
            "likely", "are", "Pres", "C"
        )
    );

    let il = ILanguage {
        lex,
        ug,
        realize_map: HashMap::new()
    };

    let stages = vec![
        Stage {
            la: LexicalArray::new(set!(
                lit!(prizes()),
                lit!(several()),
                lit!(D()),
                lit!(awarded()),
                lit!(vpass()),
                lit!(be()),
                lit!(def_to()),
                lit!(likely()),
                lit!(are()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!())
        },

        //  Select prizes
        Stage {
            la: LexicalArray::new(set!(
                lit!(several()),
                lit!(D()),
                lit!(awarded()),
                lit!(vpass()),
                lit!(be()),
                lit!(def_to()),
                lit!(likely()),
                lit!(are()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(prizes()))
            ))
        },

        //  Select several
        Stage {
            la: LexicalArray::new(set!(
                lit!(D()),
                lit!(awarded()),
                lit!(vpass()),
                lit!(be()),
                lit!(def_to()),
                lit!(likely()),
                lit!(are()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(several())),
                so!(lit!(prizes()))
            ))
        },

        //  Merge (several, prizes)
        Stage {
            la: LexicalArray::new(set!(
                lit!(D()),
                lit!(awarded()),
                lit!(vpass()),
                lit!(be()),
                lit!(def_to()),
                lit!(likely()),
                lit!(are()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(several())),
                    so!(lit!(prizes())),
                )
            ))
        },

        //  Select D
        Stage {
            la: LexicalArray::new(set!(
                lit!(awarded()),
                lit!(vpass()),
                lit!(be()),
                lit!(def_to()),
                lit!(likely()),
                lit!(are()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(D())),
                so!(
                    so!(lit!(several())),
                    so!(lit!(prizes())),
                )
            ))
        },

        //  Merge (D, QP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(awarded()),
                lit!(vpass()),
                lit!(be()),
                lit!(def_to()),
                lit!(likely()),
                lit!(are()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(D())),
                    so!(
                        so!(lit!(several())),
                        so!(lit!(prizes())),
                    ),
                )
            ))
        },

        //  Select awarded
        Stage {
            la: LexicalArray::new(set!(
                lit!(vpass()),
                lit!(be()),
                lit!(def_to()),
                lit!(likely()),
                lit!(are()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(awarded())),
                so!(
                    so!(lit!(D())),
                    so!(
                        so!(lit!(several())),
                        so!(lit!(prizes())),
                    ),
                )
            ))
        },

        //  Merge (awarded, VP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(vpass()),
                lit!(be()),
                lit!(def_to()),
                lit!(likely()),
                lit!(are()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(awarded())),
                    so!(
                        so!(lit!(D())),
                        so!(
                            so!(lit!(several())),
                            so!(lit!(prizes())),
                        ),
                    ),
                )
            ))
        },

        //  Select v_pass
        Stage {
            la: LexicalArray::new(set!(
                lit!(be()),
                lit!(def_to()),
                lit!(likely()),
                lit!(are()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(vpass())),
                so!(
                    so!(lit!(awarded())),
                    so!(
                        so!(lit!(D())),
                        so!(
                            so!(lit!(several())),
                            so!(lit!(prizes())),
                        ),
                    ),
                )
            ))
        },

        //  Merge (v_pass, VP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(be()),
                lit!(def_to()),
                lit!(likely()),
                lit!(are()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(vpass())),
                    so!(
                        so!(lit!(awarded())),
                        so!(
                            so!(lit!(D())),
                            so!(
                                so!(lit!(several())),
                                so!(lit!(prizes())),
                            ),
                        ),
                    ),
                )
            ))
        },
    ];

    assert!(is_derivation(&il, &stages));
}