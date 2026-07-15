<div align="center">

# avosetta

Rust-native HTML templates with compile-time optimization.

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/callum-hopkins-dev/avosetta/build.yaml?branch=main&event=push&style=for-the-badge)](https://github.com/callum-hopkins-dev/avosetta/actions/workflows/build.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/avosetta?style=for-the-badge)](https://crates.io/crates/avosetta)
[![docs.rs](https://img.shields.io/docsrs/avosetta?style=for-the-badge)](https://docs.rs/avosetta/latest/avosetta)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/avosetta?style=for-the-badge)](https://crates.io/crates/avosetta)
[![GitHub License](https://img.shields.io/github/license/callum-hopkins-dev/avosetta?style=for-the-badge)](https://github.com/callum-hopkins-dev/avosetta/blob/main/LICENSE)

</div>

## about

`avosetta` is a Rust-native HTML templating library built around the `asx!`
proc macro. Templates use a compact, Rust-like syntax and expand to values
implementing `Html`, which render directly into a `String`.

The generated code is designed to leave very little work for runtime: adjacent
static output is combined into larger string writes, and static string literals
are HTML-escaped during compilation. Dynamic values are rendered through the
`Html` trait and escaped where appropriate.

* Rust-like elements, attributes, interpolation, and control flow
* Compile-time escaping of static string literals
* Coalesced static output for fewer runtime string operations
* Runtime escaping for dynamic strings and characters
* Conditional attributes through `bool` and `Option`
* Direct composition through the `Html` trait
* No intermediate template tree or virtual DOM

## installation

Add `avosetta` with macro support:

```console
cargo add avosetta --features macros
```

## quick start

```rust
use avosetta::{asx, Html};

let name = "<Ada>";
let page = asx! {
    main[class="profile"] {
        h1 { "Hello, " @name }
        input[disabled=true];
    }
};

let mut html = String::new();
page.write(&mut html);

assert_eq!(
    html,
    r#"<main class="profile"><h1>Hello, &lt;Ada&gt;</h1><input disabled="disabled"></main>"#,
);
```

`asx!` returns an opaque value implementing `Html`. Call `Html::write` to append the rendered template to a string buffer.

## syntax at a glance

Elements use braces for children, while void elements end with a semicolon.
Prefix Rust expressions and control flow with `@`:

```rust
use avosetta::{asx, Html};

let title = "Messages";
let messages = ["Hello", "<Welcome>"];

let page = asx! {
    section[class="messages"] {
        h1 { @title }

        @if messages.is_empty() {
            p { "No messages" }
        } else {
            ul {
                @for message in messages {
                    li { @message }
                }
            }
        }
    }
};

let mut html = String::new();
page.write(&mut html);
```

Attribute values are Rust expressions. String literals can be written directly
as template text, and names that are not Rust identifiers can be quoted:

```rust
"x-user-card"["aria-label"=label] {
    span { "Profile" }
}
```

See the crate-level API documentation for the complete syntax reference,
including `match`, local Rust statements, quoted names, and attribute behavior.

## escaping

Escaping is the default:

* Static string literals are escaped at compile time.
* Dynamic strings and characters are escaped when rendered.
* `false` and `None` omit an attribute.
* `true` emits a boolean attribute as `name="name"`.

Use `Raw` only for trusted, already-rendered markup:

```rust
use avosetta::{asx, Html, Raw};

let trusted = "<strong>Already rendered</strong>";
let template = asx! {
    div { @Raw(trusted) }
};

let mut html = String::new();
template.write(&mut html);
```

`Raw` bypasses HTML escaping. Never use it with untrusted or user-controlled input.

## performance

`avosetta` generates string-writing code rather than constructing an
intermediate representation at runtime. Static runs are combined, static text
is escaped during compilation, and dynamic values write directly into the
destination buffer through `Html`.

This keeps the runtime path close to the final operation the application needs:
appending HTML to a `String`.

## license

`avosetta` is licensed under the MIT License. See `LICENSE` for details.

## contributing

Contributions are welcome.

Please follow the existing code style and conventions used throughout the
project. If you're proposing a new feature or API, opening an issue first is
often the easiest way to discuss the design.
