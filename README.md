# avosetta

A fast, minimal html templating language for Rust.

## about

`avosetta` is a minimal templating library for that utilises procedural macros
to generate as close to optimal code as possible for rendering `HTML` content
at runtime. It has no `unsafe` code, only a handful of dependencies, and does
allocate any values on the heap.

We implement a terse, simple syntax for specifying templates that is
straightforward to parse, has little ambiguity and integrates into `Rust`
code better. And unlike other templating libraries such as `maud`, our syntax
typically only has a single way of writing various constructs, reducing
code-style clashing. For more information, read the full syntax reference
[here](#reference).

Optimisations include automatically escaping static string literals at
compile-time and collapsing contiguous `push_str` calls into a single one.
Therefore, if your html fragment is entirely static, the generated code will
just be a single `push_str` with a `&'static str`.

## getting started

To start using `avosetta`, you'll first need to add our package to your
`Cargo.toml` manifest:

```toml
[dependencies]
avosetta = "0.1.0"
```

Then you can start writing `HTML` templates directly in your `Rust` source
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

## reference

The syntax that this macro accepts is unique to `avosetta`, however, it shares
some major similarities with crates such as `maud` and `markup`. All of these
crates implement a terse, pug-like syntax which integrates into rust code better
and is less error-prone.

Unlike the other crates, however, `avosetta` has a more minimal syntax.

### elements

There are two types of elements that can be defined: `normal` and `void`.
`void` elements cannot have a body and must be terminated with a `semicolon`.

```rust
html! {
  // A `normal` element that will be rendered as: `<article></article>`
  article {}

  // A `void` element that will be rendered as: `<br>`
  br;

  // Element names can also be string literals:
  "x-custom-element" {}

  "x-custom-element";
}
```

### attributes

Elements can also have attributes, which is a `comma` delimited list of
`key=value` pairs. Note, that attribute values can be dynamic (interpolated at
runtime), where as attribute names must be known at compile time.

```rust
html! {
  // A `meta` element with an attribute:
  meta[charset="UTF-8"];

  // Elements can have multiple attributes:
  meta[name="viewport", content="width=device-width,initial-scale=1"];

  // Attribute names can also be string literals:
  div["x-data"="Hello, World!"] {}

  // Attribute values can be any `Rust` expression:
  input[value={4 + 3}];

  // Attributes without a value are implicitly a `true` boolean attribute:
  input["type"="checkbox", checked];
  input["type"="checkbox", checked=true]; // These two elements are equivalent.
}
```

### interpolation

The process of "injecting" or writing dynamic content into the `HTML` is
called `interpolation`. This might be used for displaying a local variable
containing a username, or for performing a conditional `if` check before
rendering some sub-content.

All interpolations start with an `@`, however, depending on the context,
different interpolations will be generated in the `impl Html`.

```rust
let x = 9;

html! {
  // The most basic interpolation of a simple expression:
  @x

  // More complicated expressions can also be interpolated:
  @x + 2

  // Depending on what you're interpolating, it may remove ambiguity to use
  // a block expression:
  @{ x + 2 }

  // `Html` is implemented for `()`, therefore, expressions don't need to
  // return any `HTML` content:
  @{
    // This will be executed when the `Html` is written to a `String`.
    println!("Hello, World!");
  }

  // You can conditionally render content using the normal `Rust` syntax:
  @if x > 8 {
    // Notice how these arms take `html!` syntax and not `Rust` syntax.
    h1 { "Hello, World!" }
  } else if x < 2 {
    h2 { "Hello, World!" }
  } else {
    h3 { "Hello, World!" }
  }

  // The same concept applies to both the `match` and `for` keywords:
  @match x {
    // Each arm must be wrapped in braces.
    8 => {
      h1 { "Hello, World!" }
    }

    // Except for simple string literals.
    _ => "Hello, World!"
  }

  @for i in 0..24 {
    // Nested interpolation works as you'd expect.
    span { @i }
  }
}
```

### string literals

Whilst most content can be interpolated into the `HTML` at runtime, there
is a specific optimisation made for static string literals. When used without an
`@`, the string literals are automatically escaped at compile-time and rendered
using `avosetta::raw`.

```rust
html! {
  // Both of these elements will render to the same `HTML`, however,
  // the first one will escape the string at runtime because it is an
  // interpolated expression.
  h1 { @"Hello, World!" }

  // This one will be pre-escaped at compile time, avoiding the performance
  // cost of runtime escaping.
  h1 { "Hello, World!" }
}
```
