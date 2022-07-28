

use codespan_reporting::files::{ SimpleFiles };
use codespan_reporting::diagnostic::{ Diagnostic, Label };
use codespan_reporting::term::{ self, termcolor::{ ColorChoice, StandardStream }};

use nom::{ Finish, Parser, IResult };
use nom::branch::{ alt };
use nom::bytes::complete::{ tag };
use nom::character::complete::{ alpha1, alphanumeric0, multispace0 };
use nom::multi::{ many0 };
use nom::sequence::{ preceded };

use nom_supreme::error::{ GenericErrorTree, StackContext };
use nom_supreme::parser_ext::ParserExt;

use nom_locate::{ LocatedSpan };

use paste::paste;


use std::error::Error;



type FileId = ();
pub type Span<'a> = LocatedSpan<&'a str, FileId>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Context {
    FeatureType,
    VecType, VecTypeLeft, VecTypeRight,
    SetType, SetTypeLeft, SetTypeRight,
    TupleType, TupleTypeLeft, TupleTypeRight,
    LexicalItemType,
    LexicalItemTokenType,
    //  Transferred SO should not have a dedicated type keyword
    LexiconType,
    UniversalGrammarType,
    LexicalArrayType,
    WorkspaceType,
    StageType,
    DerivationType,
    UsizeType,
    SOType,
    Type,

    Feature,
    Vec, VecLeft, VecRight,
    Set, SetLeft, SetRight,
    Tuple, TupleLeft, TupleRight,
    Usize,
    Value, Var,
    Expr,

    Id,

    Let, LetLet, LetColon, LetEquals, LetSemicolon,
    SetStmt, SetSet, SetEquals, SetSemicolon,
    Init, InitInit, InitSemicolon,
    Check, CheckCheck, CheckSemicolon,
    Statement,

    Comment,
}

pub type SpanContextErrorTree<'a> = GenericErrorTree<Span<'a>, Span<'a>, Context, Box<dyn Error + Send + Sync + 'static>>;



fn make_message(is_fail: bool, stack_ctx: &StackContext<Context>) -> Option<String> {
    match stack_ctx {
        StackContext::Context(ref ctx) => Some(
            format!(
                "{} {}",
                if is_fail { "Failed to parse" } else { "Tried to parse this as" },
                match ctx {
                    Context::FeatureType => "the type keyword `Feature`, or `F`",
                    Context::VecType => "a vector type",
                    Context::VecTypeLeft => "a left bracket (\'[\')",
                    Context::VecTypeRight => "a right bracket (\']\')",
                    Context::SetType => "a set type",
                    Context::SetTypeLeft => "a left brace (\'{\')",
                    Context::SetTypeRight => "a right brace (\'}\')",
                    Context::TupleType => "a tuple type",
                    Context::TupleTypeLeft => "a less-than sign (\'<\')",
                    Context::TupleTypeRight => "a greater-than sign (\'>\')",
                    Context::LexicalItemType => "the type keyword `Li`",
                    Context::LexicalItemTokenType => "the type keyword `Lit`",
                    Context::LexiconType => "the type keyword `Lex`",
                    Context::UniversalGrammarType => "the type keyword `UG`",
                    Context::LexicalArrayType => "the type keyword `La`",
                    Context::WorkspaceType => "the type keyword `Wksp`",
                    Context::StageType => "the type keyword `Stage`",
                    Context::DerivationType => "the type keyword `Deriv`",
                    Context::UsizeType => "the type keyword `usize`",
                    Context::SOType => "the type keyword `SO`",
                    Context::Type => "type annotation",

                    Context::Feature => "a feature",
                    Context::Vec => "a vector",
                    Context::VecLeft => "a left bracket (\'[\')",
                    Context::VecRight => "a right bracket (\']\')",
                    Context::Set => "a set",
                    Context::SetLeft => "a left brace (\'{\')",
                    Context::SetRight => "a right brace (\'}\')",
                    Context::Tuple => "a tuple",
                    Context::TupleLeft => "a less-than sign (\'<\')",
                    Context::TupleRight => "a greater-than sign (\'>\')",
                    Context::Usize => "a `usize`",
                    Context::Value => "a value",
                    Context::Var => "a variable",
                    Context::Expr => "an expression",

                    Context::Id => "an identifier",

                    Context::Let => "a let statement",
                    Context::LetLet => "the keyword `let`",
                    Context::LetColon => "a colon (\':\')",
                    Context::LetEquals => "an equals sign (\'=\')",
                    Context::LetSemicolon => "a semicolon (\';\')",

                    Context::SetStmt => "a set statement",
                    Context::SetSet => "the keyword `set`",
                    Context::SetEquals => "an equals sign (\'=\')",
                    Context::SetSemicolon => "a semicolon (\';\')",

                    Context::Init => "an init statement",
                    Context::InitInit => "the keyword `init`",
                    Context::InitSemicolon => "a semicolon (\';\')",

                    Context::Check => "a check statement",
                    Context::CheckCheck => "the keyword `check`",
                    Context::CheckSemicolon => "a semicolon (\';\')",

                    Context::Statement => "a statement",

                    Context::Comment => "a comment",
                }
            )
        ),
        _ => None,
    }
}

fn make_label<'a>(
    is_primary: bool,
    location: Span,
    stack_ctx: &StackContext<Context>
) -> Label<FileId> {
    let begin = location.location_offset();
    let end = begin + location.fragment().len();

    let label = if is_primary {
        Label::primary(location.extra, begin..end)
    }
    else {
        Label::secondary(location.extra, begin..end)
    };

    label.with_message(
        make_message(is_primary, stack_ctx)
            .unwrap_or(format!("Unknown message"))
    )
}

fn make_diagnostics_<'a>(
    error: &SpanContextErrorTree<'a>,
    steps: &[usize], // e.g. Step 1.3.1 ...
) -> Vec<Diagnostic<FileId>> {
    match error {
        //  We ignore this case.
        SpanContextErrorTree::Base { .. } => vec![],

        SpanContextErrorTree::Stack { ref base, ref contexts } => {
            let mut diags = make_diagnostics_(base, steps);

            let labels = contexts.iter()
                .enumerate()
                .map(|(k, loc_ctx)| (k == 0, loc_ctx))
                .map(|(b, (loc, ctx))| make_label(b, *loc, ctx))
                .collect();

            let steps_msg = format!("Step {}",
                steps.iter()
                    .map(|x| format!("{}", x+1))
                    .reduce(|x1, x2| {
                        format!("{}.{}", x1, x2)
                    })
                    .unwrap_or(String::new())
            );

            let message = contexts.last().and_then(|(_, stack_ctx)| {
                make_message(true, stack_ctx)
            }).unwrap_or(format!("Unknown error"));

            let diag = Diagnostic::error()
                .with_labels(labels)
                .with_message(format!("{}: {}", steps_msg, message));

            diags.insert(0, diag);
            diags
        },

        SpanContextErrorTree::Alt(ref alts) => {
            let mut new_steps = steps.to_owned();
            new_steps.push(0);

            alts.iter()
                .enumerate()
                .map(|(k, alt)| {
                    *new_steps.last_mut().unwrap() = k;
                    make_diagnostics_(alt, &new_steps)
                })
                .flatten()
                .collect()
        },
    }
}

pub fn make_diagnostics<'a>(
    error: &SpanContextErrorTree<'a>,
) -> Vec<Diagnostic<FileId>> {
    let steps = vec![ 0 ];
    make_diagnostics_(error, &steps)
}