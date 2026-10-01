use std::{fmt::Arguments, marker::PhantomData, rc::Rc, sync::Arc};

use crate::attr::{DynamicSegments, EmptySegments, SegmentKind};

mod sealed {
    pub trait Segments {}

    pub trait StaticText {}
}
#[doc(hidden)]
pub trait StaticText: sealed::StaticText + Copy {
    const LEN: usize;
    #[inline(always)]
    fn as_str(&self) -> &str {
        let ptr = ::core::ptr::from_ref(self).cast::<u8>();

        // SAFETY: StaticText is sealed, and every implementation is a
        // densely packed sequence of exactly Self::LEN initialized bytes.
        let bytes = unsafe { ::core::slice::from_raw_parts(ptr, Self::LEN) };

        // SAFETY: Text can only be created from valid UTF-8, and Concat
        // preserves UTF-8 validity by joining valid byte sequences.
        unsafe { ::core::str::from_utf8_unchecked(bytes) }
    }
}
#[doc(hidden)]
pub trait DynamicWrite: Sized {
    fn write(self, s: &mut String);
}
#[doc(hidden)]
pub trait Segments: sealed::Segments + Sized {
    type Kind: SegmentKind;
    type PrependStatic<S: StaticText>: Segments;
    type PrependDynamic<D: DynamicWrite>: Segments;
    fn prepend_static<S: StaticText>(self, x: S) -> Self::PrependStatic<S>;
    fn prepend_dynamic<D: DynamicWrite>(self, x: D) -> Self::PrependDynamic<D>;
    fn write(self, s: &mut String);
}

/// A value that can render itself as HTML.
///
/// Implementations are consumed when written, allowing templates and captured
/// values to be moved directly into the rendering pipeline.
pub trait Html: Sized {
    #[doc(hidden)]
    type Segments<T: Segments>: Segments;

    #[doc(hidden)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T>;

    #[inline(always)]
    #[doc(hidden)]
    fn is_present(&self) -> bool {
        true
    }

    /// Appends this HTML value to an existing string.
    #[inline(always)]
    fn write(self, s: &mut String) {
        self.segments(End).write(s);
    }

    /// Renders this HTML value into a newly allocated string.
    #[inline(always)]
    fn to_string(self) -> String {
        let mut s = String::new();
        self.write(&mut s);
        s
    }
}

impl Html for () {
    type Segments<T: Segments> = T;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        x
    }

    #[inline(always)]
    fn is_present(&self) -> bool {
        false
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(hidden)]
pub struct End;

impl sealed::Segments for End {}

impl Segments for End {
    type Kind = EmptySegments;

    type PrependStatic<S: StaticText> = Static<S, Self>;
    type PrependDynamic<D: DynamicWrite> = Dynamic<D, Self>;

    #[inline(always)]
    fn prepend_static<S: StaticText>(self, x: S) -> Self::PrependStatic<S> {
        Static(x, self)
    }

    #[inline(always)]
    fn prepend_dynamic<D: DynamicWrite>(self, x: D) -> Self::PrependDynamic<D> {
        Dynamic(x, self)
    }

    #[inline(always)]
    fn write(self, _s: &mut String) {}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(hidden)]
pub struct Static<S, T>(pub S, pub T);

impl<S, T> sealed::Segments for Static<S, T>
where
    S: StaticText,
    T: Segments,
{
}

impl<S, T> Segments for Static<S, T>
where
    S: StaticText,
    T: Segments,
{
    type Kind = <T::Kind as SegmentKind>::PrependStatic;

    type PrependStatic<X: StaticText> = Static<Concat<X, S>, T>;
    type PrependDynamic<D: DynamicWrite> = Dynamic<D, Self>;

    #[inline(always)]
    fn prepend_static<X: StaticText>(self, x: X) -> Self::PrependStatic<X> {
        Static(Concat(x, self.0), self.1)
    }

    #[inline(always)]
    fn prepend_dynamic<D: DynamicWrite>(self, x: D) -> Self::PrependDynamic<D> {
        Dynamic(x, self)
    }

    #[inline(always)]
    fn write(self, s: &mut String) {
        s.push_str(self.0.as_str());
        self.1.write(s);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(hidden)]
pub struct Dynamic<D, T>(pub D, pub T);

impl<D, T> sealed::Segments for Dynamic<D, T>
where
    D: DynamicWrite,
    T: Segments,
{
}

impl<D, T> Segments for Dynamic<D, T>
where
    D: DynamicWrite,
    T: Segments,
{
    type Kind = DynamicSegments;

    type PrependStatic<S: StaticText> = Static<S, Self>;
    type PrependDynamic<X: DynamicWrite> = Dynamic<X, Self>;

    #[inline(always)]
    fn prepend_static<S: StaticText>(self, x: S) -> Self::PrependStatic<S> {
        Static(x, self)
    }

    #[inline(always)]
    fn prepend_dynamic<X: DynamicWrite>(self, x: X) -> Self::PrependDynamic<X> {
        Dynamic(x, self)
    }

    #[inline(always)]
    fn write(self, s: &mut String) {
        self.0.write(s);
        self.1.write(s);
    }
}

/// A fixed-size, UTF-8-validated static text fragment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Text<const N: usize>([u8; N]);

impl<const N: usize> Text<N> {
    /// Creates static text from an unchecked UTF-8 byte array.
    ///
    /// # Safety
    ///
    /// bytes must contain valid UTF-8.
    #[inline(always)]
    pub const unsafe fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }
}

