<div align="center">

# avosetta

A fast, minimal html templating language for Rust.

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/callum-hopkins-dev/avosetta/build.yaml?branch=main&event=push&style=for-the-badge)](https://github.com/callum-hopkins-dev/avosetta/actions/workflows/build.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/avosetta?style=for-the-badge)](https://crates.io/crates/avosetta)
[![docs.rs](https://img.shields.io/docsrs/avosetta?style=for-the-badge)](https://docs.rs/avosetta/latest/avosetta)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/avosetta?style=for-the-badge)](https://crates.io/crates/avosetta)
[![GitHub License](https://img.shields.io/github/license/callum-hopkins-dev/avosetta?style=for-the-badge)](https://github.com/callum-hopkins-dev/avosetta/blob/main/LICENSE)

</div>

## about

`avosetta` is a minimal templating library for that utilises procedural
macros to generate as close to optimal code as possible for rendering html
content at runtime. It has no `unsafe` code, only a handful of dependencies, and
does not allocate any values on the heap.

We implement a terse, simple syntax for specifying templates that is
straightforward to parse, has little ambiguity and integrates into `Rust`
code better. And unlike other templating libraries such as `maud`, our syntax
typically only has a single way of writing various constructs, reducing
code-style clashing. For more information, read the full syntax reference
[here](https://docs.rs/avosetta/latest/avosetta#reference).

Optimisations include automatically escaping static string literals at
compile-time and collapsing contiguous `String::push_str` calls into a single one.
Therefore, if your html fragment is entirely static, the generated code will
just be a single `String::push_str` with a `&str`.

## getting started

To start using `avosetta`, you'll first need to add our package to your
`Cargo.toml` manifest:

```toml
[dependencies]
avosetta = "0.1.0"
```

Then you can start writing html templates directly in your `Rust` source
code. We recommend that you import the `prelude` module to reduce unnecessary
qualifications, but that's up to you.

```rust
use avosetta::prelude::*;

fn main() {
  let mut s = String::new();
  index().write(&mut s);

  println!("{s}");
}

fn index() -> impl Html {
  html! {
    @layout(
      html! {
        title { "avosetta" }
      },

      html! {
        h1 { "Hello, World!" }
      },
    )
  }
}

fn layout(
  head: impl Html,
  body: impl Html,
) -> impl Html {
  html! {
    "!DOCTYPE"[html];
    html[lang="en"] {
      head {
        meta[charset="UTF-8"];
        meta[name="viewport", content="width=device-width,initial-scale=1"];

        @head
      }

      body {
        main {
          @body
        }
      }
    }
  }
}
```
