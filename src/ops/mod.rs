use std::marker::{ PhantomData };
use std::iter;

use crate::prelude::*;



/// Select.
/// 
/// Return a stage on success, and an error message on failure.
/// 
/// From Definition 12 in C&S 2016, p. 47:
/// 
/// >Let $S$ be a stage in a derivation $S = \\langle \\textit{LA}, W \\rangle$.
/// >
/// >If lexical token $A \\in \\textit{LA}$, then $\\textrm{Select} (A, S) = \\langle \\textit{LA} - \\{ A \\}, W \\cup \\{ A \\} \\rangle$.
pub fn select(a: LexicalItemToken, s: Stage) -> Result<Stage, String> {
    let Stage { mut la, mut w } = s;

    if !la.remove(&a) {
        return Err(
            format!("BasicSelect: error.\nIn the provided stage, the lexical array:\n{:#?}\ndoes not contain the provided lexical item token:\n{:#?}", la, a)
        );
    }

    w.insert(SyntacticObject::LexicalItemToken(a));

    Ok(Stage { la, w })
}



/// Token-based Merge.
/// 
/// From Definition 13 in C&S 2016, p. 47:
/// 
/// >Given any two distinct syntactic objects $A, B$, $\\textrm{Merge} (A, B) = \\{ A, B \\}$.
pub fn token_based_merge(a: SyntacticObject, b: SyntacticObject, _w: &Workspace) -> Result<SyntacticObject, String> {
    //  a and b must be distinct!
    if a == b {
        return Err(
            format!("TokenBasedMerge: error.\nThe provided syntactic objects are not distinct.\nSyntactic object 1:\n{:#?}\nSyntactic object 2:\n{:#?}", a, b)
        );
    }

    let pair = set!( a, b );
    Ok(SyntacticObject::Set(pair))
}



/// Triggered (token-based) Merge.
/// 
/// From Definition 27 in C&S 2016, p. 64:
/// 
/// >Given any two distinct syntactic objects $A, B$, where $\\textrm{Triggers} (A) \neq \\varnothing$ and $\\textrm{Triggers} (B) = \\varnothing$, $\\textrm{Merge} (A, B) = \\{ A, B \\}$.
pub fn triggered_merge_with_so<T: Triggers>(a: SyntacticObject, b: SyntacticObject, w: &Workspace) -> Result<SyntacticObject, String> {
    // eprintln!("Triggered Merge: A =\n{}", a);
    // eprintln!("Triggered Merge: B =\n{}", b);

    //  a and b must be distinct!
    if a == b {
        // eprintln!("Triggered Merge: Error. A == B.");
        return Err(
            format!("TriggeredMerge: error.\nThe provided syntactic objects are not distinct.\nSyntactic object 1:\n{:#?}\nSyntactic object 2:\n{:#?}", a, b)
        );
    }

    //  a must have at least one trigger feature!
    let tfs_a = T::triggers(&a, w).map_err(|_| format!("tf a fail"))?;
    if tfs_a.is_empty() {
        // eprintln!("Triggered Merge: Error. Triggers(A) = {:?}", tfs_a);
        return Err(
            format!("TriggeredMerge: error.\nfor Merge (A, B), A must have at least one trigger feature.")
        );
    }
    // eprintln!("Triggered Merge: So far so good. Triggers(A) = {:?}", tfs_a);

    //  b must have zero trigger features!
    let tfs_b = T::triggers(&b, w).map_err(|_| format!("tf b fail"))?;
    if !tfs_b.is_empty() {
        // eprintln!("Triggered Merge: Error. Triggers(B) = {:?}", tfs_b);
        return Err(
            format!("TriggeredMerge: error.\nfor Merge (A, B), B must have zero trigger features, but tfs of B = {:?}.", tfs_b)
        );
    }
    // eprintln!("Triggered Merge: So far so good. Triggers(B) = {:?}", tfs_b);

    let pair = set!( a, b );
    Ok(SyntacticObject::Set(pair))
}



