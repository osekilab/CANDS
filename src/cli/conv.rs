use crate::prelude::*;

use super::Value;

//  F
pub fn to_feature(val: Value) -> Result<Feature, ()> {
    match val {
        Value::Feature(f) => Ok(f),
        _ => Err(()),
    }
}

//  usize
pub fn to_usize(val: Value) -> Result<usize, ()> {
    match val {
        Value::Usize(x) => Ok(x),
        _ => Err(()),
    }
}

//  {T}
fn to_set<T: Ord, F: FnMut(Value) -> Result<T, ()>>(val: Value, f: F) -> Result<Set<T>, ()> {
    match val {
        Value::Set(vals) => {
            vals.into_iter()
                .map(f)
                .fold(
                    Ok(set!()),
                    |set, x| {
                        set.and_then(|mut set| {
                            x.map(|x| {
                                set.insert(x);
                                set
                            })
                        })
                    }
                )
        },

        _ => Err(()),
    }
}

//  [T]
fn to_vec<T, F: FnMut(Value) -> Result<T, ()>>(val: Value, f: F) -> Result<Vec<T>, ()> {
    match val {
        Value::Vec(vals) => {
            vals.into_iter()
                .map(f)
                .fold(
                    Ok(vec![]),
                    |vec, x| {
                        vec.and_then(|mut vec| {
                            x.map(|x| {
                                vec.push(x);
                                vec
                            })
                        })
                    }
                )
        },

        _ => Err(()),
    }
}

//  Lexical item := <{F}, {F}, [F]>
pub fn to_lexical_item(val: Value) -> Result<LexicalItem, ()> {
    match val {
        Value::Tuple(vals) => {
            if vals.len() != 3 {
                return Err(());
            }

            let mut it = vals.into_iter();

            let sem = to_set(it.next().unwrap(), to_feature)?;
            let syn = to_set(it.next().unwrap(), to_feature)?;
            let phon = to_vec(it.next().unwrap(), to_feature)?;

            Ok(LexicalItem::new(sem, syn, phon, None))
        },

        _ => Err(()),
    }
}

//  Lexical item token := < LI, usize >
pub fn to_lexical_item_token(val: Value) -> Result<LexicalItemToken, ()> {
    match val {
        Value::Tuple(vals) => {
            if vals.len() != 2 {
                return Err(());
            }

            let mut it = vals.into_iter();

            let li = to_lexical_item(it.next().unwrap())?;
            let k = to_usize(it.next().unwrap())?;

            Ok(LexicalItemToken::new(li, k))
        },

        _ => Err(()),
    }
}

//  Lexicon := {Li}
pub fn to_lexicon(val: Value) -> Result<Lexicon, ()> {
    match val {
        Value::Set(vals) =>
            vals.into_iter()
                .map(to_lexical_item)
                .fold(
                    Ok(set!()),
                    |liset, li| {
                        liset.and_then(|mut liset| {
                            li.map(|li| {
                                liset.insert(li);
                                liset
                            })
                        })
                    }
                ),

        _ => Err(()),
    }
}

//  UG := <{F}, {F}, {F}>
pub fn to_ug<T: Triggers>(val: Value) -> Result<UniversalGrammar<T>, ()> {
    match val {
        Value::Tuple(vals) => {
            if vals.len() != 3 {
                return Err(());
            }

            let mut it = vals.into_iter();

            let phon_f = to_set(it.next().unwrap(), to_feature)?;
            let syn_f = to_set(it.next().unwrap(), to_feature)?;
            let sem_f = to_set(it.next().unwrap(), to_feature)?;

            Ok(UniversalGrammar::new(phon_f, syn_f, sem_f))
        },

        _ => Err(()),
    }
}

//  SO := ... you get the idea, sum type
pub fn to_so(val: Value) -> Result<SyntacticObject, ()> {
    //  Lit?
    let val_for_set = val.clone();
    let val_for_lit = val;
    
    if let Ok(lit) = to_lexical_item_token(val_for_lit) {
        return Ok(SyntacticObject::LexicalItemToken(lit));
    }

    //  Set?
    let val_for_transfer = val_for_set.clone();
    if let Ok(set) = to_set(val_for_set, to_so) {
        return Ok(SyntacticObject::Set(set.into_iter().collect()));
    }

    //  Transfer?
    match val_for_transfer {
        Value::Tuple(vals) => {
            if vals.len() != 3 {
                return Err(());
            }

            let mut it = vals.into_iter();

            let so = to_so(it.next().unwrap())?;
            let pf = to_vec(it.next().unwrap(), to_feature)?;
            let lf = to_set(it.next().unwrap(), to_feature)?;

            Ok(SyntacticObject::Transfer { so: Box::new(so), pf, lf })
        },

        _ => Err(()),
    }
}

//  Stage := < La, Wksp >
pub fn to_stage(val: Value) -> Result<Stage, ()> {
    match val {
        Value::Tuple(vals) => {
            if vals.len() != 2 {
                return Err(());
            }

            let mut it = vals.into_iter();

            let la = to_set(it.next().unwrap(), to_lexical_item_token)?;
            let w = to_set(it.next().unwrap(), to_so)?;

            Ok(Stage { la, w: Workspace::new(w) })
        },

        _ => Err(()),
    }
}

//  Derivation := [Stage]
pub fn to_derivation(val: Value) -> Result<Vec<Stage>, ()> {
    to_vec(val, to_stage)
}