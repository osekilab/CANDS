use crate::utils::{ Set };
use crate::deriv::{
    Workspace,
    lit::{ LexicalItemToken },
    occur::{ Occurrence }
};

use std::fmt;



/// Syntactic object.
/// 
/// From Definition 7, C&S 2016, p. 46.
/// 
/// >$X$ is a *syntactic object* iff
/// 
/// >1.  $X$ is a lexical item token, or
/// >2.  $X$ is a set of syntactic objects.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntacticObject {
    LexicalItemToken(LexicalItemToken),
    Set(Set<SyntacticObject>),
}



impl fmt::Display for SyntacticObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_prefix("", "", false, true, f)
    }
}



/// Macro to generate a syntactic object.
/// 
/// # Example
/// 
/// You can generate a syntactic object from a lexical item token, in which case you should **not** provide a comma after the lexical item token.
///
/// For example, this works because there is no comma after the call to `lit!`:
/// 
/// ```
/// so!(
///     lit!(li!(; "n", "EPP"; "-voice"; "book"), 1)
/// )
/// ```
///
/// But this code will not:
/// 
/// ```compile_fail
/// so!(
///     lit!(li!(; "n", "EPP"; "-voice"; "book"), 1),
/// )
/// ```
/// 
/// Or generate a syntactic object from a set of syntactic objects. In this case, you **must** provide a comma after each syntactic object.
/// 
/// This works:
/// 
/// ```
/// so!(
///     so!(lit!(li!(; "v", "EPP"; "-voice"; "read"), 1)),
///     so!(lit!(li!(; "n", "EPP"; "-voice"; "book"), 1)),
/// )
/// ```
/// 
/// But not (note that the comma after the second `so!(lit!( ... ))` is missing):
/// 
/// ```compile_fail
/// so!(
///     so!(lit!(li!(; "v", "EPP"; "-voice"; "read"), 1)),
///     so!(lit!(li!(; "n", "EPP"; "-voice"; "book"), 1))
/// )
/// ```
macro_rules! so {
    ($($so:expr,)*) => {
        SyntacticObject::Set(set!($($so),*))
    };

    ($lit:expr) => {
        SyntacticObject::LexicalItemToken($lit)
    };
}

pub(crate) use so;



impl SyntacticObject {
    /// Immediate containment.
    /// 
    /// From Definition 8, C&S 2016, p. 46.
    /// 
    /// >Let $A$ and $B$ be syntactic objects, then $B$ *immediately contains* $A$ iff $A \\in B$.
    pub fn immediately_contains(&self, a: &SyntacticObject) -> bool {
        match self {
            //  A lexical item token does not immediately contain any syntactic object because it is not a set.
            &SyntacticObject::LexicalItemToken(_) => false,
            &SyntacticObject::Set(ref b) => b.contains(a)
        }
    }

    /// Containment.
    /// 
    /// From Definition 9, C&S 2016, p. 46.
    /// 
    /// >Let $A$ and $B$ be syntactic objects, then $B$ *contains* $A$ iff
    /// >
    /// >1.  $B$ immediately contains $A$, or
    /// >2.  for some syntactic object $C$, $B$ immediately contains $C$ and $C$ contains $A$.
    pub fn contains(&self, a: &SyntacticObject) -> bool {
        match self {
            //  A lexical item token does not immediately contain any syntactic object because it is not a set.
            &SyntacticObject::LexicalItemToken(_) => false,
            &SyntacticObject::Set(ref b) => {
                b.contains(a) ||
                b.iter()
                    .any(|so| so.contains(a))
            },
        }
    }

    /// Roothood.
    /// 
    /// From Definition 11, C&S 2016, p. 47.
    /// 
    /// >For any syntactic object $X$ and any stage $S = \\langle \textrm{LA}, W \\rangle$ with workspace $W$, if $X \\in W$, $X$ is a *root* in $W$.
    pub fn is_root(&self, w: &Workspace) -> bool {
        w.immediately_contains(self)
    }

    /// Sisterhood.
    /// 
    /// From Definition 19, C&S 2016, p. 52.
    /// 
    /// >Let $A$, $B$, $C$ be syntactic objects (where $A \\neq B$), then $A$ and $B$ are *sisters* in $C$ iff $A, B \\in C$.
    pub fn sisters_with(&self, other: &SyntacticObject, under: &SyntacticObject) -> bool {
        (self != other) &&
        under.immediately_contains(self) &&
        under.immediately_contains(other)
    }

