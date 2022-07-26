pub mod errors;
pub mod parsers;
pub mod eval {}



use crate::prelude::*;

use std::io::{ self, BufRead, Write };



#[derive(Debug)]
pub enum Type {
    Feature,
    Vec(Box<Type>),
    Set(Box<Type>),
    Tuple(Vec<Type>),
    Usize,
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

use LEXICAL_ITEM_TYPE;
use LEXICAL_ITEM_TOKEN_TYPE;



#[derive(Debug)]
pub enum Value {
    Feature(Feature),
    FVec(Vec<Feature>),
    FSet(Set<Feature>),
    LexicalItem(LexicalItem),
}



#[derive(Debug)]
pub enum Statement {
    Echo(Value),
}



pub fn run() {
    let mut buffer = String::new();

    //  Keep a var -> val map here

    loop {
        buffer.clear();
        // io::stdout().write_all(b"> ").unwrap();
        // io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();

        let input = buffer.trim();

        if input == "q" {
            break;
        }

        if let Ok(statement) = parsers::parse(&input) {
            my_info!("{:?}", statement);

            //  Evaluate statements.
            //  Let binding should evaluate the RHS (this means type-checking, also stuff like [] @ [])
            //  Other statements should have real effects, like calling cands::deriv::is_derivation
        }
    }
}