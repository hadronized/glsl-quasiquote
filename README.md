# GLSL quasiquoting

This crate exports two procedural macros: `glsl!` and `glsl_str!`. They enable quasiquoting by
allowing you to embed GLSL source code directly into rust via the syntax:

```ignore
glsl!{
  // your GLSL code here
  void main() {
  }
}
```

The `glsl!` macro accepts the GLSL code directly. You should be using that macro in pretty much
all situations, but there’s an edge corner that might require you to use its `glsl_str!`
sibling: if you wan to use the `#version` or `#extension` GLSL pragmas. Rust procedural macro
system will parse those pragmas as regular Rust token and will ignore the mandatory `\n`,
causing the macro to fail. In that case, you need to use an opaque string to encode the newlines
by doing so:

```ignore
glsl_str!{"
  #version 330 core
  // your GLSL code here
"}
```

Both the `glsl!` and `glsl_str!` procedural macro resolve at compile-time to
`glsl::syntax::TranslationUnit`, allowing you to manipulate the GLSL AST directly. Feel free
to have a look at the [`glsl`](https://crates.io/crates/glsl) crate for further information.

# Getting started

Add the following to your dependencies in your `Cargo.toml`:

```
glsl = "0.9"
glsl-quasiquote = "0.1"
```

Then, you currently need to have a nightly compiler and the following feature enabled:

```
#![feature(proc_macro_non_items)]
```

Then, depending on which you’re using the 2018 edition or not:

> *Non-2018 edition*

```
extern crate glsl;
#[macro_use] extern crate glsl_quasiquote;
```

> *2018 edition*

```
use glsl_quasiquote::{glsl, glsl_str};
```