    /// C-command.
    /// 
    /// From Definition 21, C&S 2016, p. 53.
    /// 
    /// >Let $A$ and $B$ be syntactic objects, then $A$ *c-commands* $B$ in $D$, iff there is a syntactic object $C$, such that:
    /// >
    /// >1.  $C$ is a sister of $A$ in $D$, and
    /// >2.  either $B = C$ or $C$ contains $B$.
    pub fn c_commands(&self, other: &SyntacticObject, under: &SyntacticObject) -> bool {
        //  `under` is D in the definition.
        under.contained_sos()
            .any(|so| {
                //  `so` is C in the definition.
                so.sisters_with(self, under) &&
                (other == so || so.contains(other))
            })
    }

    /// Asymmetric c-command.
    /// 
    /// From Definition 21, C&S 2016, p. 53.
    /// 
    /// $A$ *asymmetrically c-commands* $B$ iff $A$ c-commands $B$ and $A$ and $B$ are not sisters.
    pub fn asymmetrically_c_commands(&self, other: &SyntacticObject, under: &SyntacticObject) -> bool {
        (!self.sisters_with(other, under)) &&
        self.c_commands(other, under)
    }

    /// Binary branching.
    /// 
    /// From Definition 24, C&S 2016, p. 57.
    /// 
    /// >Syntactic object $A$ is *binary branching* iff both $A$ and everything contained in $A$ is either a lexical item token or a syntactic object immediately containing exactly two syntactic objects.
    pub fn is_binary_branching(&self) -> bool {
        match self {
            &SyntacticObject::LexicalItemToken(_) => true,
            &SyntacticObject::Set(ref set) => {
                set.iter()
                    .all(|so| so.is_binary_branching()) &&
                (set.len() == 2)
            },
        }
    }

    /// Return an iterator over all the syntactic objects contained in `self`.
    pub fn contained_sos(&self) -> ContainedSyntacticObjects {
        let stack = match self {
            &SyntacticObject::LexicalItemToken(_) => vec![],
            &SyntacticObject::Set(ref set) => set.iter().collect(),
        };

        ContainedSyntacticObjects::new(stack)
    }

    /// Return an iterator over all the occurrences contained in `self`.
    pub fn iter_contained_as_occ(&self) -> Box<dyn Iterator<Item = &Occurrence<'_>> + '_> {
        Box::new(std::iter::empty())
    }

    /*
    ┌─┬─ this
    │ └─┬─ person
    │   └─┬─ Q
    │     └─┬─ you
    │       └─ know
    └─┬─ Pres
      └─┬─ v
        └─┬─ wrote
          └─┬─ this
            └─ book
    */

    /// Recursive function used by the `fmt::Display` implementation to pretty-print the current syntactic object.
    fn fmt_with_prefix(&self,
        prefix1: &str,
        prefix2: &str,
        newline: bool,
        first: bool,
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
        match self {
            &SyntacticObject::LexicalItemToken(ref lit) =>
                write!(f,
                    "{}{}{}{}",
                    prefix1,
                    if first { "" } else { "─ " },
                    lit,
                    if newline { "\n" } else { "" }
                ),

            &SyntacticObject::Set(ref set) => {
                if set.is_empty() {
                    return write!(f,
                        "{}{}Ø{}",
                        prefix1,
                        if first { "" } else { "─ " },
                        if newline { "\n" } else { "" }
                    );
                }

                if set.len() == 1 {
                    let newprefix1 = format!("{}{}═", prefix1, if first { " " } else { "─" });
                    let newprefix2 = format!("{}{} ", prefix2, if first { " " } else { "─" });
                    return set.iter().next().unwrap()
                        .fmt_with_prefix(&newprefix1, &newprefix2, newline, false, f);
                }

                let mut it = set.iter();

                let newprefix1 = format!("{}{}", prefix1, if first { " ╔" } else { "─╦" });
                let newprefix2 = format!("{}{}║", prefix1, if first { " " } else { "─" });
                it.next().unwrap()
                    .fmt_with_prefix(&newprefix1, &newprefix2, true, false, f)?;

                let newprefix1 = format!("{} ╠", prefix2);
                let newprefix2 = format!("{} ║", prefix2);
                for _ in 0..(set.len() - 2) {
                    it.next().unwrap()
                        .fmt_with_prefix(&newprefix1, &newprefix2, true, false, f)?;
                }

                let newprefix1 = format!("{} ╚", prefix2);
                let newprefix2 = format!("{}  ", prefix2);
                let res = it.next().unwrap()
                    .fmt_with_prefix(&newprefix1, &newprefix2, newline, false, f);

                assert_eq!(it.next(), None);
                res
            },
        }
    }
}



