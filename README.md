<div align="center">

# avosetta

Rust-native HTML templates with compile-time optimization.

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/callum-hopkins-dev/avosetta/build.yaml?branch=main&event=push&style=for-the-badge)](https://github.com/callum-hopkins-dev/avosetta/actions/workflows/build.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/avosetta?style=for-the-badge)](https://crates.io/crates/avosetta)
[![docs.rs](https://img.shields.io/docsrs/avosetta?style=for-the-badge)](https://docs.rs/avosetta/latest/avosetta)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/avosetta?style=for-the-badge)](https://crates.io/crates/avosetta)
[![GitHub License](https://img.shields.io/github/license/callum-hopkins-dev/avosetta?style=for-the-badge)](https://github.com/callum-hopkins-dev/avosetta/blob/main/LICENSE)

</div>

Avosetta represents HTML as composable Rust values. Consecutive static
fragments are concatenated at compile time, while injected values and control
flow write directly into the same output buffer.

The `asx!` macro is the primary entry point:

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

The complete ASX syntax reference and public API documentation are available on
[docs.rs](https://docs.rs/avosetta).

## License

Avosetta is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome. Please follow the existing style and conventions. For substantial API changes, opening an issue first is usually the easiest way to discuss the design.
