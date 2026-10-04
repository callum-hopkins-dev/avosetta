<div align="center">

# avosetta

Rust-native HTML templates with compile-time optimization.

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/callum-hopkins-dev/avosetta/build.yaml?branch=main&event=push&style=for-the-badge)](https://github.com/callum-hopkins-dev/avosetta/actions/workflows/build.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/avosetta?style=for-the-badge)](https://crates.io/crates/avosetta)
[![docs.rs](https://img.shields.io/docsrs/avosetta?style=for-the-badge)](https://docs.rs/avosetta/latest/avosetta)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/avosetta?style=for-the-badge)](https://crates.io/crates/avosetta)
[![GitHub License](https://img.shields.io/github/license/callum-hopkins-dev/avosetta?style=for-the-badge)](https://github.com/callum-hopkins-dev/avosetta/blob/main/LICENSE)

</div>

Avosetta turns ASX markup into ordinary Rust values that implement `Html`.
Static markup is joined at compile time, while dynamic expressions and control
flow write directly into a single output buffer.

Text and attribute values are escaped by default. Use `Raw` only when the input
is already trusted HTML.

## Quick start

```rust
use avosetta::{Html, asx};

let name = "<world>";
let html = asx! {
    p[class: "greeting&friends"] {
        @("Hello, ")
        @{name}
    }
};

assert_eq!(
    html.to_string(),
    "<p class=\"greeting&amp;friends\">Hello, &lt;world&gt;</p>"
);
```

## Installation

```console
cargo add avosetta
```

See the [`asx!` syntax guide](https://docs.rs/avosetta/latest/avosetta/macro.asx.html)
for the complete language reference.

## License

Avosetta is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome. Please follow the existing style and conventions. For substantial API changes, opening an issue first is usually the easiest way to discuss the design.
