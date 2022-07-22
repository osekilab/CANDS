pub mod li;
pub mod lit;
pub mod so;
pub mod occur;



use crate::prelude::*;

use derive_more::{ Deref, DerefMut };

use std::marker::PhantomData;
use std::fmt;



/// Universal Grammar.
/// 
/// From Definition 1 in C&S 2016, p. 44.
/// 
/// >Universal Grammar is a 6-tuple: $\\langle \\textrm{PHON-F}, \\textrm{SYN-F}, \\textrm{SEM-F}, \\textrm{Select}, \\textrm{Merge}, \\textrm{Transfer} \\rangle$.
pub struct UniversalGrammar {
    pub phon_f:     Set<Feature>,
    pub syn_f:      Set<Feature>,
    pub sem_f:      Set<Feature>,
}



/// Lexicon.
/// 
/// From Definition 3 in C&S 2016, p. 44.
/// 
/// >A *lexicon* is a finite set of lexical items.
pub type Lexicon = Set<LexicalItem>;



/// I-language.
/// 
/// From Definition 4 in C&S 2016, p. 45.
/// 
/// An I-language is a pair $\\langle \\textrm{Lex}, \\textrm{UG} \\rangle$ where $\\textrm{Lex}$ is a lexicon and $\\textrm{UG}$ is Universal Grammar.
pub struct ILanguage {
    pub lex: Lexicon,
    pub ug: UniversalGrammar,
}



/// Lexical array.
/// 
/// From Definition 6 in C&S 2016, p. 45.
/// 
/// >A *lexical array* (LA) is a finite set of lexical item tokens.
pub type LexicalArray = Set<LexicalItemToken>;



/// Workspace.
/// 
/// See [`Stage`].
#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct Workspace(Set<SyntacticObject>);



impl Workspace {
    pub fn new(set: Set<SyntacticObject>) -> Self {
        Workspace(set)
    }
}



macro_rules! w {
    ($($so:expr),*) => {
        Workspace::new(set!( $($so),* ))
    }
}

pub(crate) use w;



impl Workspace {
    /// Immediate containment.
    /// 
    /// Derived from Definition 8, C&S 2016, p. 46.
    /// 
    /// Something like:
    /// 
    /// >Let $W$ be a workspace and $A$ a syntactic object, then $W$ *immediately contains* $A$ iff $A \\in W$.
    pub fn immediately_contains(&self, so: &SyntacticObject) -> bool {
        self.0.contains(so)
    }

    /// Containment.
    /// 
    /// From Definition 9, C&S 2016, p. 46.
    /// 
    /// Something like:
    /// 
    /// >Let $W$ be a workspace and $A$ a syntactic object, then $W$ *contains* $A$ iff
    /// >
    /// >1.  $W$ immediately contains $A$, or
    /// >2.  for some syntactic object $B$, $W$ immediately contains $B$ and $B$ contains $A$.
    pub fn contains(&self, so: &SyntacticObject) -> bool {
        self.0.contains(so) ||
        self.0.iter()
            .any(|b| b.contains(so))
    }



    /// Return an iterator over all the syntactic objects contained in `self`.
    pub fn contained_sos(&self, pic_compliant: bool) -> ContainedSyntacticObjects {
        let stack = self.0.iter().collect();
        ContainedSyntacticObjects::new(stack, pic_compliant)
    }
}



/// An iterator over the syntactic objects contained in a `Workspace`.
pub struct ContainedSyntacticObjects<'a> {
    /// The stack of syntactic objects that this iterator is supposed to visit.
    stack: Vec<&'a SyntacticObject>,
    pic_compliant: bool,
}



impl<'a> ContainedSyntacticObjects<'a> {
    fn new(stack: Vec<&'a SyntacticObject>, pic_compliant: bool) -> Self {
        Self { stack, pic_compliant }
    }
}



impl<'a> Iterator for ContainedSyntacticObjects<'a> {
    type Item = &'a SyntacticObject;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
            .map(|so| {
                match so {
                    &SyntacticObject::Set(ref children) => {
                        for child in children {
                            self.stack.push(child);
                        }
                    },
                    &SyntacticObject::Transfer { ref so, .. } => {
                        if !self.pic_compliant {
                            self.stack.push(so);
                        }
                    },
                    _ => (),
                }

                so
            })
    }
}