pub fn triggered_merge_with_f<T: Triggers>(a: SyntacticObject, b: SyntacticFeature, w: &Workspace) -> Result<SyntacticObject, String> {
    let tfs_a = T::triggers(&a, w).map_err(|_| format!("tf a fail"))?;
    if tfs_a.is_empty() {
        return Err(
            format!("TriggeredMerge: error.\nfor Merge (A, B), A must have at least one trigger feature.")
        );
    }

    Ok(SyntacticObject::WithFeature { so: Box::new(a), f: b })
}



pub fn is_strong_phase<T: Triggers>(so: &SyntacticObject, w: &Workspace) -> bool {
    w.contained_sos(false)
        .find(|&maybe_head| {
            match maybe_head {
                &SyntacticObject::LexicalItemToken(ref maybe_label) => {
                    so.is_maximal_projection_of::<T>(maybe_label, w) &&
                    (
                        maybe_label.li.syn.contains(&comp_feature!()) ||
                        maybe_label.li.syn.contains(&strong_light_verb_feature!())
                    )
                },
                _ => false,
            }
        })
        .is_some()
}



fn transfer_pf<T: Triggers>(phase: &SyntacticObject, so: &SyntacticObject, w: &Workspace) -> Vec<Feature> {
    let res = match so {
        &SyntacticObject::LexicalItemToken(ref lit) =>
            lit.li.phon.iter().map(|f| f.clone()).collect::<Vec<_>>(),

        &SyntacticObject::Set(ref set) => {
            assert!(set.len() == 2); // todo: don't assert, return Result

            let mut it = set.iter();

            let x1 = it.next().unwrap();
            let x2 = it.next().unwrap();

            let mut pf1 = 
                if x1.is_final(so, phase) {
                    Some(transfer_pf::<T>(phase, x1, w))
                }
                else {
                    None
                };

            let mut pf2 =
                if x2.is_final(so, phase) {
                    Some(transfer_pf::<T>(phase, x2, w))
                }
                else {
                    None
                };

            // eprintln!("TransferPF: X1 =\n{}", x1);
            // eprintln!("TransferPF: Is X1 final? {}", x1.is_final(so, phase));
            // eprintln!("TransferPF: PF of X1 = {:?}", pf1);
            // eprintln!("TransferPF: X2 =\n{}", x2);
            // eprintln!("TransferPF: Is X2 final? {}", x2.is_final(so, phase));
            // eprintln!("TransferPF: PF of X2 = {:?}", pf2);

            match (pf1, pf2) {
                (Some(mut pf1), Some(mut pf2)) => {
                    if (x2.is_complement_of::<T>(x1, so, w) ||
                        x1.is_specifier_of::<T>(x2, so, w)) {
                        pf1.extend(pf2);
                        pf1
                    }
                    else if (x1.is_complement_of::<T>(x2, so, w) ||
                        x2.is_specifier_of::<T>(x1, so, w)) {
                        pf2.extend(pf1);
                        pf2
                    }
                    else {
                        panic!("\nx1 = {}\nx2 = {}", SOPrefixFormatter::new(x1, 5), SOPrefixFormatter::new(x2, 5));
                    }
                },
                (Some(pf1), None) => pf1,
                (None, Some(pf2)) => pf2,
                (None, None) => vec![],
            }
        },

        &SyntacticObject::Transfer { ref pf, .. } => pf.clone(),
        &SyntacticObject::WithFeature { ref so, .. } =>
            transfer_pf::<T>(phase, so, w),
    };

    // eprintln!("TransferPF: Result = {:?}", res);
    res
}



fn transfer_lf(phase: &SyntacticObject, so: &SyntacticObject) -> Set<Feature> {
    match so {
        &SyntacticObject::LexicalItemToken(ref lit) =>
            lit.li.sem.clone(),

        &SyntacticObject::Set(ref vec) => {
            assert!(vec.len() == 2); // todo: don't assert, return Result
            vec.iter()
                .map(|so| transfer_lf(phase, so))
                .fold(
                    set!(),
                    |mut acc, sem| { acc.extend(sem.into_iter()); acc }
                )
        },

        &SyntacticObject::Transfer { ref lf, .. } => lf.clone(),
        &SyntacticObject::WithFeature { ref so, .. } => transfer_lf(phase, so),
    }
}



