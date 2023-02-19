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
    use crate::deriv::LexicalArray;
    use crate::{f, fset, fvec, set};
    use crate::prelude::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test1() {
        init();

        let lex = set!(
            li!("Mary"; "D"; "Mary"),
            li!("dances"; "V"; "dances"),
            li!("v*"; "v*", "=V", "=D";),
            li!("PRES"; "T", "=v*";),
            li!("C"; "C", "=T";)
        );
    
        let ug = UniversalGrammar::<BasicTriggers>::new(
            fset!("Mary", "dances", ""),
            fset!("D", "V", "v*", "T", "C", "=D", "=V", "=v*", "=T"),
            fset!("Mary", "dances", "v*", "PRES", "C")
        );
    
        let il = ILanguage {
            lex,
            ug,
        };

        let stages = vec![
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("Mary"; "D"; "Mary"), 1),
                    lit!(li!("dances"; "V"; "dances"), 2),
                    lit!(li!("v*"; "v*", "=V", "=D";), 3),
                    lit!(li!("PRES"; "T", "=v*";), 4),
                    lit!(li!("C"; "C", "=T";), 5)
                )),
                w: Workspace::new(set!())
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("Mary"; "D"; "Mary"), 1),
                    lit!(li!("v*"; "v*", "=V", "=D";), 3),
                    lit!(li!("PRES"; "T", "=v*";), 4),
                    lit!(li!("C"; "C", "=T";), 5)
                )),
                w: Workspace::new(set!(
                    so!(lit!(li!("dances"; "V"; "dances"), 2))
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("Mary"; "D"; "Mary"), 1),
                    lit!(li!("PRES"; "T", "=v*";), 4),
                    lit!(li!("C"; "C", "=T";), 5)
                )),
                w: Workspace::new(set!(
                    so!(lit!(li!("dances"; "V"; "dances"), 2)),
                    so!(lit!(li!("v*"; "v*", "=V", "=D";), 3))
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("Mary"; "D"; "Mary"), 1),
                    lit!(li!("PRES"; "T", "=v*";), 4),
                    lit!(li!("C"; "C", "=T";), 5)
                )),
                w: Workspace::new(set!(
                    so!(
                        so!(lit!(li!("dances"; "V"; "dances"), 2)),
                        so!(lit!(li!("v*"; "v*", "=V", "=D";), 3)),
                    )
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("PRES"; "T", "=v*";), 4),
                    lit!(li!("C"; "C", "=T";), 5)
                )),
                w: Workspace::new(set!(
                    so!(
                        so!(lit!(li!("dances"; "V"; "dances"), 2)),
                        so!(lit!(li!("v*"; "v*", "=V", "=D";), 3)),
                    ),
                    so!(lit!(li!("Mary"; "D"; "Mary"), 1))
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("PRES"; "T", "=v*";), 4),
                    lit!(li!("C"; "C", "=T";), 5)
                )),
                w: Workspace::new(set!(
                    so!(
                        so!(
                            so!(lit!(li!("dances"; "V"; "dances"), 2)),
                            so!(lit!(li!("v*"; "v*", "=V", "=D";), 3)),
                        ),
                        so!(lit!(li!("Mary"; "D"; "Mary"), 1)),
                    )
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("C"; "C", "=T";), 5)
                )),
                w: Workspace::new(set!(
                    so!(lit!(li!("PRES"; "T", "=v*";), 4)),
                    so!(
                        so!(
                            so!(lit!(li!("dances"; "V"; "dances"), 2)),
                            so!(lit!(li!("v*"; "v*", "=V", "=D";), 3)),
                        ),
                        so!(lit!(li!("Mary"; "D"; "Mary"), 1)),
                    )
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!(
                    lit!(li!("C"; "C", "=T";), 5)
                )),
                w: Workspace::new(set!(
                    so!(
                        so!(lit!(li!("PRES"; "T", "=v*";), 4)),
                        so!(
                            so!(
                                so!(lit!(li!("dances"; "V"; "dances"), 2)),
                                so!(lit!(li!("v*"; "v*", "=V", "=D";), 3)),
                            ),
                            so!(lit!(li!("Mary"; "D"; "Mary"), 1)),
                        ),
                    )
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!()),
                w: Workspace::new(set!(
                    so!(lit!(li!("C"; "C", "=T";), 5)),
                    so!(
                        so!(lit!(li!("PRES"; "T", "=v*";), 4)),
                        so!(
                            so!(
                                so!(lit!(li!("dances"; "V"; "dances"), 2)),
                                so!(lit!(li!("v*"; "v*", "=V", "=D";), 3)),
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
                        so!(lit!(li!("C"; "C", "=T";), 5)),
                        so!(
                            so!(lit!(li!("PRES"; "T", "=v*";), 4)),
                            so!(
                                so!(
                                    so!(lit!(li!("dances"; "V"; "dances"), 2)),
                                    so!(lit!(li!("v*"; "v*", "=V", "=D";), 3)),
                                ),
                                so!(lit!(li!("Mary"; "D"; "Mary"), 1)),
                            ),
                        ),
                    )
                ))
            },
            
            Stage {
                la: LexicalArray::new(set!()),
                w: Workspace::new(set!(
                    so!(
                        so!(
                            so!(lit!(li!("C"; "C", "=T";), 5)),
                            so!(
                                so!(lit!(li!("PRES"; "T", "=v*";), 4)),
                                so!(
                                    so!(
                                        so!(lit!(li!("dances"; "V"; "dances"), 2)),
                                        so!(lit!(li!("v*"; "v*", "=V", "=D";), 3)),
                                    ),
                                    so!(lit!(li!("Mary"; "D"; "Mary"), 1)),
                                ),
                            ),
                        ) =>
                        fvec!( "Mary", "dances" ) ;
                        fset!( "C", "PRES", "v*", "dances", "Mary" )
                    )
                ))
            },
        ];

        assert!(is_derivation(&il, &stages));
        assert!(converges(&stages));
    }
}