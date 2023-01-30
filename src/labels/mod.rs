use std::ops::{ Deref };

use crate::prelude::*;



/// A trait for trigger features, labels, etc.
pub trait Triggers {
    /// Triggers.
    /// 
    /// From Definition 26 in C&S 2016, p. 63:
    /// 
    /// >Triggers is any function from each syntactic object $A$ to a subset of the trigger features of $A$, meeting the following conditions:
    /// >
    /// >1.  If $A$ is a lexical item token with $n$ trigger features, then $\\textrm{Triggers} (A)$ returns all of those $n$ trigger features.
    /// >2.  If $A$ is a set, then $A = \\{ B, C \\}$ where $\\textrm{Triggers} (B)$ is nonempty, and $\\textrm{Triggers} (C) = \\varnothing$, and $\\textrm{Triggers} (A) = \\textrm{Triggers} (B) \setminus \\{ \\textrm{TF} \\}$, for some trigger feature $\\textrm{TF} \\in \\textrm{Triggers} B)$.
    /// >3.  Otherwise, $\\textrm{Triggers} (A)$ is undefined.
    fn triggers(so: &SyntacticObject, w: &Workspace) -> Result<Set<SyntacticFeature>, ()>;

    /// Label.
    /// 
    /// From Definition 28 in C&S 2016, p. 65:
    /// 
    /// >Label is a syntactic function from syntactic objects to lexical item tokens, defined in the following way:
    /// >
    /// >1.  For all lexical item tokens LI, Label(LI) = LI.
    /// >2.  Let W be a derivable workspace. If {A, B} is contained in W, and Triggers (A) is nonempty, then Label({A, B}) = Label(A).
    fn label_of<'a>(so: &'a SyntacticObject, w: &Workspace) -> Result<&'a LexicalItemToken, ()> {
        // eprintln!("Label: so =\n{}", so);

        match so {
            &SyntacticObject::LexicalItemToken(ref lit) => Ok(lit),
            &SyntacticObject::Set(ref set) => {
                if !w.contains(so) {
                    // eprintln!("Label: The workspace does not contain so. W =\n{}", w);
                    return Err(());
                }

                match set.len() == 2 {
                    true => {
                        let mut iter = set.iter();
                        let b = iter.next().unwrap();
                        let c = iter.next().unwrap();
                        assert!(iter.next().is_none());

                        // eprintln!("Label: so = {{ B, C }}, where B =\n{}", b);
                        // eprintln!("Label: C = \n{}", c);
                        
                        let tfs_b = Self::triggers(b, w)?;

                        // eprintln!("Label: Triggers(B) = {:?}", tfs_b);

                        let tfs_c = Self::triggers(c, w)?;

                        // eprintln!("Label: Triggers(C) = {:?}", tfs_c);

                        if !tfs_b.is_empty() && tfs_c.is_empty() {
                            // eprintln!("Label: Triggers(B) != ∅, Triggers(C) == ∅");
                            Self::label_of(b, w)
                        }
                        else if !tfs_c.is_empty() && tfs_b.is_empty() {
                            // eprintln!("Label: Triggers(C) != ∅, Triggers(B) == ∅");
                            Self::label_of(c, w)
                        }
                        else {
                            // eprintln!("Label: Error. Triggers(B) = {:?}, Triggers(C) = {:?}", tfs_b, tfs_c);
                            Err(())
                        }
                    },
                    false => Err(()),
                }
            },
            &SyntacticObject::Transfer{ ref so, .. } =>
                Self::label_of(so, w),
        }
    }
}



#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BasicTriggers;



impl BasicTriggers {
    /// Check one feature.
    fn check_tf(mut from: Set<SyntacticFeature>, wrt: &SyntacticObject, w: &Workspace) -> Result<Set<SyntacticFeature>, ()> {
        //  Get the syntactic features of the label of `wrt`
        let wrt_syn = &Self::label_of(wrt, w)?.li.syn;
        // eprintln!("Check-TF: wrt_syn = {:?}", wrt_syn);

        //  Check wh
        if from.contains(&wh_feature!()) && wrt_syn.contains(&wh_feature!()) {
            assert!(from.remove(&wh_feature!()));
            return Ok(from);
        }

        //  Check category selection
        if let Some(catsel_feature) = from.iter()
            .find(|&f| {
                if let SyntacticFeature::Normal(f) = f {
                    //  If `f` were "=v*", then `cat_feature` would be "v*".
                    let cat_feature = SyntacticFeature::Normal(f!(&f.0[CATSEL_FEATURE_PREFIX.len()..]));
                    f.0.starts_with(CATSEL_FEATURE_PREFIX) &&
                    (wrt_syn.contains(&cat_feature))
                }
                else {
                    false
                }
            })
            .map(|f| f.clone()) // borrowck wins
        {
            assert!(from.remove(&catsel_feature));
            return Ok(from);
        }

        //  Check EPP
        if from.contains(&epp_feature!()) {
            assert!(from.remove(&epp_feature!()));
            return Ok(from);
        }

        Err(())
    }
}



impl Triggers for BasicTriggers {
    fn triggers(so: &SyntacticObject, w: &Workspace) -> Result<Set<SyntacticFeature>, ()> {
        // eprintln!("Triggers: so =\n{}", so);

        match so {
            &SyntacticObject::LexicalItemToken(ref lit) => {
                Ok(
                    lit.li.syn.iter()
                        .filter(|&f| {
                            f == &wh_feature!() ||
                            f == &epp_feature!() ||
                            if let SyntacticFeature::Normal(f) = f {
                                f.0.starts_with(CATSEL_FEATURE_PREFIX)
                            } else { false }
                        })
                        .cloned()
                        .collect()
                )
                //  Ok(lit.li.syn.intersection(&self.0).cloned().collect())
            },
            &SyntacticObject::Set(ref set) => {
                match set.len() == 2 {
                    true => {
                        let mut iter = set.iter();
                        let b = iter.next().unwrap();
                        let c = iter.next().unwrap();
                        assert!(iter.next().is_none());

                        // eprintln!("Triggers: so = {{ B, C }}, where B =\n{}", b);
                        // eprintln!("Triggers: C = \n{}", c);
                        
                        let tfs_b = Self::triggers(b, w)?;

                        // eprintln!("Triggers: Triggers(B) = {:?}", tfs_b);

                        let tfs_c = Self::triggers(c, w)?;

                        // eprintln!("Triggers: Triggers(C) = {:?}", tfs_c);

                        if !tfs_b.is_empty() && tfs_c.is_empty() {
                            // eprintln!("Triggers: Triggers(B) != ∅, Triggers(C) == ∅");
                            let res = Self::check_tf(tfs_b, c, w);
                            // eprintln!("Triggers: Triggers(so) = {:?}", res);
                            res
                        }
                        else if !tfs_c.is_empty() && tfs_b.is_empty() {
                            // eprintln!("Triggers: Triggers(C) != ∅, Triggers(B) == ∅");
                            let res = Self::check_tf(tfs_c, b, w);
                            // eprintln!("Triggers: Triggers(so) = {:?}", res);
                            res
                        }
                        else {
                            // eprintln!("Triggers: Either B or C has to be empty, but Triggers(B) = {:?}, Triggers(C) = {:?}", tfs_b, tfs_c);
                            Err(())
                        }
                    },
                    false => {
                        // eprintln!("Triggers: so needs to be a binary tree");
                        Err(())
                    },
                }
            },
            &SyntacticObject::Transfer{ ref so, .. } =>
                Self::triggers(&so, w),
        }
    }
}