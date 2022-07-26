# `cands`

Implementation of Collins and Stabler 2016: "A Formalization of Minimalist Syntax", Syntax 19:1, pp. 43--78.

# For users

1.  This program is written in [Rust](https://www.rust-lang.org/). To build and run `cands`, you first need to install `rustup`, the Rust toolchain installer. Follow the steps [here](https://rustup.rs/).

2.  By default, `rustup` will install the default toolchain. To build and run `cands`, you need to install the nightly toolchain. To do this, run:

    ```
    rustup default nightly
    ```

3.  You are now ready to build and run `cands`. Once you are in this directory (i.e. the one that contains this README file), you can build `cands` with:

    ```
    cargo build
    ```

    Or, build and run `cands` with:

    ```
    cargo run
    ```

# Developer docs

The documentation for `cands` contains KaTeX. For more information on how to build documentation with KaTeX, see [here](https://github.com/paulkernfeld/rustdoc-katex-demo). The following command should work:

Powershell:

```
$env:RUSTDOCFLAGS = "--html-in-header katex-header.html"; cargo doc --no-deps --open
```

Shell:

```
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps --open
```

# Todo

- [x] Set up CI and test coverage (tarpaulin)
- [ ] c-command is not constant time given the current implementation of syntactic objects. To fix this:
    - [ ] Define SO as a trait instead and have multiple implementations
    - [ ] Impl 1: binary set formation -- current impl
    - [ ] Impl 2: immutable mutually recursive data structure, i.e. tree with parent pointers