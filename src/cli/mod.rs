pub mod errors;
pub mod parsers;

pub mod tyck {
    use crate::prelude::*;

    use super::{
        Value,
        Type, LEXICAL_ITEM_TOKEN_TYPE, TRANSFERRED_SO_TYPE,
        parsers::{
            Expr as RawExpr,
            Value as RawValue,
        }
    };

    use std::collections::{ HashMap };

    //  Big to-do: add codespan support here???
    //  The RawExpr and RawValues should probably have a codespan in them.

    //  Tyck should be called for every let and set statement.
    //  The user should see type error messages!

    fn tyck_var(ty: Type, id: &str, map: &HashMap<String, (Type, Value)>) -> Result<(), ()> {
        match map.get(id).map(|(var_ty, _)| *var_ty == ty) {
            Some(true) => Ok(()),
            _ => Err(()),
        }
    }

    fn tyck_feature(expr: &RawExpr, map: &HashMap<String, (Type, Value)>) -> Result<(), ()> {
        match expr {
            &RawExpr::Value(RawValue::Feature(_)) => Ok(()),
            &RawExpr::Var(ref id) => tyck_var(Type::Feature, id, map),
            _ => Err(()),
        }
    }

    fn tyck_vec(ty: &Type, expr: &RawExpr, map: &HashMap<String, (Type, Value)>) -> Result<(), ()> {
        match expr {
            &RawExpr::Value(RawValue::Vec(ref vals)) => {
                vals.iter()
                    .map(|expr| tyck(ty, expr, map))
                    .fold(
                        Ok(()),
                        |res1, res2| {
                            match (res1, res2) {
                                (Ok(_), Ok(_)) => Ok(()),
                                _ => Err(()),
                            }
                        }
                    )
            },

            &RawExpr::Var(ref id) => tyck_var(Type::Vec(Box::new(ty.clone())), id, map),
            _ => Err(()),
        }
    }

    fn tyck_set(ty: &Type, expr: &RawExpr, map: &HashMap<String, (Type, Value)>) -> Result<(), ()> {
        match expr {
            &RawExpr::Value(RawValue::Set(ref vals)) => {
                vals.iter()
                    .map(|expr| tyck(ty, expr, map))
                    .fold(
                        Ok(()),
                        |res1, res2| {
                            match (res1, res2) {
                                (Ok(_), Ok(_)) => Ok(()),
                                _ => Err(()),
                            }
                        }
                    )
            },

            &RawExpr::Var(ref id) => tyck_var(Type::Set(Box::new(ty.clone())), id, map),
            _ => Err(()),
        }
    }

    fn tyck_tuple(tys: &[Type], expr: &RawExpr, map: &HashMap<String, (Type, Value)>) -> Result<(), ()> {
        match expr {
            &RawExpr::Value(RawValue::Tuple(ref vals)) => {
                if tys.len() != vals.len() {
                    return Err(());
                }

                tys.iter()
                    .zip(vals.iter())
                    .map(|(ty, expr)| tyck(ty, expr, map))
                    .fold(
                        Ok(()),
                        |res1, res2| {
                            match (res1, res2) {
                                (Ok(_), Ok(_)) => Ok(()),
                                _ => Err(()),
                            }
                        }
                    )
            },

            &RawExpr::Var(ref id) => tyck_var(Type::Tuple(tys.to_vec()), id, map),
            _ => Err(()),
        }
    }

    fn tyck_usize(expr: &RawExpr, map: &HashMap<String, (Type, Value)>) -> Result<(), ()> {
        match expr {
            &RawExpr::Value(RawValue::Usize(_)) => Ok(()),
            &RawExpr::Var(ref id) => tyck_var(Type::Usize, id, map),
            _ => Err(()),
        }
    }

