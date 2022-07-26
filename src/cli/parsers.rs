use super::{ Type };
use super::errors::{ self, Span, Context, SpanContextErrorTree };

use codespan_reporting::files::{ SimpleFiles };
use codespan_reporting::diagnostic::{ Diagnostic, Label };
use codespan_reporting::term::{ self, termcolor::{ ColorChoice, StandardStream }};

use nom::{ Finish, Parser, IResult };
use nom::branch::{ alt };
use nom::bytes::complete::{ tag };
use nom::character::complete::{ alpha1, alphanumeric0, multispace0, one_of, digit1 };
use nom::multi::{ many0, many1, separated_list0 };
use nom::sequence::{ preceded };

use nom_supreme::error::{ GenericErrorTree, StackContext };
use nom_supreme::parser_ext::ParserExt;

use nom_locate::{ LocatedSpan };

use paste::paste;


use std::error::Error;
use std::str::FromStr;



//  Internal value representation.
#[derive(Debug)]
pub enum Value {
    Feature(String),
    Vec(Vec<Value>),
    Set(Vec<Value>),
    Tuple(Vec<Value>),
    Usize(usize),
    //  Does anything about SO go here?
}



//  Internal expression representation.
#[derive(Debug)]
pub enum Expr {
    Value(Value),
    Var(String),
}



//  Internal statement representation.
#[derive(Debug)]
pub enum Statement {
    Let(String, Type, Value),
}



macro_rules! parser_with_ctx {
    {
        fn $fn_name:ident($s:ident: Span) -> IResult<Span, $ret_ty:ty, SpanContextErrorTree> $blk:block .. $ctx:expr
    } => {
        paste! {
            fn [< $fn_name _ >]($s: Span) -> IResult<Span, $ret_ty, SpanContextErrorTree> $blk

            fn $fn_name($s: Span) -> IResult<Span, $ret_ty, SpanContextErrorTree> {
                [< $fn_name _ >].context($ctx).parse($s)
            }
        }
    };
}



/// Parse an identifier.
fn id(s: Span) -> IResult<Span, String, SpanContextErrorTree> {
    let (s, first) = alpha1
        .context(Context::Id)
        .parse(s)?;

    let (s, second) = alphanumeric0
        .context(Context::Id)
        .parse(s)?;

    Ok((s, format!("{}{}", first, second)))
}



fn feature_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("Feature")
        .context(Context::FeatureType)
        .parse(s)?;

    Ok((s, Type::Feature))
}



fn usize_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("usize")
        .context(Context::UsizeType)
        .parse(s)?;

    Ok((s, Type::Usize))
}



fn lexical_item_token_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("Lit")
        .context(Context::LexicalItemTokenType)
        .parse(s)?;

    Ok((s, super::LEXICAL_ITEM_TOKEN_TYPE!()))
}



fn lexical_item_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("Li")
        .context(Context::LexicalItemType)
        .parse(s)?;

    Ok((s, super::LEXICAL_ITEM_TYPE!()))
}



parser_with_ctx! {
    fn vector_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
        let (s, _) = tag("[")
            .context(Context::VecTypeLeft)
            .parse(s)?;

        let (s, ty) = preceded(multispace0, parse_type)
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag("]"))
            .context(Context::VecTypeRight)
            .parse(s)?;

        Ok((s, Type::Vec(Box::new(ty))))
    } .. Context::VecType
}



parser_with_ctx! {
    fn set_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
        let (s, _) = tag("{")
            .context(Context::SetTypeLeft)
            .parse(s)?;

        let (s, ty) = preceded(multispace0, parse_type)
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag("}"))
            .context(Context::SetTypeRight)
            .parse(s)?;

        Ok((s, Type::Set(Box::new(ty))))
    } .. Context::SetType
}



parser_with_ctx! {
    fn tuple_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
        let (s, _) = tag("<")
            .context(Context::TupleTypeLeft)
            .parse(s)?;

        let (s, tys) = separated_list0(
            preceded(multispace0, tag(",")),
            preceded(multispace0, parse_type),
        )
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag(">"))
            .context(Context::TupleTypeRight)
            .parse(s)?;

        Ok((s, Type::Tuple(tys)))
    } .. Context::TupleType
}