/// An iterator over the syntactic objects contained in a `SyntacticObject`.
pub struct ContainedSyntacticObjects<'a> {
    /// The stack of syntactic objects that this iterator is supposed to visit.
    stack: Vec<&'a SyntacticObject>,
}



impl<'a> ContainedSyntacticObjects<'a> {
    fn new(stack: Vec<&'a SyntacticObject>) -> Self {
        Self { stack }
    }
}



impl<'a> Iterator for ContainedSyntacticObjects<'a> {
    type Item = &'a SyntacticObject;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
            .map(|so| {
                if let SyntacticObject::Set(ref children) = so {
                    for child in children {
                        self.stack.push(child);
                    }
                }

                so
            })
    }
}



#[cfg(test)]
mod tests {

    mod so {
        use crate::prelude::*;



        #[test]
        //  Use `cargo test -- --nocapture` to show the results.
        fn display() {
            println!("{}",
                so!(lit!(li!(;;; "Alex"), 1))
            );

            println!("{}",
                so!()
            );
            
            println!("{}",
                so!(
                    so!(lit!(li!(;;; "Alex"), 1)),
                )
            );

            println!("{}",
                so!(
                    so!(lit!(li!(;;; "Alex"), 1)),
                    so!(lit!(li!(;;; "danced"), 1)),
                )
            );

            println!("{}",
                so!(
                    so!(lit!(li!(;;; "Alex"), 1)),
                    so!(lit!(li!(;;; "never"), 1)),
                    so!(lit!(li!(;;; "danced"), 1)),
                )
            );

            println!("{}",
                so!(
                    so!(
                        so!(lit!(li!(;;; "Alpha"), 1)),
                        so!(lit!(li!(;;; "Beta"), 1)),
                        so!(lit!(li!(;;; "Gamma"), 1)),
                    ),
                    so!(
                        so!(lit!(li!(;;; "never"), 1)),
                        so!(lit!(li!(;;; "ever"), 1)),
                        so!(lit!(li!(;;; "ever"), 1)),
                    ),
                    so!(
                        so!(lit!(li!(;;; "danced"), 1)),
                        so!(lit!(li!(;;; "like"), 1)),
                        so!(lit!(li!(;;; "there\'s"), 1)),
                        so!(lit!(li!(;;; "no"), 1)),
                        so!(lit!(li!(;;; "tomorrow"), 1)),
                    ),
                )
            );

            println!("{}",
                so!(
                    so!(
                        so!(lit!(li!(;;; "the"), 1)),
                        so!(
                            so!(lit!(li!(;;; "person"), 1)),
                            so!(
                                so!(lit!(li!(;;; "Q"), 1)),
                                so!(
                                    so!(lit!(li!(;;; "you"), 1)),
                                    so!(lit!(li!(;;; "love"), 1)),
                                ),
                            ),
                        ),
                    ),
                    so!(
                        so!(lit!(li!(;;; "Pres"), 1)),
                        so!(
                            so!(lit!(li!(;;; "v"), 1)),
                            so!(
                                so!(lit!(li!(;;; "wrote"), 1)),
                                so!(
                                    so!(lit!(li!(;;; "this"), 1)),
                                    so!(lit!(li!(;;; "book"), 1)),
                                ),
                            ),
                        ),
                    ),
                )
            );
        }
    }



    mod config {
        use crate::prelude::*;

        fn get_so() -> SyntacticObject {
            /*
                [
                    A
                    [
                        [ B C D ]
                        E
                        [
                            [ F G ]
                            H
                            I
                        ]
                    ]
                ]
            */
            so!(
                so!(lit!(li!("A"))),
                so!(
                    so!(
                        so!(lit!(li!("B"))),
                        so!(lit!(li!("C"))),
                        so!(lit!(li!("D"))),
                    ),
                    so!(lit!(li!("E"))),
                    so!(
                        so!(
                            so!(lit!(li!("F"))),
                            so!(lit!(li!("G"))),
                        ),
                        so!(lit!(li!("H"))),
                        so!(lit!(li!("I"))),
                    ),
                ),
            )
        }



        #[test]
        /// Test if a SO that is a lexical item token immediately contains the right things.
        fn lit_immediately_contains() {
            let so = so!(lit!(li!("A")));

            //  Should not immediately contain anything
            assert!(!so.immediately_contains(&so!(lit!(li!("A")))));
            assert!(!so.immediately_contains(&so!(lit!(li!("A")))));
            assert!(!so.immediately_contains(&so!()));
            assert!(!so.immediately_contains(&so!(
                so!(lit!(li!("A"))),
            )));
            assert!(!so.immediately_contains(&so!(
                so!(lit!(li!("A"))),
                so!(lit!(li!("B"))),
                so!(lit!(li!("C"))),
            )));
        }