    fn tyck_so(expr: &RawExpr, map: &HashMap<String, (Type, Value)>) -> Result<(), ()> {
        tyck(&LEXICAL_ITEM_TOKEN_TYPE!(), expr, map)
            .or_else(|_| tyck(&Type::Set(Box::new(Type::SO)), expr, map))
            .or_else(|_| tyck(&TRANSFERRED_SO_TYPE!(), expr, map))
            .or_else(|_| {
                match expr {
                    &RawExpr::Var(ref id) =>
                        tyck_var(Type::SO, id, map),
                    _ => Err(()),
                }
            })
    }

    pub fn tyck(ty: &Type, expr: &RawExpr, map: &HashMap<String, (Type, Value)>) -> Result<(), ()> {
        match ty {
            Type::Feature => tyck_feature(expr, map),
            Type::Vec(ty) => tyck_vec(ty, expr, map),
            Type::Set(ty) => tyck_set(ty, expr, map),
            Type::Tuple(tys) => tyck_tuple(tys, expr, map),
            Type::Usize => tyck_usize(expr, map),
            Type::SO => tyck_so(expr, map),
        }
    }
}

pub mod eval {
    use crate::prelude::*;

    use super::{
        Value,
        Type,
        parsers::{
            Expr as RawExpr,
            Value as RawValue,
        }
    };

    use std::collections::{ HashMap };

    fn eval_exprs(exprs: Vec<RawExpr>, map: &HashMap<String, (Type, Value)>) -> Result<Vec<Value>, ()> {
        exprs.into_iter()
            .map(|expr| eval(expr, map))
            .fold(
                Ok(vec![]),
                |vec, val| {
                    vec.and_then(|mut vec| {
                        val.map(|val| {
                            vec.push(val);
                            vec
                        })
                    })
                }
            )
    }

    fn eval_value(val: RawValue, map: &HashMap<String, (Type, Value)>) -> Result<Value, ()> {
        match val {
            RawValue::Feature(feature) =>
                Ok(Value::Feature(Feature::new(feature))),

            RawValue::Vec(exprs) =>
                eval_exprs(exprs, map).map(Value::Vec),

            RawValue::Set(exprs) =>
                eval_exprs(exprs, map).map(Value::Set),

            RawValue::Tuple(exprs) => 
                eval_exprs(exprs, map).map(Value::Tuple),

            RawValue::Usize(x) =>
                Ok(Value::Usize(x)),
        }
    }

    pub fn eval(expr: RawExpr, map: &HashMap<String, (Type, Value)>) -> Result<Value, ()> {
        match expr {
            RawExpr::Value(val) =>
                eval_value(val, map),

            RawExpr::Var(var) =>
                map.get(&var).map(|(ty, val)| val).cloned().ok_or(()),
        }
    }
}



use crate::prelude::*;

use std::collections::{ HashMap };
use std::fmt;
use std::fs::{ File };
use std::io::{ self, BufReader, Read, Write };
use std::path::{ Path };

use parsers::{ Expr, Statement };



//  Type representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Feature,
    Vec(Box<Type>),
    Set(Box<Type>),
    Tuple(Vec<Type>),
    Usize,
    SO, // = rec(t. Lit + {t} + <t, [F], {F}>)
}



impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Feature => write!(f, "F"),
            Type::Vec(ty) => write!(f, "[{}]", ty),
            Type::Set(ty) => write!(f, "{{{}}}", ty),
            Type::Tuple(tys) =>
                write!(
                    f,
                    "<{}>",
                    tys.iter().map(|ty| format!("{}", ty)).reduce(|ty1, ty2| format!("{}, {}", ty1, ty2)).unwrap_or_else(|| format!(""))
                ),
            Type::Usize => write!(f, "usize"),
            Type::SO => write!(f, "SO"),
        }
    }
}

//  Type aliases.

macro_rules! LEXICAL_ITEM_TYPE {
    () => {
        Type::Tuple(vec![
            Type::Set(Box::new(Type::Feature)),
            Type::Set(Box::new(Type::Feature)),
            Type::Vec(Box::new(Type::Feature)),
        ])
    };
}