/// Parse a type.
fn parse_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, ty) = alt((
        feature_type,
        usize_type,
        lexical_item_token_type,
        lexical_item_type,
        vector_type,
        set_type,
        tuple_type,
    )).context(Context::Type).parse(s)?;

    Ok((s, ty))
}



/// Parse a feature.
fn feature(s: Span) -> IResult<Span, Value, SpanContextErrorTree> {
    let (s, feature) =
        many1(
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890*=\'")
        )
            .context(Context::Feature)
            .parse(s)?;

    Ok((s, Value::Feature(format!("{}", feature.iter().collect::<String>()))))
}



fn usize_value(s: Span) -> IResult<Span, Value, SpanContextErrorTree> {
    let (s, x) = digit1
        .context(Context::Usize)
        .parse(s)?;

    Ok((s, Value::Usize(usize::from_str(&x).unwrap())))
}



parser_with_ctx! {
    fn vector(s: Span) -> IResult<Span, Value, SpanContextErrorTree> {
        let (s, _) = tag("[")
            .context(Context::VecLeft)
            .parse(s)?;

        let (s, vals) = separated_list0(
            preceded(multispace0, tag(",")),
            preceded(multispace0, value),
        )
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag("]"))
            .context(Context::VecRight)
            .parse(s)?;

        Ok((s, Value::Vec(vals)))
    } .. Context::Vec
}



parser_with_ctx! {
    fn set(s: Span) -> IResult<Span, Value, SpanContextErrorTree> {
        let (s, _) = tag("{")
            .context(Context::SetLeft)
            .parse(s)?;

        let (s, vals) = separated_list0(
            preceded(multispace0, tag(",")),
            preceded(multispace0, value),
        )
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag("}"))
            .context(Context::SetRight)
            .parse(s)?;

        Ok((s, Value::Set(vals)))
    } .. Context::Set
}



parser_with_ctx! {
    fn tuple(s: Span) -> IResult<Span, Value, SpanContextErrorTree> {
        let (s, _) = tag("<")
            .context(Context::TupleLeft)
            .parse(s)?;

        let (s, vals) = separated_list0(
            preceded(multispace0, tag(",")),
            preceded(multispace0, value),
        )
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag(">"))
            .context(Context::TupleRight)
            .parse(s)?;

        Ok((s, Value::Tuple(vals)))
    } .. Context::Tuple
}



fn value(s: Span) -> IResult<Span, Value, SpanContextErrorTree> {
    let (s, val) = alt((
        feature,
        usize_value,
        vector,
        set,
        tuple,
    )).context(Context::Value).parse(s)?;

    Ok((s, val))
}



parser_with_ctx! {
    fn parse_let(s: Span) -> IResult<Span, Statement, SpanContextErrorTree> {
        let (s, _) = preceded(multispace0, tag("let"))
            .context(Context::LetLet)
            .parse(s)?;

        let (s, id) = preceded(multispace0, id)
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag(":"))
            .context(Context::LetColon)
            .parse(s)?;

        let (s, ty) = preceded(multispace0, parse_type)
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag("="))
            .context(Context::LetEquals)
            .parse(s)?;

        let (s, val) = preceded(multispace0, value)
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag(";"))
            .context(Context::LetSemicolon)
            .parse(s)?;

        Ok((s, Statement::Let(id, ty, val)))
    } .. Context::Let
}



fn statement(s: Span) -> IResult<Span, Statement, SpanContextErrorTree> {
    let (s, stmt) = alt((
        parse_let,
        parse_let
    )).context(Context::Statement).parse(s)?;

    Ok((s, stmt))
}



pub fn parse(s: &str) -> Result<Statement, ()> {
    let filename = format!("[stdin]");
    let mut files = SimpleFiles::new();
    let file_id = files.add(filename, s);

    let span = Span::new_extra(s, file_id);
    let res = statement(span).finish();

    match res {
        Ok(res) => Ok(res.1),
        Err(error) => {
            // eprintln!("{:#?}", error);

            let diags = errors::make_diagnostics(&error);

            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config::default();

            for diag in diags.iter() {
                term::emit(&mut writer.lock(), &config, &files, diag).unwrap();
            }

            Err(())
        },
    }
}