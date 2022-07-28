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