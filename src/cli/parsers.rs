use super::errors::{ self, Span, Context, SpanContextErrorTree };
use super::{ Type, LEXICAL_ITEM_TYPE, LEXICAL_ITEM_TOKEN_TYPE, LEXICON_TYPE,
    UNIVERSAL_GRAMMAR_TYPE, LEXICAL_ARRAY_TYPE, WORKSPACE_TYPE, STAGE_TYPE,
    DERIVATION_TYPE };

use codespan_reporting::files::{ SimpleFile };
use codespan_reporting::diagnostic::{ Diagnostic, Label };
use codespan_reporting::term::{ self, termcolor::{ ColorChoice, StandardStream }};

use nom::{ Finish, Parser, IResult };
use nom::branch::{ alt };
use nom::bytes::streaming::{ tag };
use nom::character::streaming::{ alpha1, alphanumeric0, multispace0, not_line_ending, one_of, digit1 };
use nom::combinator::{ eof, map };
use nom::error::{ Error as NomError };
use nom::multi::{ many0, many1, separated_list0 };
use nom::sequence::{ preceded };

use nom_supreme::error::{ GenericErrorTree, StackContext };
use nom_supreme::parser_ext::ParserExt;

use nom_locate::{ LocatedSpan };

use paste::paste;


use std::error::Error;
use std::str::FromStr;
use std::path::Path;



//  Internal value representation.
#[derive(Debug, Clone)]
pub enum Value {
    Feature(String),
    Vec(Vec<Expr>),
    Set(Vec<Expr>),
    Tuple(Vec<Expr>),
    Usize(usize),
}



//  Internal expression representation.
#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    Var(String),
}



//  Internal statement representation.
#[derive(Debug, Clone)]
pub enum Statement {
    Let(String, Type, Expr),
    Set(String, Expr),
    Init,
    Check(Expr),
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
    let (s, first) =
        many1(
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")
        )
            .context(Context::Id)
            .parse(s)?;

    let (s, second) = 
        many0(
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_")
        )
            .context(Context::Id)
            .parse(s)?;

    Ok((s, format!("{}{}", first.iter().collect::<String>(), second.iter().collect::<String>())))
}



fn feature_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = alt((tag("Feature"), tag("F")))
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

    Ok((s, LEXICAL_ITEM_TOKEN_TYPE!()))
}



fn lexical_item_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("Li")
        .context(Context::LexicalItemType)
        .parse(s)?;

    Ok((s, LEXICAL_ITEM_TYPE!()))
}



fn lexicon_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("Lex")
        .context(Context::LexiconType)
        .parse(s)?;

    Ok((s, LEXICON_TYPE!()))
}



fn universal_grammar_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("UG")
        .context(Context::UniversalGrammarType)
        .parse(s)?;

    Ok((s, UNIVERSAL_GRAMMAR_TYPE!()))
}



fn lexical_array_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("La")
        .context(Context::LexicalArrayType)
        .parse(s)?;

    Ok((s, LEXICAL_ARRAY_TYPE!()))
}



fn workspace_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("Wksp")
        .context(Context::WorkspaceType)
        .parse(s)?;

    Ok((s, WORKSPACE_TYPE!()))
}



fn stage_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("Stage")
        .context(Context::StageType)
        .parse(s)?;

    Ok((s, STAGE_TYPE!()))
}



fn derivation_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("Deriv")
        .context(Context::DerivationType)
        .parse(s)?;

    Ok((s, DERIVATION_TYPE!()))
}



fn so_type(s: Span) -> IResult<Span, Type, SpanContextErrorTree> {
    let (s, _) = tag("SO")
        .context(Context::SOType)
        .parse(s)?;

    Ok((s, Type::SO))
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
        lexicon_type,
        universal_grammar_type,
        lexical_array_type,
        workspace_type,
        stage_type,
        derivation_type,
        so_type,
        vector_type,
        set_type,
        tuple_type,
    )).context(Context::Type).parse(s)?;

    Ok((s, ty))
}



/// Parse a feature.
fn feature(s: Span) -> IResult<Span, Value, SpanContextErrorTree> {
    let (s, _) = tag("\"").parse(s)?;

    let (s, feature) =
        many1(
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890*=\'")
        )
            .context(Context::Feature)
            .parse(s)?;

    let (s, _) = tag("\"").parse(s)?;

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

        let (s, exprs) = separated_list0(
            preceded(multispace0, tag(",")),
            preceded(multispace0, expr),
        )
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag("]"))
            .context(Context::VecRight)
            .parse(s)?;

        Ok((s, Value::Vec(exprs)))
    } .. Context::Vec
}



parser_with_ctx! {
    fn set(s: Span) -> IResult<Span, Value, SpanContextErrorTree> {
        let (s, _) = tag("{")
            .context(Context::SetLeft)
            .parse(s)?;

        let (s, exprs) = separated_list0(
            preceded(multispace0, tag(",")),
            preceded(multispace0, expr),
        )
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag("}"))
            .context(Context::SetRight)
            .parse(s)?;

        Ok((s, Value::Set(exprs)))
    } .. Context::Set
}



