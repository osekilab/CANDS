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