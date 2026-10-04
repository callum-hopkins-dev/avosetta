//! Rust-native HTML templates with compile-time optimization.
//!
//! Avosetta turns ASX markup into ordinary Rust values that implement
//! [`Html`]. Static markup is joined at compile time, while dynamic expressions
//! and control flow write directly into a single output buffer.
//!
//! Text and attribute values are escaped by default. Use [`Raw`] only when the
//! input is already trusted HTML.
//!
//! # Quick start
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
//!
//! See [`asx!`] for the complete syntax guide.

mod attr;
mod fragment;
mod html;
mod syntax;

#[doc(hidden)]
pub mod elements;

pub use attr::{Attr, Attributes};
pub use fragment::{Attrs, Children, Context, Fragment, Omitted, Props};
pub use html::{Chain, Html, Raw, Text};

#[doc(hidden)]
pub use attr::{AttributeSet, Class, ClassChain, Style, StyleValue};

#[doc(hidden)]
pub use fragment::FnFragment;

#[doc(hidden)]
pub use html::{FnHtml, WriteHtml};

#[cfg(test)]
mod tests;
