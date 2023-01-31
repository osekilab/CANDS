//! A bunch of tests.

#[cfg(test)]
mod macros;

#[cfg(test)]
mod agree;

#[cfg(test)]
mod basic_select_tests {
    //  use crate::*;

    #[test]
    fn select_word() {

    }
}



fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}



#[cfg(test)]
mod derivck_tests {
    use std::collections::HashMap;

    use crate::deriv::LexicalArray;
    use crate::tests::init_logger;
    use crate::{f, fset, fvec, set};
    use crate::prelude::*;

    #[test]
    fn test1() {
        init_logger();

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