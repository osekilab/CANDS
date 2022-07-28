# User docs

`cands` is an implementation of Collins and Stabler (2016).

You can provide a derivation, and check if it meets their criterion!

If this is your first time to `cands`, start from the **Getting Started** section.

# How to use

`cands` is like `python` -- it's an interpreter. If you run `cands` by itself, a REPL will start and you can type in commands:

```
$ cands
> ...
```

You can also write a script and load it with `cands` like this:

```
$ cands my_script.cands
```

If you are running `cands` using the Rust compiler `cargo`, then you can run `cands` with:

```
$ cargo run
```

And load a script with:

```
$ cargo run -- my_script.cands
```

# Getting started

First, start `cands`:

```
$ cands
```

Although the whole point of `cands` is to check derivations, we need to walk through some basics of how `cands` works before we get to that.

`cands` supports a very simple programming language. It accepts four kinds of statements, and one of them is a `let` statement, like this:

```
> let acc: Feature = "acc";
```

`let` statements like this introduce **typed variable bindings**. The above statement says we think `"acc"` is a value of type `Feature`, and if so we would like to bind this value to the name `acc`.

`Feature` is a type that represents features, and they are written as strings enclosed in double quotes (`"`). If you provide a value that is not of type `Feature`, e.g. a number, you will get an error:

```
> let acc: Feature = 123;
[2022-07-28T03:02:39Z ERROR]  let: Type error. Does not typecheck to F
```

If it takes too long to type `Feature`, you can also just type `F` -- this is equivalent to the type `Feature`:

```
> let acc: F = "acc";
```

You can bind a new value to a previously used name. `cands` will tell you that this name was previously bound to a different value:

```
> let x: F = "nom";
> let x: F = "acc";
```

Hashtag (`#`) starts a line comment.

```
> # This is a comment
> let x: F = "nom"; # This is also a comment
```

# Types

Values in `cands` come in 6 types:

*   `Feature`, `F`: Features
*   `usize`: Unsigned integers
*   `[T]`: Vectors whose elements have type `T`
*   `{T}`: Sets whose elements have type `T`
*   `<T1, ..., Tn>`: `n`-tuple whose first element has type `T1`, ..., whose `n`th element has type `Tn`
*   `SO`: Syntactic objects

## Features

We just saw features -- they can be written with double quotes-enclosed strings.

## Unsigned integers

This should be pretty self-explanatory:

```
> let x: usize = 12;
```

Other forms of numbers that include negative signs, decimal points, etc. will not even parse:

```
> let x: usize = -12;
... (a bunch of syntax errors)
```

## Vectors and sets

Vectors are written as a comma-separated list enclosed in square brackets:

```
> let fvec: [F] = [ "nom", "acc", "gen", "dat" ];
```

Similarly, sets are written as a comma-separated list enclosed in braces:

```
> let fset: [F] = { "sg", "du", "pl", "paucal" };
```

Vectors `[T]` and sets `{T}` are homogenous, i.e. their elements must have the same type `T`:

```
> let fvec: [F] = [ "nom", 123 ];
[2022-07-28T03:21:20Z ERROR]  let: Type error. Does not typecheck to [F]
```

Empty vectors and sets are okay:

```
> let fvec: [F] = [];
> let fset: {F} = {};
```

## Tuples

Tuples are written as a comma-separated list enclosed in a pair of less-than and greater-than signs:

```
> let pair: <F, F> = < "nom", "acc" >;
```

Unlike vectors and sets, tuples can be heterogenous, i.e. the elements don't have to have the same type:

```
> let feature_and_integer: <F, usize> = < "me", 12 >;
```

But the order of elements matter:

```
> let feature_and_integer: <F, usize> = < 12, "me" >;
[2022-07-28T03:27:59Z ERROR]  let: Type error. Does not typecheck to <F, usize>
```

## Syntactic objects

If you are not familiar with Collins and Stabler (2016), this part might make more sense after you go through the next section: **Checking Derivations**.

Syntactic objects can take one of the following three forms:

*   A lexical item token,
*   A set of syntactic objects, or
*   A 3-tuple of a syntactic object SO, a feature vector PF and a feature set LF.

    This last case represents a transferred syntactic object; SO is the transferred syntactic object, and PF and LF are its interface representations.

The type of a lexical item token is `<<{F},{F},[F]>, usize>`, but there is a convenient alias: `Lit`.