pub fn transfer<T: Triggers>(phase: &SyntacticObject, so: SyntacticObject, w: &Workspace) -> SyntacticObject {
    // eprintln!("Transfer: We are trying to transfer SO =\n{}", so);
    // eprintln!("Transfer: In the phase\n{}", phase);
    let pf = transfer_pf::<T>(&phase, &so, w);
    let lf = transfer_lf(&phase, &so);

    SyntacticObject::Transfer { so: Box::new(so), pf, lf }
}



fn unwind_and_transfer<T: Triggers>(mut so: SyntacticObject, head: &SyntacticObject, w: &Workspace) -> Result<SyntacticObject, SyntacticObject> {
    enum Action {
        Unwind, TransferFirst, TransferSecond, Return,
    }

    //  First, figure out what we should do
    let action = match so {
        SyntacticObject::Set(ref set) => {
            if set.len() == 2 {
                let mut it = set.iter();
                
                let x0 = it.next().unwrap();
                let x1 = it.next().unwrap();

                if x0.is_complement_of::<T>(&head, &so, w) {
                    // eprintln!("UnwindAndTransfer: This phase:\n{}", so);
                    // eprintln!("UnwindAndTransfer: Has the head:\n{}", head);
                    // eprintln!("UnwindAndTransfer: And the complement:\n{}", vec[0]);
                    // eprintln!("UnwindAndTransfer: Therefore, the complement will be transferred.");
                    Action::TransferFirst
                }
                else if x1.is_complement_of::<T>(&head, &so, w) {
                    // eprintln!("UnwindAndTransfer: This phase:\n{}", so);
                    // eprintln!("UnwindAndTransfer: Has the head:\n{}", head);
                    // eprintln!("UnwindAndTransfer: And the complement:\n{}", vec[1]);
                    // eprintln!("UnwindAndTransfer: Therefore, the complement will be transferred.");
                    Action::TransferSecond
                }
                else {
                    Action::Unwind
                }
            }
            else {
                Action::Unwind
            }
        },
        _ => Action::Return,
    };

    //  Then carry out the action
    match action {
        Action::Return => Err(so),
        Action::Unwind => {
            if let SyntacticObject::Set(vec) = so {
                let (set, is_ok) = vec.into_iter()
                    .map(|so| {
                        match unwind_and_transfer::<T>(so, head, w) {
                            Ok(so) => (so, true),
                            Err(so) => (so, false),
                        }
                    })
                    .fold(
                        (set!(), false),
                        |(mut set, is_ok1), (so, is_ok2)| {
                            set.insert(so);
                            (set, is_ok1 || is_ok2)
                        }
                    );

                match is_ok {
                    true => Ok(SyntacticObject::Set(set)),
                    false => Err(SyntacticObject::Set(set)),
                }
            }
            else {
                panic!()
            }
        },

        _ => {
            if let SyntacticObject::Set(ref set) = so {
                let mut it = set.iter();
                let x1 = it.next().unwrap().clone();
                let x2 = it.next().unwrap().clone();
    
                Ok(SyntacticObject::Set(match action {
                    Action::TransferFirst => set!( transfer::<T>(&so, x1, w), x2 ),
                    Action::TransferSecond => set!( x1, transfer::<T>(&so, x2, w) ),
                    _ => panic!(),
                }))
            }
            else {
                panic!()
            }
        },
    }
}



pub fn cyclic_transfer<T: Triggers>(so: SyntacticObject, w: &Workspace) -> Result<SyntacticObject, ()> {
    match T::label_of(&so, w) {
        Ok(label) => {
            let head = so!(label.clone());
            unwind_and_transfer::<T>(so, &head, w).map_err(|_| ())
        },
        _ => {
            Err(())
        }
    }
}