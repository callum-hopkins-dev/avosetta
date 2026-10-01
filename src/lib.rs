//! Rust-native HTML templates with compile-time optimization.
//!
//! Avosetta represents HTML as composable Rust values. Consecutive static
//! fragments are concatenated at compile time, while injected values and
//! control flow write directly into the same output buffer.
//!
//! The asx! macro is the primary entry point:
//!
//! ```
//! use avosetta::{Html, asx};
//!
//! let name = "<world>";
//! let html = asx! {
//!     p[class: "greeting&friends"] {
//!         @("Hello, ")
//!         @{name}
//!     }
//! };
//!
//! assert_eq!(
//!     html.to_string(),
//!     "<p class=\"greeting&amp;friends\">Hello, &lt;world&gt;</p>"
//! );
//! ```

#![warn(missing_docs)]

mod attr;
mod fragment;
mod html;
mod syntax;

#[doc(hidden)]
pub mod elements;

pub use attr::{Attr, Attributes};
pub use fragment::{Attrs, Children, Props};
pub use html::{Chain, Html, Raw, Text};

#[doc(hidden)]
pub use attr::{AttributeSet, Class, ClassChain, Style, StyleValue};

#[doc(hidden)]
pub use fragment::Fragment;

#[doc(hidden)]
pub use html::{FnHtml, WriteHtml};

#[cfg(test)]
mod tests;
