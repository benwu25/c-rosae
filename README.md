This is the source repository for `c-rosae`, a fork of the Rust compiler. `c-rosae` provides verification tooling for Rust, implementing a Rust frontend for the [Daikon dynamic invariant detector](https://github.com/codespecs/daikon) to hypothesize likely program invariants through dynamic analysis.

 * Build the compiler.
     * `./x setup`
     * `./x build library/std`
 * Set up daikon as a toolchain with the stage 1 build.
     * `rustup toolchain link daikon build/<platform>/stage1`
 * Produce dtrace and decls files in one command via `cargo +daikon run`,
   or instrument with rustc via `rustc +daikon foo.rs`, and run to produce trace data.

----------------------------------------------------------------------------------------------------------------------------------------------------

# Implementation overview

The implementation consists of two parts: dtrace instrumentation and decls generation.

Dtrace instrumentation requires mutation of the AST, and this is done at the same time as parsing and macro expansion.

Decls generation does not require mutation of the AST, so it is implemented after parsing and macro expansion, when the entire AST is formed and immutable.

### Dtrace instrumentation

Dtrace instrumentation is implemented in `compiler/rustc_parse/src/parser/item.rs`. Code for the instrumentation starts at the beginning of this file and continues through the function `parse_mod` (line 1923). The rest of the file is part of `rustc`.

Dtrace instrumentation is the first pass to run. During compilation, `rustc` parses source code one file at a time. Files are parsed by a call to `parse_mod` in item.rs. One pass of dtrace instrumentation is executed for every call to `parse_mod`. Each pass is managed by a `DaikonDtraceVisitor` struct, defined at `item.rs:73`. This struct applies a mutable visitor pass on the file-scope AST fragment, walking functions to add logging of parameters and return values.

The mutable visitor also visits struct definitions within the file, and bundles together a new `impl` block containing struct-specific dtrace routines. E.g.,
```Rust
impl X {
    pub fn dtrace_print_fields(&self, depth: i32, prefix: String) {
        ...
    }
}

fn foo(a: X) {
    a.dtrace_print_fields(DEPTH, String::from("a");
    ...
}
```
Multiple routines and helper functions are defined in these `impl` blocks to expose field-specific routines and to handle `Vec` cases.

### Decls generation

Once all files have been parsed and instrumented, a single decls generation pass is initiated to run over the complete and immutable AST. Code for decls generation starts in `compiler/rustc_expand/src/expand.rs:474` and continues through the function `expand_crate` (line 1933).

The main entry point for decls generation is in `expand_crate`. After calling `fully_expand_fragment`, the AST is complete and immutable, and decls generation can run as soon as the crate has been expanded.

Decls generation uses identical techniques to dtrace instrumentation to walk functions. The struct `DaikonDeclsVisitor` is responsible for applying an immutable visitor on the AST to walk functions and write out the entire decls file.

Three core structs are used to represent the decls file in memory.

The `TopLevlDecl` represents a record for a function parameter or return value. It represents the root of a tree of sub declarations. If the `TopLevlDecl` is a struct, then it also contains a `Vec<FieldDecl>`. The `FieldDecl` struct represents a field declaration record, which may also contain a `Vec<FieldDecl>` for nested structs.

If a `TopLevlDecl` is an array or `Vec`, then it contains an `ArrayContents` struct rather than a `Vec<FieldDecl>`. For primitive arrays and `Vec`s, the `ArrayContents` only stores the name (e.g., `arr[..]`) and Java type of the outer array. For `Vec`s and arrays of structs, the `ArrayContents` contains a `Vec<ArrayContents>`, with one for each field of the struct.

A depth counter is used to track depth of recursion when building these trees.

### daikon_strs.rs

`daikon_strs.rs` is a large file full of helper routines for building parameterized code snippets which will be inserted into the final executable. Each function is paired with a `String` or `String` array. The functions take arguments which represent identifiers for variables or types, and they essentially smash together the `String` array with identifiers in between.

<br></br>

# Next Steps

### 1. Logging the correct structs

`daikon-rustc` should only instrument structs which belong to the crate which it is called to compile.
```Rust
fn foo(x: Option<i32>, y: &HashMap<String, i32>) {
    ...
}
```
In this example, both parameter types are defined in the Rust standard library, so `daikon-rustc` should not generate *any* instrumentation for this function. Thus, the dtrace instrumentation pass must know that `Option` and `HashMap` should be left alone.

Unfortunately, dtrace instrumentation only has access to file-local struct definitions.

The simplest way to determine which structs belong to the crate is to run a visitor pass over the entire immutable AST. But, by the time the AST is immutable, dtrace instrumentation cannot run.

So, we can implement two executions of `daikon-rustc`, where in the first, one visitor walks the entire AST to log all structs which should be instrumented in a `/tmp` file.

Both dtrace instrumentation and decls generation passes will then have access to which structs belong to the crate in the next run.

### 2. Generic functions

Dynamic constraints do not make sense for generic functions, so we need to skip over generic functions. These are currently broken, and commonly seen in real Rust code.

### 3. Real-world programs for testing

I should start testing with real programs from the corpus.

### 4. Nested structs

These are broken. I think they are less common in real code, but still out there.

### 5. Fuzz testing

Once I write enough small tests which cover a wide range of features in the Rust programming language, it could be useful to do some fuzz testing to automate finding code paths which `panic` in my implementation, and also to learn about fuzzing as it relates to language tools.

### 6. Misc

Accept depth argument on the command line.

Other command line arguments?

Fix broken nonce counter: use a single protected counter across all threads and files (currently one per file).

Fix reused variable labels in dtrace instrumentation.

<br></br>

(Below is the original `rustc` README.md)

-----------------------------------------------------------------------------------------------------------------------------------------------------------

<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg">
    <img alt="The Rust Programming Language: A language empowering everyone to build reliable and efficient software"
         src="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg"
         width="50%">
  </picture>

[Website][Rust] | [Getting started] | [Learn] | [Documentation] | [Contributing]
</div>

This is the main source code repository for [Rust]. It contains the compiler,
standard library, and documentation.

[Rust]: https://www.rust-lang.org/
[Getting Started]: https://www.rust-lang.org/learn/get-started
[Learn]: https://www.rust-lang.org/learn
[Documentation]: https://www.rust-lang.org/learn#learn-use
[Contributing]: CONTRIBUTING.md

## Why Rust?

- **Performance:** Fast and memory-efficient, suitable for critical services, embedded devices, and easily integrated with other languages.

- **Reliability:** Our rich type system and ownership model ensure memory and thread safety, reducing bugs at compile-time.

- **Productivity:** Comprehensive documentation, a compiler committed to providing great diagnostics, and advanced tooling including package manager and build tool ([Cargo]), auto-formatter ([rustfmt]), linter ([Clippy]) and editor support ([rust-analyzer]).

[Cargo]: https://github.com/rust-lang/cargo
[rustfmt]: https://github.com/rust-lang/rustfmt
[Clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer

## Quick Start

Read ["Installation"] from [The Book].

["Installation"]: https://doc.rust-lang.org/book/ch01-01-installation.html
[The Book]: https://doc.rust-lang.org/book/index.html

## Installing from Source

If you really want to install from source (though this is not recommended), see
[INSTALL.md](INSTALL.md).

## Getting Help

See https://www.rust-lang.org/community for a list of chat platforms and forums.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Rust is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

## Trademark

[The Rust Foundation][rust-foundation] owns and protects the Rust and Cargo
trademarks and logos (the "Rust Trademarks").

If you want to use these names or brands, please read the
[Rust language trademark policy][trademark-policy].

Third-party logos may be subject to third-party copyrights and trademarks. See
[Licenses][policies-licenses] for details.

[rust-foundation]: https://rustfoundation.org/
[trademark-policy]: https://rustfoundation.org/policy/rust-trademark-policy/
[policies-licenses]: https://www.rust-lang.org/policies/licenses