macro_rules! LEXICAL_ITEM_TOKEN_TYPE {
    () => {
        Type::Tuple(vec![
            Type::Tuple(vec![
                Type::Set(Box::new(Type::Feature)),
                Type::Set(Box::new(Type::Feature)),
                Type::Vec(Box::new(Type::Feature)),
            ]),
            Type::Usize
        ])
    };
}



macro_rules! TRANSFERRED_SO_TYPE {
    () => {
        Type::Tuple(vec![
            Type::SO,
            Type::Vec(Box::new(Type::Feature)),
            Type::Set(Box::new(Type::Feature)),
        ])
    };
}



macro_rules! LEXICON_TYPE {
    () => {
        Type::Set(Box::new(LEXICAL_ITEM_TYPE!()))
    };
}



macro_rules! UNIVERSAL_GRAMMAR_TYPE {
    () => {
        Type::Tuple(vec![
            Type::Set(Box::new(Type::Feature)),
            Type::Set(Box::new(Type::Feature)),
            Type::Set(Box::new(Type::Feature)),
        ])
    };
}



macro_rules! LEXICAL_ARRAY_TYPE {
    () => {
        Type::Set(Box::new(LEXICAL_ITEM_TOKEN_TYPE!()))
    };
}



macro_rules! WORKSPACE_TYPE {
    () => {
        Type::Set(Box::new(Type::SO))
    };
}



macro_rules! STAGE_TYPE {
    () => {
        Type::Tuple(vec![
            LEXICAL_ARRAY_TYPE!(),
            WORKSPACE_TYPE!(),
        ])
    };
}



macro_rules! DERIVATION_TYPE {
    () => {
        Type::Vec(Box::new(STAGE_TYPE!()))
    };
}

pub(crate) use LEXICAL_ITEM_TYPE;
pub(crate) use LEXICAL_ITEM_TOKEN_TYPE;
pub(crate) use TRANSFERRED_SO_TYPE;
pub(crate) use LEXICON_TYPE;
pub(crate) use UNIVERSAL_GRAMMAR_TYPE;
pub(crate) use LEXICAL_ARRAY_TYPE;
pub(crate) use WORKSPACE_TYPE;
pub(crate) use STAGE_TYPE;
pub(crate) use DERIVATION_TYPE;



//  External value representation.
#[derive(Debug, Clone)]
pub enum Value {
    Feature(Feature),
    Vec(Vec<Value>),
    Set(Vec<Value>),
    Tuple(Vec<Value>),
    Usize(usize),
}



mod conv {
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
}



//  Should have a mut self method for processing each statement
//  Keep a var -> val map here
struct Engine {
    map: HashMap<String, (Type, Value)>,
    lex: Option<Lexicon>,
    ug: Option<UniversalGrammar<BasicTriggers>>,
    il: Option<ILanguage<BasicTriggers>>,
}