impl<const N: usize> sealed::StaticText for Text<N> {}

impl<const N: usize> StaticText for Text<N> {
    const LEN: usize = N;
}

impl<const N: usize> Html for Text<N> {
    type Segments<T: Segments> = T::PrependStatic<Self>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        x.prepend_static(self)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[doc(hidden)]
pub struct Concat<T0, T1>(pub T0, pub T1);

impl<T0, T1> sealed::StaticText for Concat<T0, T1>
where
    T0: StaticText,
    T1: StaticText,
{
}

impl<T0, T1> StaticText for Concat<T0, T1>
where
    T0: StaticText,
    T1: StaticText,
{
    const LEN: usize = T0::LEN + T1::LEN;
}

/// Two adjacent HTML values rendered in order.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chain<T0, T1>(pub T0, pub T1);

impl<T0, T1> Html for Chain<T0, T1>
where
    T0: Html,
    T1: Html,
{
    type Segments<T: Segments> = T0::Segments<T1::Segments<T>>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        self.0.segments(self.1.segments(x))
    }

    #[inline(always)]
    fn is_present(&self) -> bool {
        self.0.is_present() || self.1.is_present()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(hidden)]
pub struct FnHtml<F, H>(pub F, pub PhantomData<H>);

impl<F, H> Html for FnHtml<F, H>
where
    F: FnOnce() -> H,
    H: Html,
{
    type Segments<T: Segments> = H::Segments<T>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        (self.0)().segments(x)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(hidden)]
pub struct WriteHtml<F>(pub F);

impl<F> Html for WriteHtml<F>
where
    F: FnOnce(&mut String),
{
    type Segments<T: Segments> = T::PrependDynamic<Self>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        x.prepend_dynamic(self)
    }
}

impl<F> DynamicWrite for WriteHtml<F>
where
    F: FnOnce(&mut String),
{
    #[inline(always)]
    fn write(self, s: &mut String) {
        (self.0)(s);
    }
}

#[inline]
fn write_escaped(value: &str, s: &mut String) {
    let mut start = 0;

    for (index, character) in value.char_indices() {
        let escaped = match character {
            '&' => "&amp;",
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot;",
            '\'' => "&#39;",
            _ => continue,
        };

        s.push_str(&value[start..index]);
        s.push_str(escaped);
        start = index + character.len_utf8();
    }

    s.push_str(&value[start..]);
}

struct EscapeWriter<'a>(&'a mut String);

impl ::core::fmt::Write for EscapeWriter<'_> {
    #[inline]
    fn write_str(&mut self, value: &str) -> ::core::fmt::Result {
        write_escaped(value, self.0);
        Ok(())
    }
}

/// Trusted text that is rendered without HTML escaping.
///
/// Only wrap content that is already valid, trusted HTML. Untrusted input can
/// otherwise inject markup into the rendered document.
///
/// # Examples
///
/// ```
/// use avosetta::{Html, Raw};
///
/// assert_eq!(Raw("<strong>trusted</strong>").to_string(), "<strong>trusted</strong>");
/// ```
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Raw<T>(pub T);

impl<T> Html for Raw<T>
where
    T: AsRef<str>,
{
    type Segments<S: Segments> = S::PrependDynamic<Self>;

    #[inline(always)]
    fn segments<S: Segments>(self, x: S) -> Self::Segments<S> {
        x.prepend_dynamic(self)
    }
}

impl<T> DynamicWrite for Raw<T>
where
    T: AsRef<str>,
{
    #[inline(always)]
    fn write(self, s: &mut String) {
        s.push_str(self.0.as_ref());
    }
}

impl Html for bool {
    type Segments<T: Segments> = T::PrependDynamic<Self>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        x.prepend_dynamic(self)
    }

    #[inline(always)]
    fn is_present(&self) -> bool {
        *self
    }
}

impl DynamicWrite for bool {
    #[inline(always)]
    fn write(self, s: &mut String) {
        s.push_str(if self { "true" } else { "false" });
    }
}

macro_rules! impl_dynamic_html {
    ($ty:ty) => {
        impl Html for $ty {
            type Segments<T: Segments> = T::PrependDynamic<Self>;

            #[inline(always)]
            fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
                x.prepend_dynamic(self)
            }
        }
    };
}

impl_dynamic_html!(char);

