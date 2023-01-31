//! A bunch of tests.

#[cfg(test)]
mod macros;

#[cfg(test)]
mod basic_select_tests {
    //  use crate::*;

    #[test]
    fn select_word() {

    }
}

#[cfg(test)]
mod derivck_tests {
    use std::collections::HashMap;

    use crate::deriv::LexicalArray;
    use crate::{f, fset, fvec, set};
    use crate::prelude::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test3() {
        std::env::set_var("RUST_LOG", "debug");
        init();

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

    #[test]
    fn test2() {
        std::env::set_var("RUST_LOG", "debug");
        init();

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

    #[test]
    fn test1() {
        init();

        let lex = set!(
            li!("Mary"; "D"; "Mary"),
            li!("dances"; "V"; "dances"),
            li!("v*"; "v*", "=V", "=D"; ""),
            li!("PRES"; "T", "=v*"; ""),
            li!("C"; "C", "=T"; "")
        );
    
        let ug = UniversalGrammar::<BasicTriggers>::new(
            fset!("Mary", "dances", ""),
            synfset!("D", "V", "v*", "T", "C", "=D", "=V", "=v*", "=T"),
            fset!("Mary", "dances", "v*", "PRES", "C")
        );
    
        let il = ILanguage {
            lex,
            ug,
            realize_map: HashMap::new()
        };

        let stages = vec![
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("Mary"; "D"; "Mary"), 1),
                    lit!(li!("dances"; "V"; "dances"), 2),
                    lit!(li!("v*"; "v*", "=V", "=D"; ""), 3),
                    lit!(li!("PRES"; "T", "=v*"; ""), 4),
                    lit!(li!("C"; "C", "=T"; ""), 5)
                )),
                w: Workspace::new(set!())
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("Mary"; "D"; "Mary"), 1),
                    lit!(li!("v*"; "v*", "=V", "=D"; ""), 3),
                    lit!(li!("PRES"; "T", "=v*"; ""), 4),
                    lit!(li!("C"; "C", "=T"; ""), 5)
                )),
                w: Workspace::new(set!(
                    so!(lit!(li!("dances"; "V"; "dances"), 2))
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("Mary"; "D"; "Mary"), 1),
                    lit!(li!("PRES"; "T", "=v*"; ""), 4),
                    lit!(li!("C"; "C", "=T"; ""), 5)
                )),
                w: Workspace::new(set!(
                    so!(lit!(li!("dances"; "V"; "dances"), 2)),
                    so!(lit!(li!("v*"; "v*", "=V", "=D"; ""), 3))
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("Mary"; "D"; "Mary"), 1),
                    lit!(li!("PRES"; "T", "=v*"; ""), 4),
                    lit!(li!("C"; "C", "=T"; ""), 5)
                )),
                w: Workspace::new(set!(
                    so!(
                        so!(lit!(li!("dances"; "V"; "dances"), 2)),
                        so!(lit!(li!("v*"; "v*", "=V", "=D"; ""), 3)),
                    )
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("PRES"; "T", "=v*"; ""), 4),
                    lit!(li!("C"; "C", "=T"; ""), 5)
                )),
                w: Workspace::new(set!(
                    so!(
                        so!(lit!(li!("dances"; "V"; "dances"), 2)),
                        so!(lit!(li!("v*"; "v*", "=V", "=D"; ""), 3)),
                    ),
                    so!(lit!(li!("Mary"; "D"; "Mary"), 1))
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("PRES"; "T", "=v*"; ""), 4),
                    lit!(li!("C"; "C", "=T"; ""), 5)
                )),
                w: Workspace::new(set!(
                    so!(
                        so!(
                            so!(lit!(li!("dances"; "V"; "dances"), 2)),
                            so!(lit!(li!("v*"; "v*", "=V", "=D"; ""), 3)),
                        ),
                        so!(lit!(li!("Mary"; "D"; "Mary"), 1)),
                    )
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("C"; "C", "=T"; ""), 5)
                )),
                w: Workspace::new(set!(
                    so!(lit!(li!("PRES"; "T", "=v*"; ""), 4)),
                    so!(
                        so!(
                            so!(lit!(li!("dances"; "V"; "dances"), 2)),
                            so!(lit!(li!("v*"; "v*", "=V", "=D"; ""), 3)),
                        ),
                        so!(lit!(li!("Mary"; "D"; "Mary"), 1)),
                    )
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("C"; "C", "=T"; ""), 5)
                )),
                w: Workspace::new(set!(
                    so!(
                        so!(lit!(li!("PRES"; "T", "=v*"; ""), 4)),
                        so!(
                            so!(
                                so!(lit!(li!("dances"; "V"; "dances"), 2)),
                                so!(lit!(li!("v*"; "v*", "=V", "=D"; ""), 3)),
                            ),
                            so!(lit!(li!("Mary"; "D"; "Mary"), 1)),
                        ),
                    )
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!()),
                w: Workspace::new(set!(
                    so!(lit!(li!("C"; "C", "=T"; ""), 5)),
                    so!(
                        so!(lit!(li!("PRES"; "T", "=v*"; ""), 4)),
                        so!(
                            so!(
                                so!(lit!(li!("dances"; "V"; "dances"), 2)),
                                so!(lit!(li!("v*"; "v*", "=V", "=D"; ""), 3)),
                            ),
                            so!(lit!(li!("Mary"; "D"; "Mary"), 1)),
                        ),
                    )
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!()),
                w: Workspace::new(set!(
                    so!(
                        so!(lit!(li!("C"; "C", "=T"; ""), 5)),
                        so!(
                            so!(lit!(li!("PRES"; "T", "=v*"; ""), 4)),
                            so!(
                                so!(
                                    so!(lit!(li!("dances"; "V"; "dances"), 2)),
                                    so!(lit!(li!("v*"; "v*", "=V", "=D"; ""), 3)),
                                ),
                                so!(lit!(li!("Mary"; "D"; "Mary"), 1)),
                            ),
                        ),
                    )
                ))
            },
        ];

        // std::env::set_var("RUST_LOG", "info");
        assert!(is_derivation(&il, &stages));
    }
}