impl fmt::Display for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n")?;
        for so in self.0.iter() {
            write!(f, "{}\n", so)?;
        }
        write!(f, "}}")
    }
}



/// Stage.
/// 
/// From Definition 10, C&S 2016, p. 46.
/// 
/// >A *stage* is a pair $S = \\langle \textrm{LA}, W \\rangle$, where $\\textrm{LA}$ is a lexical array and $W$ is a set of syntactic objects. We call $W$ the *workspace* of $S$.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage {
    pub la: LexicalArray,
    pub w: Workspace,
}



/// Check if the sequence of stages `stages` is a derivation from the I-language `il`.
/// 
/// From Definition 14, C&S 2016, p. 48. The original derivation, given below, defines a derivation with respect to just a lexicon, but since it invokes syntactic operations like Select and Merge, we define it with respect to an I-language, which includes a UG as well as a lexicon.
/// 
/// >A *derivation* from lexicon $L$ is a finite sequence of stages $\\langle S\_1, \ldots, S\_n \\rangle$ for $n \\geq 1$, where each $S\_i = \\langle \\textrm{LA}\_i, W\_i \\rangle$, such that
/// >
/// >1.  For all $\\textrm{LI}$ and $k$ such that $\\langle \\textrm{LI}$, $k \\rangle \\in \\textrm{LA}_1$, $\\textrm{LI} \\in L$,
/// >2.  $W\_1 = \\{ \\}$ (the empty set)
/// >3.  for all $i$, such that $1 \\leq i < n$, either:
/// >    *   (Derive-by-Select) for some $A \\in \\textrm{LA}\_i$, $\\langle \\textrm{LA}\_{i+1}, W\_{i+1} \\rangle = \\textrm{Select} ( A, \\langle \\textrm{LA}\_i, W\_i \\rangle )$ or
/// >    *   (Derive-by-Merge) $\\textrm{LA}\_i = \\textrm{LA}\_{i+1}$ and the following conditions hold for some $A, B$:
/// >        1.  $A \\in W\_i$,
/// >        2.  either $A$ contains $B$ or $W\_i$ immediately contains $B$, and
/// >        3.  $W\_{i+1} = ( W\_i - \\{ A, B \\} ) \\cup \\{ \\textrm{Merge} ( A, B ) \\}$.
pub fn is_derivation<T: Triggers>(il: &ILanguage, stages: &[Stage]) -> bool {
    //  A derivation must have positive length.
    if stages.len() < 1 {
        return false;
    }

    //  Check if all lexical item tokens at the first stage are in the lexicon.
    let stage1 = &stages[0];
    let Stage { la: la1, w: w1 } = stage1;

    let ILanguage { lex, ug } = il;

    for lit in la1 {
        let LexicalItemToken { li, .. } = lit;
        if !lex.contains(li) {
            eprintln!("The lexicon does not contain this lexical item: {}", li);
            return false;
        }
    }

    //  Check if the workspace at the first stage is empty.
    if !w1.0.is_empty() {
        eprintln!("The first workspace must be empty");
        return false;
    }

    //  Check every stage.
    for stage_pair in stages.windows(2) {
        let stage1 = &stage_pair[0];
        let stage2 = &stage_pair[1];

        let Stage { la: la1, w: w1 } = stage1;
        let Stage { la: la2, w: w2 } = stage2;

        let step_ok = loop {
            //  Derive-by-Select?
            if la1.iter()
                .any(|lit| {
                    select(lit.clone(), stage1.clone())
                        .map_or(false, |stage| {
                            //eprintln!("Test stage: {:?}", stage);
                            stage2 == &stage
                        })
                })
            {
                break true;
            }

            if la1 == la2 {
                /*
                    Derive-by-Merge?

                    Derive-by-Merge is satisfied if there is a pair A, B that satisfy 3 conditions, one of which is that A in Wi. So we just search for a pair A, B where A is immediately contained in Wi and A, B satisfy the 2 other conditions. This works only if Wi is not empty, so we check that separately.
                */
                if !w1.0.is_empty() {
                    let res = w1.0.iter()
                        //  Iterate over all A, i.e., all root SOs in W1.
                        .any(|a| {
                            // eprintln!("Derive-by-Merge: Try A =\n{}", a);
                            //  Iterate over some B, i.e. all SOs contained in A ...
                            a.contained_sos(false, true)
                                //  ... as well as all SOs immediately contained in W1.
                                .chain(w1.0.iter())
                                //  Check if the final condition match.
                                .any(|b| {
                                    // eprintln!("Derive-by-Merge: Try B =\n{}", b);
                                    let mut w = w1.clone();
                                    w.0.remove(a);
                                    w.0.remove(b);

                                    triggered_merge::<T>(a.clone(), b.clone(), w1)
                                        // .map_or_else(|e| { eprintln!("Derive-by-Merge: Triggered Merge failed with the following error: {}", e); false }, move |ab| {
                                        .map_or(false, move |ab| {
                                            w.0.insert(ab);
                                            // eprintln!("Derived-by-Merge: Merge(A, B) works. w = {}", w);
                                            w2 == &w
                                        })
                                })
                        });

                    if res {
                        break true;
                    }

                    //  Derive-by-Transfer?
                    let res = w1.0.iter()
                        .any(|so1| {
                            //  Is a strong phase...
                            is_strong_phase::<T>(so1, w1) &&
                            //  Containing no other...
                            (!so1.contained_sos(false, true)
                                .any(|so2| {
                                    //  Strong phases...
                                    is_strong_phase::<T>(so2, w1) &&
                                    //  whose...
                                    match T::label_of(so2, w1) {
                                        Ok(lit) => {
                                            let head = so!(lit.clone());

                                            so2.contained_sos(true, true)
                                                .any(|so3| {
                                                    match so3 {
                                                        &SyntacticObject::Set(ref vec) => {
                                                            vec.iter()
                                                                .any(|so4| {
                                                                    //  complement...
                                                                    so4.is_complement_of::<T>(&head, so3, w1) &&
                                                                    //  has not yet been transferred...
                                                                    match so4 {
                                                                        &SyntacticObject::Transfer { .. } => false,
                                                                        _ => true,
                                                                    }
                                                                })
                                                        },
                                                        _ => false,
                                                    }
                                                })
                                        },
                                        _ => false,
                                    }
                                })) &&
                            //  And either...
                            ({
                                eprintln!("Derivation: Try Transfer ::::::::::::::::::::::::::::::::");
                                let mut w = w1.clone();
                                w.0.remove(so1);
                                w.0.insert(transfer::<T>(&so1, so1.clone(), w1));
                                w == *w2
                            } || {
                                eprintln!("Derivation: Try Cyclic-Transfer :::::::::::::::::::::::::");
                                let mut w = w1.clone();
                                w.0.remove(so1);
                                match cyclic_transfer::<T>(so1.clone(), w1) {
                                    Ok(so2) => {
                                        w.0.insert(so2);
                                        w == *w2
                                    },
                                    Err(_) => false,
                                }
                            })
                        });

                    if res {
                        break true;
                    }
                }
            }

            break false;
        };

        if !step_ok {
            eprintln!("This pair of stages is invalid:");
            eprintln!("Stage 1:\n");
            eprintln!(
                " :: Lexical array: {{{}\n}}",
                stage1.la.iter()
                    .map(|lit| format!("{}", lit))
                    .fold(format!(""), |a, b| format!("{}\n{}", a, b))
            );
            eprintln!(" :: Workspace:\n{}", stage1.w);
            eprintln!("Stage 2:\n");
            eprintln!(
                " :: Lexical array: {{{}\n}}",
                stage2.la.iter()
                    .map(|lit| format!("{}", lit))
                    .fold(format!(""), |a, b| format!("{}\n{}", a, b))
            );
            eprintln!(" :: Workspace:\n{}", stage2.w);
            return false;
        }
        else {
            eprintln!(" XXXXXXXXXXXXXXXXXXXXX good pair XXXXXXXXXXXXXXXXXXXXX ")
        }
    }

    true
}