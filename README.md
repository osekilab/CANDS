# `cands`

Implementation of Collins and Stabler 2016: "A Formalization of Minimalist Syntax", Syntax 19:1, pp. 43--78.

# Docs

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