        #[test]
        /// Test if a SO that is a set immediately contains the right things.
        fn set_immediately_contains() {
            let so = get_so();

            /*
                Should immediately contain:
                 -  A
                -   [ [ B C D ] E [ [ F G ] H I ] ]
            */
            for other in [
                so!(lit!(li!("A"))),

                so!(
                    so!(
                        so!(lit!(li!("B"))),
                        so!(lit!(li!("C"))),
                        so!(lit!(li!("D"))),
                    ),
                    so!(lit!(li!("E"))),
                    so!(
                        so!(
                            so!(lit!(li!("F"))),
                            so!(lit!(li!("G"))),
                        ),
                        so!(lit!(li!("H"))),
                        so!(lit!(li!("I"))),
                    ),
                )
            ] {
                assert!(so.contains(&other));
            }

            //  Should NOT immediately contain itself
            assert!(!so.immediately_contains(&so));
        }


        #[test]
        /// Test if a SO that is a lexical item token contains the right things.
        fn lit_contains() {
            let so = so!(lit!(li!("A")));

            //  Should not contain anything
            assert!(!so.contains(&so!(lit!(li!("A")))));
            assert!(!so.contains(&so!(lit!(li!("A")))));
            assert!(!so.contains(&so!()));
            assert!(!so.contains(&so!(
                so!(lit!(li!("A"))),
            )));
            assert!(!so.contains(&so!(
                so!(lit!(li!("A"))),
                so!(lit!(li!("B"))),
                so!(lit!(li!("C"))),
            )));
        }



        #[test]
        /// Test if a SO that is a set contains the right things.
        fn set_contains() {
            let so = get_so();

            //  Should contain each LIT
            for ch in "ABCDEFGHI".chars() {
                assert!(so.contains(
                    &so!(lit!(li!(ch)))
                ));
            }

            /*
                Should contain:

                 -  [ B C D ]
                 -  [ F G ]
                 -  [ [ F G ] H I ]
                 -  [ [ B C D ] E [ [ F G ] H I ] ]
            */
            for other in [
                so!(
                    so!(lit!(li!("B"))),
                    so!(lit!(li!("C"))),
                    so!(lit!(li!("D"))),
                ),

                so!(
                    so!(lit!(li!("F"))),
                    so!(lit!(li!("G"))),
                ),

                so!(
                    so!(
                        so!(lit!(li!("F"))),
                        so!(lit!(li!("G"))),
                    ),
                    so!(lit!(li!("H"))),
                    so!(lit!(li!("I"))),
                ),

                so!(
                    so!(
                        so!(lit!(li!("B"))),
                        so!(lit!(li!("C"))),
                        so!(lit!(li!("D"))),
                    ),
                    so!(lit!(li!("E"))),
                    so!(
                        so!(
                            so!(lit!(li!("F"))),
                            so!(lit!(li!("G"))),
                        ),
                        so!(lit!(li!("H"))),
                        so!(lit!(li!("I"))),
                    ),
                )
            ] {
                assert!(so.contains(&other));
            }

            //  Should NOT contain itself
            assert!(!so.contains(&so));
        }



        #[test]
        /// Test roothood.
        fn is_root() {
            let w1 = w!(
                so!(lit!(li!("A"))),
                so!(
                    so!(lit!(li!("B"))),
                    so!(lit!(li!("C"))),
                )
            );

            let w2 = w!(
                so!(lit!(li!("B")))
            );

            assert!(so!(lit!(li!("A"))).is_root(&w1));
            assert!(!so!(lit!(li!("A"))).is_root(&w2));

            assert!(!so!(lit!(li!("B"))).is_root(&w1));
            assert!(so!(lit!(li!("B"))).is_root(&w2));

            assert!(so!(
                so!(lit!(li!("B"))),
                so!(lit!(li!("C"))),
            ).is_root(&w1));
            assert!(!so!(
                so!(lit!(li!("B"))),
                so!(lit!(li!("C"))),
            ).is_root(&w2));
        }