impl Engine {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            lex: None,
            ug: None,
            il: None,
        }
    }

    fn r#let(&mut self, id: String, ty: Type, expr: Expr) -> Result<(), ()> {
        if let Err(_) = tyck::tyck(&ty, &expr, &self.map) {
            my_error!("let: Type error. Does not typecheck to {}", ty);
            return Err(());
        }

        let val = match eval::eval(expr, &self.map) {
            Ok(val) => val,
            _ => {
                my_error!("let: Evaluation error.");
                return Err(());
            }
        };

        my_info!("let: Binding the name `{}` to {:?}", id, val);
        if let Some(old_val) = self.map.insert(id, (ty, val)) {
            my_info!("let: This overwrites the old binding to {:?}", old_val.1);
        }
        Ok(())
    }

    fn set(&mut self, id: String, expr: Expr) -> Result<(), ()> {
        if id != "lex" && id != "ug" {
            my_error!("set: Unknown global variable `{}`.", id);
            return Err(());
        }

        if id == "lex" {
            if let Err(_) = tyck::tyck(&LEXICON_TYPE!(), &expr, &self.map) {
                my_error!("set: Type error.");
                return Err(());
            }
        
            let val = match eval::eval(expr, &self.map) {
                Ok(val) => val,
                _ => {
                    my_error!("set: Evaluation error.");
                    return Err(());
                },
            };

            let lexicon = match conv::to_lexicon(val) {
                Ok(lexicon) => lexicon,
                _ => {
                    my_error!("set: Value conversion error.");
                    return Err(());
                },
            };

            my_info!("set: Setting lex to {:?}", lexicon);
            self.lex = Some(lexicon);
            return Ok(());
        }
        else if id == "ug" {
            if let Err(_) = tyck::tyck(&UNIVERSAL_GRAMMAR_TYPE!(), &expr, &self.map) {
                my_error!("set: Type error.");
                return Err(());
            }
        
            let val = match eval::eval(expr, &self.map) {
                Ok(val) => val,
                _ => {
                    my_error!("set: Evaluation error.");
                    return Err(());
                },
            };

            let ug = match conv::to_ug(val) {
                Ok(ug) => ug,
                _ => {
                    my_error!("set: Value conversion error.");
                    return Err(());
                },
            };

            my_info!("set: Setting UG to {:?}", ug);
            self.ug = Some(ug);
            return Ok(());
        }

        unreachable!()
    }

    fn init(&mut self) -> Result<(), ()> {
        if self.lex.is_none() || self.ug.is_none() {
            my_error!("init: Failed to initialize I-language. Did you set the lexicon and UG?");
            return Err(());
        }

        let lex = std::mem::replace(&mut self.lex, None).unwrap();
        let ug = std::mem::replace(&mut self.ug, None).unwrap();

        self.il = Some(ILanguage { lex, ug });

        Ok(())
    }

    fn check(&self, expr: Expr) -> Result<(), ()> {
        if self.il.is_none() {
            my_error!("check: I-language is not initialized. Run `init` first.");
            return Err(());
        }

        if let Err(_) = tyck::tyck(&DERIVATION_TYPE!(), &expr, &self.map) {
            my_error!("set: Type error.");
            return Err(());
        }
    
        let val = match eval::eval(expr, &self.map) {
            Ok(val) => val,
            _ => {
                my_error!("set: Evaluation error.");
                return Err(());
            },
        };

        let derivation = match conv::to_derivation(val) {
            Ok(derivation) => derivation,
            _ => {
                my_error!("set: Value conversion error.");
                return Err(());
            },
        };

        my_info!("Checking the derivation...");
        if is_derivation::<BasicTriggers>(self.il.as_ref().unwrap(), &derivation) {
            my_info!("Valid derivation.");
        }
        else {
            my_error!("Invalid derivation.");
        }
        return Ok(());
    }

    fn process(&mut self, stmt: Statement) {
        //  Evaluate statements.
        //  Let binding should evaluate the RHS (this means type-checking, also stuff like [] @ [])
        //  Other statements should have real effects, like calling cands::deriv::is_derivation
        my_info!("{:?}", stmt);

        match stmt {
            Statement::Let(id, ty, expr) => {
                self.r#let(id, ty, expr);
            },

            Statement::Set(id, expr) => {
                self.set(id, expr);
            },

            Statement::Init => {
                self.init();
            },

            Statement::Check(expr) => {
                self.check(expr);
            },
        };
    }
}



pub fn run_stdin() {
    let mut buffer = String::new();
    let mut engine = Engine::new();

    loop {
        buffer.clear();
        io::stdout().write_all(b"> ").unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();

        let input = buffer.trim();

        if input.trim() == "q" {
            break;
        }

        for stmt in parsers::Statements::make(&input, None) {
            engine.process(stmt);
        }
    }
}



pub fn run_file(path: &Path) {
    let mut buffer = String::new();
    let mut engine = Engine::new();

    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut buffer).unwrap();

    for stmt in parsers::Statements::make(&buffer, Some(path)) {
        engine.process(stmt);
    }
}