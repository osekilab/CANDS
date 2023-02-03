use std::collections::HashMap;

use crate::deriv::LexicalArray;
use crate::tests::{init_logger, make_word, make_empty};
use crate::{f, fset, fvec, set};
use crate::prelude::*;



/// Chomsky (2001)
/// (4ci)   We expect there to be awarded several prizes.
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
                synf!(false; "Case"; "acc")
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
            set!(synf!("v"), synf!("=V"), synf!("=D")),
            fset!("v")
        )
    };

    let there = || {
        make_word(
            set!(
                synf!("D"),
                // synf!(false; "Person"; _)
                synf!(false; "Person"; "3")
            ),
            fvec!("there")
        )
    };

    //  No `there_agreed` bc it would be the same as `there`

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

    let expect = || {
        make_word(
            set!(synf!("V"), synf!("=T")),
            fvec!("expect")
        )
    };

    let vstar = || {
        make_empty(
            set!(synf!("v*"), synf!("=V"), synf!("=D"), synf!(false; "Person"; _), synf!(false; "Number"; _), synf!(false; "Case"; "acc")),
            fset!("v*")
        )
    };

    let vstar_agreed1 = || {
        make_empty(
            set!(synf!("v*"), synf!("=V"), synf!("=D"), synf!(false; "Person"; "3"), synf!(false; "Number"; _), synf!(false; "Case"; "acc")),
            fset!("v*")
        )
    };

    let vstar_agreed2 = || {
        make_empty(
            set!(synf!("v*"), synf!("=V"), synf!("=D"), synf!(false; "Person"; "3"), synf!(false; "Number"; "pl"), synf!(false; "Case"; "acc")),
            fset!("v*")
        )
    };

    let we = || {
        make_word(
            set!(synf!("D"), synf!(true; "Person"; "1"), synf!(true; "Number"; "pl"), synf!(false; "Case"; _)),
            fvec!("we")
        )
    };

    let we_agreed = || {
        make_word(
            set!(synf!("D"), synf!(true; "Person"; "1"), synf!(true; "Number"; "pl"), synf!(false; "Case"; "nom")),
            fvec!("we")
        )
    };

    let Pres = || {
        make_empty(
            set!(
                synf!("T"), synf!("=v*"), synf!(false; "Person"; _),
                synf!(false; "Number"; _), synf!(false; "Case"; "nom"),
                synf!("EPP")
            ),
            fset!("Pres")
        )
    };

    let Pres_agreed = || {
        make_empty(
            set!(
                synf!("T"), synf!("=v*"), synf!(false; "Person"; "1"),
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
        prizes(),   several(),  D(),        awarded(),  vpass(),    there(),
        be(),       def_to(),   expect(),   vstar(),    we(),       Pres(),
        C()
    );

    let ug = UniversalGrammar::<BasicTriggers>::new(
        fset!(
            "prizes", "several", "awarded", "there", "be", "to", "expect",
            "we"
        ),
        set!(
            synf!("N"), synf!("Q"), synf!("D"), synf!("V"), synf!("v"),
            synf!("Aux"), synf!("T"), synf!("v*"), synf!("C"),
            synf!("=N"), synf!("=Q"), synf!("=D"), synf!("=V"), synf!("=v"),
            synf!("=Aux"), synf!("=T"), synf!("=v*"),
            synf!("EPP"),
            synf!("Person"), synf!("Number"), synf!("Case") // lil' sloppy
        ),
        fset!(
            "prizes", "several", "D", "awarded", "v", "there", "be", "to",
            "v*", "expect", "we", "Pres", "C"
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
                lit!(there()),
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
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
                lit!(there()),
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
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
                lit!(there()),
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
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
                lit!(there()),
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
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
                lit!(there()),
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
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
                lit!(there()),
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
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
                lit!(there()),
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
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
                lit!(there()),
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
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
                lit!(there()),
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
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
                lit!(there()),
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
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

        //  Select there
        Stage {
            la: LexicalArray::new(set!(
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(there())),
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

        //  Merge (there, vP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(be()),
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(there())),
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
                    ),
                )
            ))
        },

        //  Select be
        Stage {
            la: LexicalArray::new(set!(
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(be())),
                so!(
                    so!(lit!(there())),
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
                    ),
                )
            ))
        },

        //  Merge (be, vP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(def_to()),
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(be())),
                    so!(
                        so!(lit!(there())),
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
                        ),
                    ),
                )
            ))
        },

        //  Select T_def
        Stage {
            la: LexicalArray::new(set!(
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(def_to())),
                so!(
                    so!(lit!(be())),
                    so!(
                        so!(lit!(there())),
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
                        ),
                    ),
                )
            ))
        },

        //  Merge (T_def, AuxP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(def_to())),
                    so!(
                        so!(lit!(be())),
                        so!(
                            so!(lit!(there())),
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
                            ),
                        ),
                    ),
                )
            ))
        },

        //  T_def Agrees with there;
        //  there moves to [Spec; T_def]
        Stage {
            la: LexicalArray::new(set!(
                lit!(expect()),
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(there())),
                    so!(
                        so!(lit!(def_to_agreed())),
                        so!(
                            so!(lit!(be())),
                            so!(
                                so!(lit!(there())),
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
                                ),
                            ),
                        ),
                    ),
                )
            ))
        },

        //  Select expect
        Stage {
            la: LexicalArray::new(set!(
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(expect())),
                so!(
                    so!(lit!(there())),
                    so!(
                        so!(lit!(def_to_agreed())),
                        so!(
                            so!(lit!(be())),
                            so!(
                                so!(lit!(there())),
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
                                ),
                            ),
                        ),
                    ),
                )
            ))
        },

        //  Merge (expect, TP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(vstar()),
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(expect())),
                    so!(
                        so!(lit!(there())),
                        so!(
                            so!(lit!(def_to_agreed())),
                            so!(
                                so!(lit!(be())),
                                so!(
                                    so!(lit!(there())),
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
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            ))
        },

        //  Select vstar
        Stage {
            la: LexicalArray::new(set!(
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(vstar())),
                so!(
                    so!(lit!(expect())),
                    so!(
                        so!(lit!(there())),
                        so!(
                            so!(lit!(def_to_agreed())),
                            so!(
                                so!(lit!(be())),
                                so!(
                                    so!(lit!(there())),
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
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            ))
        },

        //  Merge (vstar, VP)
        Stage {
            la: LexicalArray::new(set!(
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(vstar())),
                    so!(
                        so!(lit!(expect())),
                        so!(
                            so!(lit!(there())),
                            so!(
                                so!(lit!(def_to_agreed())),
                                so!(
                                    so!(lit!(be())),
                                    so!(
                                        so!(lit!(there())),
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
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            ))
        },

        //  vstar agrees with there
        Stage {
            la: LexicalArray::new(set!(
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(vstar_agreed1())),
                    so!(
                        so!(lit!(expect())),
                        so!(
                            so!(lit!(there())),
                            so!(
                                so!(lit!(def_to_agreed())),
                                so!(
                                    so!(lit!(be())),
                                    so!(
                                        so!(lit!(there())),
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
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
            ))
        },

        //  vstar agrees with prizes
        Stage {
            la: LexicalArray::new(set!(
                lit!(we()),
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(vstar_agreed2())),
                    so!(
                        so!(lit!(expect())),
                        so!(
                            so!(lit!(there())),
                            so!(
                                so!(lit!(def_to_agreed())),
                                so!(
                                    so!(lit!(be())),
                                    so!(
                                        so!(lit!(there())),
                                        so!(
                                            so!(lit!(vpass())),
                                            so!(
                                                so!(lit!(awarded())),
                                                so!(
                                                    so!(lit!(D())),
                                                    so!(
                                                        so!(lit!(several())),
                                                        so!(lit!(prizes_agreed())),
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
            ))
        },

        //  Select we
        Stage {
            la: LexicalArray::new(set!(
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(lit!(we())),
                so!(
                    so!(lit!(vstar_agreed2())),
                    so!(
                        so!(lit!(expect())),
                        so!(
                            so!(lit!(there())),
                            so!(
                                so!(lit!(def_to_agreed())),
                                so!(
                                    so!(lit!(be())),
                                    so!(
                                        so!(lit!(there())),
                                        so!(
                                            so!(lit!(vpass())),
                                            so!(
                                                so!(lit!(awarded())),
                                                so!(
                                                    so!(lit!(D())),
                                                    so!(
                                                        so!(lit!(several())),
                                                        so!(lit!(prizes_agreed())),
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
            ))
        },

        //  Merge (we, v*P)
        Stage {
            la: LexicalArray::new(set!(
                lit!(Pres()),
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(we())),
                    so!(
                        so!(lit!(vstar_agreed2())),
                        so!(
                            so!(lit!(expect())),
                            so!(
                                so!(lit!(there())),
                                so!(
                                    so!(lit!(def_to_agreed())),
                                    so!(
                                        so!(lit!(be())),
                                        so!(
                                            so!(lit!(there())),
                                            so!(
                                                so!(lit!(vpass())),
                                                so!(
                                                    so!(lit!(awarded())),
                                                    so!(
                                                        so!(lit!(D())),
                                                        so!(
                                                            so!(lit!(several())),
                                                            so!(lit!(prizes_agreed())),
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
            ))
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
                        so!(lit!(vstar_agreed2())),
                        so!(
                            so!(lit!(expect())),
                            so!(
                                so!(lit!(there())),
                                so!(
                                    so!(lit!(def_to_agreed())),
                                    so!(
                                        so!(lit!(be())),
                                        so!(
                                            so!(lit!(there())),
                                            so!(
                                                so!(lit!(vpass())),
                                                so!(
                                                    so!(lit!(awarded())),
                                                    so!(
                                                        so!(lit!(D())),
                                                        so!(
                                                            so!(lit!(several())),
                                                            so!(lit!(prizes_agreed())),
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
            ))
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
                            so!(lit!(vstar_agreed2())),
                            so!(
                                so!(lit!(expect())),
                                so!(
                                    so!(lit!(there())),
                                    so!(
                                        so!(lit!(def_to_agreed())),
                                        so!(
                                            so!(lit!(be())),
                                            so!(
                                                so!(lit!(there())),
                                                so!(
                                                    so!(lit!(vpass())),
                                                    so!(
                                                        so!(lit!(awarded())),
                                                        so!(
                                                            so!(lit!(D())),
                                                            so!(
                                                                so!(lit!(several())),
                                                                so!(lit!(prizes_agreed())),
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
            ))
        },

        //  Pres agrees with we
        Stage {
            la: LexicalArray::new(set!(
                lit!(C())
            )),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(we_agreed())),
                    so!(
                        so!(lit!(Pres_agreed())),
                        so!(
                            so!(lit!(we_agreed())),
                            so!(
                                so!(lit!(vstar_agreed2())),
                                so!(
                                    so!(lit!(expect())),
                                    so!(
                                        so!(lit!(there())),
                                        so!(
                                            so!(lit!(def_to_agreed())),
                                            so!(
                                                so!(lit!(be())),
                                                so!(
                                                    so!(lit!(there())),
                                                    so!(
                                                        so!(lit!(vpass())),
                                                        so!(
                                                            so!(lit!(awarded())),
                                                            so!(
                                                                so!(lit!(D())),
                                                                so!(
                                                                    so!(lit!(several())),
                                                                    so!(lit!(prizes_agreed())),
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
            ))
        },

        //  Select C
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(lit!(C())),
                so!(
                    so!(lit!(we_agreed())),
                    so!(
                        so!(lit!(Pres_agreed())),
                        so!(
                            so!(lit!(we_agreed())),
                            so!(
                                so!(lit!(vstar_agreed2())),
                                so!(
                                    so!(lit!(expect())),
                                    so!(
                                        so!(lit!(there())),
                                        so!(
                                            so!(lit!(def_to_agreed())),
                                            so!(
                                                so!(lit!(be())),
                                                so!(
                                                    so!(lit!(there())),
                                                    so!(
                                                        so!(lit!(vpass())),
                                                        so!(
                                                            so!(lit!(awarded())),
                                                            so!(
                                                                so!(lit!(D())),
                                                                so!(
                                                                    so!(lit!(several())),
                                                                    so!(lit!(prizes_agreed())),
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
            ))
        },

        //  Merge (C, TP)
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(lit!(C())),
                    so!(
                        so!(lit!(we_agreed())),
                        so!(
                            so!(lit!(Pres_agreed())),
                            so!(
                                so!(lit!(we_agreed())),
                                so!(
                                    so!(lit!(vstar_agreed2())),
                                    so!(
                                        so!(lit!(expect())),
                                        so!(
                                            so!(lit!(there())),
                                            so!(
                                                so!(lit!(def_to_agreed())),
                                                so!(
                                                    so!(lit!(be())),
                                                    so!(
                                                        so!(lit!(there())),
                                                        so!(
                                                            so!(lit!(vpass())),
                                                            so!(
                                                                so!(lit!(awarded())),
                                                                so!(
                                                                    so!(lit!(D())),
                                                                    so!(
                                                                        so!(lit!(several())),
                                                                        so!(lit!(prizes_agreed())),
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
                    ),
                )
            ))
        },

        //  Transfer (CP, CP)
        Stage {
            la: LexicalArray::new(set!()),
            w: Workspace::new(set!(
                so!(
                    so!(
                        so!(lit!(C())),
                        so!(
                            so!(lit!(we_agreed())),
                            so!(
                                so!(lit!(Pres_agreed())),
                                so!(
                                    so!(lit!(we_agreed())),
                                    so!(
                                        so!(lit!(vstar_agreed2())),
                                        so!(
                                            so!(lit!(expect())),
                                            so!(
                                                so!(lit!(there())),
                                                so!(
                                                    so!(lit!(def_to_agreed())),
                                                    so!(
                                                        so!(lit!(be())),
                                                        so!(
                                                            so!(lit!(there())),
                                                            so!(
                                                                so!(lit!(vpass())),
                                                                so!(
                                                                    so!(lit!(awarded())),
                                                                    so!(
                                                                        so!(lit!(D())),
                                                                        so!(
                                                                            so!(lit!(several())),
                                                                            so!(lit!(prizes_agreed())),
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
                        ),
                    ) =>
                    fvec!(
                        "we", "expect", "there", "to", "be", "awarded",
                        "several", "prizes"
                    ) ;
                    fset!(
                        "prizes", "several", "D", "awarded", "v", "there",
                        "be", "to", "v*", "expect", "we", "Pres", "C"
                    )
                )
            ))
        },
    ];

    assert!(is_derivation(&il, &stages));
}