parser_with_ctx! {
    fn tuple(s: Span) -> IResult<Span, Value, SpanContextErrorTree> {
        let (s, _) = tag("<")
            .context(Context::TupleLeft)
            .parse(s)?;

        let (s, exprs) = separated_list0(
            preceded(multispace0, tag(",")),
            preceded(multispace0, expr),
        )
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag(">"))
            .context(Context::TupleRight)
            .parse(s)?;

        Ok((s, Value::Tuple(exprs)))
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



fn expr(s: Span) -> IResult<Span, Expr, SpanContextErrorTree> {
    //  This parser will likely be more complex as we support binops/unops
    let (s, expr) = alt((
        map(id, |id| Expr::Var(id)).context(Context::Var),
        map(value, |val| Expr::Value(val))
    ))
        .context(Context::Expr)
        .parse(s)?;

    Ok((s, expr))
}



parser_with_ctx! {
    fn parse_let(s: Span) -> IResult<Span, Statement, SpanContextErrorTree> {
        let (s, _) = tag("let")
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

        let (s, expr) = preceded(multispace0, expr)
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag(";"))
            .context(Context::LetSemicolon)
            .parse(s)?;

        Ok((s, Statement::Let(id, ty, expr)))
    } .. Context::Let
}



parser_with_ctx! {
    fn parse_set(s: Span) -> IResult<Span, Statement, SpanContextErrorTree> {
        let (s, _) = tag("set")
            .context(Context::SetSet)
            .parse(s)?;

        let (s, id) = preceded(multispace0, id)
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag("="))
            .context(Context::SetEquals)
            .parse(s)?;

        let (s, expr) = preceded(multispace0, expr)
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag(";"))
            .context(Context::SetSemicolon)
            .parse(s)?;

        Ok((s, Statement::Set(id, expr)))
    } .. Context::Set
}



parser_with_ctx! {
    fn parse_init(s: Span) -> IResult<Span, Statement, SpanContextErrorTree> {
        let (s, _) = tag("init")
            .context(Context::InitInit)
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag(";"))
            .context(Context::InitSemicolon)
            .parse(s)?;

        Ok((s, Statement::Init))
    } .. Context::Init
}



parser_with_ctx! {
    fn parse_check(s: Span) -> IResult<Span, Statement, SpanContextErrorTree> {
        let (s, _) = tag("check")
            .context(Context::CheckCheck)
            .parse(s)?;

        let (s, expr) = preceded(multispace0, expr)
            .parse(s)?;

        let (s, _) = preceded(multispace0, tag(";"))
            .context(Context::CheckSemicolon)
            .parse(s)?;

        Ok((s, Statement::Check(expr)))
    } .. Context::Check
}



fn statement(s: Span) -> IResult<Span, Statement, SpanContextErrorTree> {
    let (s, stmt) = alt((
        parse_let,
        parse_set,
        parse_init,
        parse_check
    )).context(Context::Statement).parse(s)?;

    Ok((s, stmt))
}



fn comment(s: Span) -> IResult<Span, (), SpanContextErrorTree> {
    let (s, _) = tag("#").context(Context::Comment).parse(s)?;
    let (s, _) = not_line_ending.parse(s)?;

    Ok((s, ()))
}



pub fn line(s: Span) -> IResult<Span, Option<Statement>, SpanContextErrorTree> {
    let (s, line) = alt((
        map(statement, |stmt| Some(stmt)),
        map(comment,   |_|    None)
    )).parse(s)?;

    Ok((s, line))
}



pub struct Statements<'a> {
    files: SimpleFile<String, &'a str>,
    buffer: Span<'a>,
}



#[derive(Debug, Clone)]
pub enum StatementsAction {
    //  Statement
    Statement(Statement),
    //  Maybe a statement? Need more data to figure out
    MaybeStatement,
    //  Not a statement (EOF or syntax error)
    NoStatement,
}



impl<'a> Statements<'a> {
    pub fn make(s: &'a str, file_path: Option<&Path>) -> Self {
        let files =
            SimpleFile::new(
                file_path
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| format!("[stdin]")),
                s
            );

        let file_id = ();
        let buffer = Span::new_extra(s, file_id);

        Self { files, buffer }
    }

    pub fn next(&mut self) -> StatementsAction {
        loop {
            //  Consume as much whitespace as we can.
            let s =
                match nom::character::complete::multispace0::<Span, NomError<Span>>.parse(self.buffer) {
                    Ok((s, _)) => s,
                    //  If we fail here, something is wrong. The iterator ends here.
                    _ => break,
                };

            //  Check if we are at EOF. If yes, the iterator ends here.
            if let Ok(_) = eof::<Span, NomError<Span>>(s) {
                break;
            }

            //  Otherwise, parse.
            match line(s) {
                //  Parsed a statement, not a comment
                Ok((s, Some(stmt))) => {
                    //  Update buffer because we just parsed a line
                    self.buffer = s;
                    return StatementsAction::Statement(stmt)
                },

                //  Parsed a comment
                Ok((_, None)) => {},

                //  Need more data
                Err(nom::Err::Incomplete(_)) => {
                    return StatementsAction::MaybeStatement
                },

                //  Parser error
                Err(nom::Err::Error(error)) |
                Err(nom::Err::Failure(error)) => {
                    let diags = errors::make_diagnostics(&error);

                    let writer = StandardStream::stderr(ColorChoice::Always);
                    let config = codespan_reporting::term::Config::default();

                    for diag in diags.iter() {
                        term::emit(&mut writer.lock(), &config, &self.files, diag).unwrap();
                    }

                    break
                },
            }
        }

        StatementsAction::NoStatement
    }
}