        #[test]
        /// Test sisterhood
        fn sisters_with() {
            let under1 = so!(
                so!(lit!(li!("A"))),
                so!(lit!(li!("B"))),
            );
            let under2 = so!(
                so!(lit!(li!("B"))),
                so!(lit!(li!("C"))),
            );
            let under3 = so!(
                so!(lit!(li!("B"))),
                so!(
                    so!(lit!(li!("B"))),
                    so!(lit!(li!("C"))),
                ),
            );
            let under4 = so!(
                so!(lit!(li!("A"))),
                so!(
                    so!(lit!(li!("B"))),
                    so!(
                        so!(lit!(li!("B"))),
                        so!(lit!(li!("C"))),
                    ),
                ),
            );

            let sis1 = so!(lit!(li!("A")));
            let sis2 = so!(lit!(li!("B")));
            let sis3 = so!(lit!(li!("C")));
            let sis4 = so!(
                so!(lit!(li!("B"))),
                so!(lit!(li!("C"))),
            );

            //  Assert TRUE
            assert!(sis1.sisters_with(&sis2, &under1));
            assert!(sis2.sisters_with(&sis3, &under2));
            assert!(sis2.sisters_with(&sis4, &under3));
            assert!(sis1.sisters_with(&under3, &under4));

            assert!(sis2.sisters_with(&sis1, &under1));
            assert!(sis3.sisters_with(&sis2, &under2));
            assert!(sis4.sisters_with(&sis2, &under3));
            assert!(under3.sisters_with(&sis1, &under4));

            //  Assert FALSE
            assert!(!sis2.sisters_with(&sis4, &under4));
            assert!(!sis4.sisters_with(&sis2, &under4));
        }
    }



    mod iter {
        use crate::prelude::*;

        use std::collections::{ HashMap };

        /// Take an `ContainedSyntacticObjects` iterator and return a hashmap
        /// that maintains a count for each syntactic object that appears in
        /// the iterator.
        fn iter_to_multiset(
            iter: ContainedSyntacticObjects
        ) -> HashMap<SyntacticObject, usize> {
            iter
                .fold(
                    HashMap::new(),
                    |mut multiset, so| {
                        match multiset.get_mut(so) {
                            None => {
                                assert!(multiset.insert(so.clone(), 1).is_none());
                            },
                            Some(count) => { *count += 1; },
                        }
                        multiset
                    }
                )
        }

        fn iter_equals_multiset(
            iter: ContainedSyntacticObjects,
            multiset: &HashMap<SyntacticObject, usize>
        ) -> bool {
            //  make set out of iter
            let iter_multiset = iter_to_multiset(iter);

            //  set equality
            let equals =  iter_multiset == *multiset;

            if !equals {
                //  println!("iterset: {:#?}", iterset);
                //  println!("set: {:#?}", set);
                eprintln!("iter_multiset:");
                for (k, (so, count)) in iter_multiset.iter().enumerate() {
                    eprintln!("SO {} appears {} times:\n{}\n", k, count, so);
                }

                eprintln!("multiset:");
                for (k, (so, count)) in multiset.iter().enumerate() {
                    eprintln!("SO {} appears {} times:\n{}\n", k, count, so);
                }
            }

            equals
        }

        #[test]
        fn contained_sos1() {
            let so = so!(lit!(li!(;;; "John"), 1));
            let iter = so.contained_sos();

            let multiset = HashMap::new();

            assert!(iter_equals_multiset(iter, &multiset));
        }

        #[test]
        fn contained_sos2() {
            let so = so!(
                so!(lit!(li!(;;; "John"), 1)),
                so!(lit!(li!(;;; "eat"), 1)),
            );
            let iter = so.contained_sos();

            let multiset = HashMap::from([
                (so!(lit!(li!(;;; "John"), 1)), 1),
                (so!(lit!(li!(;;; "eat"), 1)),  1),
            ]);

            assert!(iter_equals_multiset(iter, &multiset));
        }

        #[test]
        fn contained_sos3() {
            //  [ John [ was [ helped John ] ] ]
            let so = so!(
                so!(lit!(li!(;;; "John"), 1)),
                so!(
                    so!(lit!(li!(;;; "was"), 1)),
                    so!(
                        so!(lit!(li!(;;; "helped"), 1)),
                        so!(lit!(li!(;;; "John"), 1)),
                    ),
                ),
            );
            let iter = so.contained_sos();

            let multiset = HashMap::from([
                //  helped
                (so!(lit!(li!(;;; "helped"), 1)),           1),

                //  John (appears TWICE)
                (so!(lit!(li!(;;; "John"), 1)),             2),

                //  [ helped John ]
                (so!(
                    so!(lit!(li!(;;; "helped"), 1)),
                    so!(lit!(li!(;;; "John"), 1)),
                ),                                          1),

                //  was
                (so!(lit!(li!(;;; "was"), 1)),              1),

                //  [ was [ helped John ] ]
                (so!(
                    so!(lit!(li!(;;; "was"), 1)),
                    so!(
                        so!(lit!(li!(;;; "helped"), 1)),
                        so!(lit!(li!(;;; "John"), 1)),
                    ),
                ),                                          1),
            ]);

            assert!(iter_equals_multiset(iter, &multiset));
        }
    }
}