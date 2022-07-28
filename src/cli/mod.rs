pub mod errors;
pub mod parsers;
pub mod tyck;
pub mod eval;
pub mod conv;



use crate::prelude::*;

use std::collections::{ HashMap };
use std::fmt;
use std::fs::{ File };
use std::io::{ self, BufReader, Read, Write };
use std::path::{ Path };

use parsers::{ Expr, Statement, StatementsAction };



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
    //  Buffer for stdin
    let mut buffer = String::new();
    let mut engine = Engine::new();

    //  Actual input
    let mut input = String::new();

    let mut clear_input: bool = true;

    loop {
        buffer.clear();
        io::stdout().write_all(
            if clear_input { b">>> " } else { b"... " }
        ).unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();

        if clear_input {
            input.clear();
        }
        input.extend(buffer.trim().chars());

        if input == "q" {
            break;
        }

        let mut stmts = parsers::Statements::make(&input, None);

        clear_input = true;
        loop {
            match stmts.next() {
                StatementsAction::Statement(stmt) => {
                    engine.process(stmt);
                },

                StatementsAction::MaybeStatement => {
                    clear_input = false;
                    break;
                },

                StatementsAction::NoStatement => {
                    break;
                },
            }
        }
    }
}



pub fn run_file(path: &Path) {
    let mut buffer = String::new();
    let mut engine = Engine::new();

    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut buffer).unwrap();

    let mut stmts = parsers::Statements::make(&buffer, None);

    loop {
        match stmts.next() {
            StatementsAction::Statement(stmt) => {
                engine.process(stmt);
            },

            _ => { break },
        }
    }
}