If you know about sum types, you can think of syntactic objects `SO` as a sum type: `Lit + {SO} + <SO, [F], {F}>`.

Here is an example of an SO that is a lexical item token:

```
let mary: SO = < < {}, { "D" }, [ "Mary" ] >, 1 >;
```

and an example of an SO that is a set:

```
let VP: SO = {
    < {}, { "V", "=D" }, [ "V" ] >,
    < {}, { "D" }, [ "Mary" ] >
};
```

and an example of a transferred SO:

```
let VP: SO = <
    {
        < {}, { "V", "=D" }, [ "V" ] >,
        < {}, { "D" }, [ "Mary" ] >
    },
    [ "Mary" ],
    {}
>;
```

# Checking derivations

Now, let's write and check a derivation! But before that, we need to set up the **lexicon** and the **Universal Grammar**.

First, let's set the lexicon. The lexicon is a set of lexical items. For a derivation to be valid, all lexical item tokens from the derivation must be based on a lexical item from the lexicon.

Let's say we want the lexicon to contain two lexical items: the object pronoun *me* and the verbal root *HELP*. Let's define these two lexical items. First, we define *me*:

```
> let me: Li = < { "me'" }, { "D" }, [ "me" ] >;
```

The type `Li` (shorthand for lexical item) is equivalent to `<{F},{F},[F]>`, i.e. a 3-tuple of a feature set, a feature set and a feature vector. These are respectively the semantic, syntactic and phonological features of the lexical item.

Let's define *HELP*:

```
> let HELP: Li = < { "help'" }, { "V", "=D" }, [ "HELP" ] >;
```

Now, we can put *me* and *HELP* into a set, and set the lexicon to be this set! We do that with a `set` statement:

```
> set lex = { me, HELP };
```

This is saying that we want to set the value of the global variable `lex` to `{ me, HELP }`. There are two global variables you can set in `cands`: `lex`, which represents the lexicon, and `ug`, which represents UG.

Note that the type of the value you provide to a `set` statement must match that of the global variable. For `lex`, this type has to be `{Li}`, or equivalently `{<{F},{F},[F]>}`. For `ug`, this type has to be `<{F},{F},{F}>`, and we will see an example soon.

We are done setting the lexicon. The next step is to set UG (the order doesn't matter, though). In Collins and Stabler (2016), UG is a 6-tuple where Select, Merge and Transfer take up 3 elements. In `cands`, UG is a 3-tuple because we don't include syntactic operations in UG. We only need to specify the set of phonological, syntactic and semantic features. Let's combine the features we used in *me* and *HELP*, and set `ug`:

```
set ug = <
    { "HELP", "me" },
    { "D", "V", "=D" },
    { "help'", "me'" }
>;
```

Now that we have set both the lexicon and UG, it's time to initialize the I-language. We can do this with an `init` statement, which is very simple:

```
init;
```

That's it.

Now, it's time to write a derivation. A derivation `Deriv` is `[Stage]`, i.e. a vector of stages. A stage `Stage` is `<La, Wksp>`, i.e. a 2-tuple of a lexical array and a workspace. A lexical array `La` is `{Lit}`, i.e. a set of lexical item tokens. Finally, a lexical item token `Lit` is `<Li, usize>`, i.e. a 2-tuple of a lexical item and an index. Let's make a lexical array that contains both <*me*, 1> and <*HELP*, 1>:

```
> let me1: Lit = < me, 1 >;
> let HELP1: Lit = < HELP, 1 >;
> let la1: La = { me1, HELP1 };
```

A workspace `Wksp` is `{SO}`, a set of syntactic objects. Let's make an empty workspace:

```
> let w1: Wksp = {};
```

And we can combine `la1` and `w1` into a stage:

```
> let s1: Stage = < la1, w1 >;
```

We can make a very boring derivation that only contains `s1`:

```
> let d1: Deriv = [ s1 ];
```

And check this derivation with a `check` statement:

```
> check d1;
...
[2022-07-28T04:49:11Z INFO ]  Checking the derivation...
[2022-07-28T04:49:11Z INFO ]  Valid derivation.
```

# Syntax

`cands` support four statements:

*   `let`: Bind a name to a typed value.

*   `set`: Set a global variable.

*   `init`: Initialize I-language.

*   `check`: Check a derivation.