impl DynamicWrite for char {
    #[inline(always)]
    fn write(self, s: &mut String) {
        let mut buffer = [0; 4];
        write_escaped(self.encode_utf8(&mut buffer), s);
    }
}

macro_rules! impl_integer {
    ($ty:ty) => {
        impl_dynamic_html!($ty);

        impl DynamicWrite for $ty {
            #[inline(always)]
            fn write(self, s: &mut String) {
                s.push_str(::itoa::Buffer::new().format(self));
            }
        }
    };
}

impl_integer!(usize);
impl_integer!(isize);
impl_integer!(u8);
impl_integer!(i8);
impl_integer!(u16);
impl_integer!(i16);
impl_integer!(u32);
impl_integer!(i32);
impl_integer!(u64);
impl_integer!(i64);
impl_integer!(u128);
impl_integer!(i128);

macro_rules! impl_float {
    ($ty:ty) => {
        impl_dynamic_html!($ty);

        impl DynamicWrite for $ty {
            #[inline(always)]
            fn write(self, s: &mut String) {
                s.push_str(::ryu::Buffer::new().format(self));
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);

impl_dynamic_html!(&str);

impl DynamicWrite for &str {
    #[inline(always)]
    fn write(self, s: &mut String) {
        write_escaped(self, s);
    }
}

macro_rules! impl_string {
    ($ty:ty) => {
        impl_dynamic_html!($ty);

        impl DynamicWrite for $ty {
            #[inline(always)]
            fn write(self, s: &mut String) {
                write_escaped(self.as_ref(), s);
            }
        }
    };
}

impl_string!(String);
impl_string!(&String);
impl_string!(&mut String);
impl_string!(Box<str>);
impl_string!(&Box<str>);
impl_string!(&mut Box<str>);
impl_string!(Rc<str>);
impl_string!(&Rc<str>);
impl_string!(&mut Rc<str>);
impl_string!(Arc<str>);
impl_string!(&Arc<str>);
impl_string!(&mut Arc<str>);

impl_dynamic_html!(Arguments<'_>);

impl DynamicWrite for Arguments<'_> {
    #[inline(always)]
    fn write(self, s: &mut String) {
        ::core::fmt::write(&mut EscapeWriter(s), self).unwrap();
    }
}

impl<T> Html for Option<T>
where
    T: Html,
{
    type Segments<S: Segments> = S::PrependDynamic<Self>;

    #[inline(always)]
    fn segments<S: Segments>(self, x: S) -> Self::Segments<S> {
        x.prepend_dynamic(self)
    }

    #[inline(always)]
    fn is_present(&self) -> bool {
        self.is_some()
    }
}

impl<T> DynamicWrite for Option<T>
where
    T: Html,
{
    #[inline(always)]
    fn write(self, s: &mut String) {
        if let Some(x) = self {
            Html::write(x, s);
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __static_text {
    (@raw $expr:expr) => {{
        const __STR: &str = $expr;
        const __TEXT: $crate::Text<{ __STR.len() }> =
            // SAFETY: __STR is valid UTF-8, and the target array length is
            // exactly its byte length.
            unsafe { $crate::Text::new(*__STR.as_bytes().as_array().unwrap_unchecked()) };

        __TEXT
    }};

    ($expr:expr) => {{
        const fn __escaped_len(value: &str) -> usize {
            let bytes = value.as_bytes();
            let mut index = 0;
            let mut len = 0;

            while index < bytes.len() {
                len += match bytes[index] {
                    38 => 5,
                    60 | 62 => 4,
                    34 => 6,
                    39 => 5,
                    _ => 1,
                };
                index += 1;
            }

            len
        }

        const fn __escape<const N: usize>(value: &str) -> [u8; N] {
            let input = value.as_bytes();
            let mut output = [0; N];
            let mut input_index = 0;
            let mut output_index = 0;

            while input_index < input.len() {
                let escaped: &[u8] = match input[input_index] {
                    38 => b"&amp;",
                    60 => b"&lt;",
                    62 => b"&gt;",
                    34 => b"&quot;",
                    39 => b"&#39;",
                    byte => {
                        output[output_index] = byte;
                        input_index += 1;
                        output_index += 1;
                        continue;
                    }
                };

                let mut escaped_index = 0;
                while escaped_index < escaped.len() {
                    output[output_index] = escaped[escaped_index];
                    output_index += 1;
                    escaped_index += 1;
                }

                input_index += 1;
            }

            output
        }

        const __STR: &str = $expr;
        const __LEN: usize = __escaped_len(__STR);
        const __BYTES: [u8; __LEN] = __escape::<__LEN>(__STR);
        const __TEXT: $crate::Text<__LEN> =
            // SAFETY: __escape copies UTF-8 bytes unchanged and substitutes
            // ASCII characters with valid UTF-8 entity references.
            unsafe { $crate::Text::new(__BYTES) };

        __TEXT
    }};
}
