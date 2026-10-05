use std::{fmt::Arguments, marker::PhantomData, rc::Rc, sync::Arc};

use crate::attr::{EmptySegments, PresentSegments, SegmentKind};

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

        // SAFETY: Every StaticText implementation contains valid UTF-8 or
        // joins valid UTF-8 byte sequences without inserting other bytes.
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
    type PrependPresent<D: DynamicWrite>: Segments;
    fn prepend_static<S: StaticText>(self, x: S) -> Self::PrependStatic<S>;
    fn prepend_dynamic<D: DynamicWrite>(self, x: D) -> Self::PrependDynamic<D>;
    fn prepend_present<D: DynamicWrite>(self, x: D) -> Self::PrependPresent<D>;
    fn write(self, s: &mut String);
}

/// A value that can be rendered as HTML.
///
/// Rendering consumes the value, allowing captured data to move directly into
/// the output pipeline without an intermediate tree.
pub trait Html: Sized {
    #[doc(hidden)]
    type Segments<T: Segments>: Segments;

    #[doc(hidden)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T>;

    #[inline(always)]
    #[doc(hidden)]
    fn is_attribute_present(&self) -> bool {
        true
    }

    /// Renders this value at the end of an existing string.
    #[inline(always)]
    fn write(self, s: &mut String) {
        self.segments(End).write(s);
    }

    /// Renders this value into a new string.
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
    fn is_attribute_present(&self) -> bool {
        false
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct End;

impl sealed::Segments for End {}

impl Segments for End {
    type Kind = EmptySegments;

    type PrependStatic<S: StaticText> = Static<RunOne<S, RunEnd>, Self>;
    type PrependDynamic<D: DynamicWrite> = Dynamic<D, Self>;
    type PrependPresent<D: DynamicWrite> = Present<D, Self>;

    #[inline(always)]
    fn prepend_static<S: StaticText>(self, x: S) -> Self::PrependStatic<S> {
        Static(RunOne(x, RunEnd), self)
    }

    #[inline(always)]
    fn prepend_dynamic<D: DynamicWrite>(self, x: D) -> Self::PrependDynamic<D> {
        Dynamic(x, self)
    }

    #[inline(always)]
    fn prepend_present<D: DynamicWrite>(self, x: D) -> Self::PrependPresent<D> {
        Present(x, self)
    }

    #[inline(always)]
    fn write(self, _s: &mut String) {}
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Static<S, T>(pub S, pub T);

impl<S, T> sealed::Segments for Static<S, T>
where
    S: StaticRun,
    T: Segments,
{
}

impl<S, T> Segments for Static<S, T>
where
    S: StaticRun,
    T: Segments,
{
    type Kind = <T::Kind as SegmentKind>::PrependStatic;

    type PrependStatic<X: StaticText> = Static<S::Prepend<X>, T>;
    type PrependDynamic<D: DynamicWrite> = Dynamic<D, Self>;
    type PrependPresent<D: DynamicWrite> = Present<D, Self>;

    #[inline(always)]
    fn prepend_static<X: StaticText>(self, x: X) -> Self::PrependStatic<X> {
        Static(self.0.prepend(x), self.1)
    }

    #[inline(always)]
    fn prepend_dynamic<D: DynamicWrite>(self, x: D) -> Self::PrependDynamic<D> {
        Dynamic(x, self)
    }

    #[inline(always)]
    fn prepend_present<D: DynamicWrite>(self, x: D) -> Self::PrependPresent<D> {
        Present(x, self)
    }

    #[inline(always)]
    fn write(self, s: &mut String) {
        s.push_str(self.0.as_str());
        self.1.write(s);
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    type Kind = <T::Kind as SegmentKind>::PrependDynamic;

    type PrependStatic<S: StaticText> = Static<RunOne<S, RunEnd>, Self>;
    type PrependDynamic<X: DynamicWrite> = Dynamic<X, Self>;
    type PrependPresent<X: DynamicWrite> = Present<X, Self>;

    #[inline(always)]
    fn prepend_static<S: StaticText>(self, x: S) -> Self::PrependStatic<S> {
        Static(RunOne(x, RunEnd), self)
    }

    #[inline(always)]
    fn prepend_dynamic<X: DynamicWrite>(self, x: X) -> Self::PrependDynamic<X> {
        Dynamic(x, self)
    }

    #[inline(always)]
    fn prepend_present<X: DynamicWrite>(self, x: X) -> Self::PrependPresent<X> {
        Present(x, self)
    }

    #[inline(always)]
    fn write(self, s: &mut String) {
        self.0.write(s);
        self.1.write(s);
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Present<D, T>(pub D, pub T);

impl<D, T> sealed::Segments for Present<D, T>
where
    D: DynamicWrite,
    T: Segments,
{
}

impl<D, T> Segments for Present<D, T>
where
    D: DynamicWrite,
    T: Segments,
{
    type Kind = PresentSegments;

    type PrependStatic<S: StaticText> = Static<RunOne<S, RunEnd>, Self>;
    type PrependDynamic<X: DynamicWrite> = Dynamic<X, Self>;
    type PrependPresent<X: DynamicWrite> = Present<X, Self>;

    #[inline(always)]
    fn prepend_static<S: StaticText>(self, x: S) -> Self::PrependStatic<S> {
        Static(RunOne(x, RunEnd), self)
    }

    #[inline(always)]
    fn prepend_dynamic<X: DynamicWrite>(self, x: X) -> Self::PrependDynamic<X> {
        Dynamic(x, self)
    }

    #[inline(always)]
    fn prepend_present<X: DynamicWrite>(self, x: X) -> Self::PrependPresent<X> {
        Present(x, self)
    }

    #[inline(always)]
    fn write(self, s: &mut String) {
        self.0.write(s);
        self.1.write(s);
    }
}

/// A fixed-size fragment of valid UTF-8 known at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Text<const N: usize>([u8; N]);

impl<const N: usize> Text<N> {
    /// Creates static text without validating its bytes.
    ///
    /// # Safety
    ///
    /// `bytes` must contain valid UTF-8.
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

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
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

#[doc(hidden)]
pub trait StaticRun: StaticText {
    type Prepend<S: StaticText>: StaticRun;

    fn prepend<S: StaticText>(self, x: S) -> Self::Prepend<S>;
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct RunEnd;

impl sealed::StaticText for RunEnd {}

impl StaticText for RunEnd {
    const LEN: usize = 0;
}

impl StaticRun for RunEnd {
    type Prepend<S: StaticText> = RunOne<S, Self>;

    #[inline(always)]
    fn prepend<S: StaticText>(self, x: S) -> Self::Prepend<S> {
        RunOne(x, self)
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct RunZero<R>(pub R);

impl<R> sealed::StaticText for RunZero<R> where R: StaticRun {}

impl<R> StaticText for RunZero<R>
where
    R: StaticRun,
{
    const LEN: usize = R::LEN;
}

impl<R> StaticRun for RunZero<R>
where
    R: StaticRun,
{
    type Prepend<S: StaticText> = RunOne<S, R>;

    #[inline(always)]
    fn prepend<S: StaticText>(self, x: S) -> Self::Prepend<S> {
        RunOne(x, self.0)
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct RunOne<B, R>(pub B, pub R);

impl<B, R> sealed::StaticText for RunOne<B, R>
where
    B: StaticText,
    R: StaticRun,
{
}

impl<B, R> StaticText for RunOne<B, R>
where
    B: StaticText,
    R: StaticRun,
{
    const LEN: usize = B::LEN + R::LEN;
}

impl<B, R> StaticRun for RunOne<B, R>
where
    B: StaticText,
    R: StaticRun,
{
    type Prepend<S: StaticText> = RunZero<R::Prepend<Concat<S, B>>>;

    #[inline(always)]
    fn prepend<S: StaticText>(self, x: S) -> Self::Prepend<S> {
        RunZero(self.1.prepend(Concat(x, self.0)))
    }
}

/// Two HTML values rendered consecutively.
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
    fn is_attribute_present(&self) -> bool {
        self.0.is_attribute_present() || self.1.is_attribute_present()
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteHtml<F>(pub F);

impl<F> Html for WriteHtml<F>
where
    F: FnOnce(&mut String),
{
    type Segments<T: Segments> = T::PrependPresent<Self>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        x.prepend_present(self)
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

/// A trusted string rendered without HTML escaping.
///
/// Use this wrapper only for content that is already valid, trusted HTML.
/// Wrapping untrusted input can introduce markup into the document.
///
/// # Example
///
/// ```
/// use avosetta::{Html, Raw};
///
/// assert_eq!(Raw("<strong>trusted</strong>").to_string(), "<strong>trusted</strong>");
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Raw<T>(pub T);

impl<T> Html for Raw<T>
where
    T: AsRef<str>,
{
    type Segments<S: Segments> = S::PrependPresent<Self>;

    #[inline(always)]
    fn segments<S: Segments>(self, x: S) -> Self::Segments<S> {
        x.prepend_present(self)
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
    fn is_attribute_present(&self) -> bool {
        *self
    }
}

impl DynamicWrite for bool {
    #[inline(always)]
    fn write(self, s: &mut String) {
        s.push_str(if self { "true" } else { "false" });
    }
}

macro_rules! impl_present_html {
    ($ty:ty) => {
        impl Html for $ty {
            type Segments<T: Segments> = T::PrependPresent<Self>;

            #[inline(always)]
            fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
                x.prepend_present(self)
            }
        }
    };
}

impl_present_html!(char);

impl DynamicWrite for char {
    #[inline(always)]
    fn write(self, s: &mut String) {
        let mut buffer = [0; 4];
        write_escaped(self.encode_utf8(&mut buffer), s);
    }
}

macro_rules! impl_integer {
    ($ty:ty) => {
        impl_present_html!($ty);

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
        impl_present_html!($ty);

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

impl_present_html!(&str);

impl DynamicWrite for &str {
    #[inline(always)]
    fn write(self, s: &mut String) {
        write_escaped(self, s);
    }
}

macro_rules! impl_string {
    ($ty:ty) => {
        impl_present_html!($ty);

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

impl_present_html!(Arguments<'_>);

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
    fn is_attribute_present(&self) -> bool {
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

/// Concatenates HTML values without allocating an intermediate string.
///
/// Literal arguments become escaped static text, allowing adjacent static
/// segments to collapse at compile time. Other arguments retain their own
/// [`Html`] implementations, including direct integer and float formatting.
///
/// # Example
///
/// ```
/// use avosetta::Html;
///
/// let width = 320;
/// let style = avosetta::concat!("width: ", width, "px");
///
/// assert_eq!(style.to_string(), "width: 320px");
/// ```
#[macro_export]
macro_rules! concat {
    () => {
        ()
    };

    ($literal:literal $(,)?) => {
        $crate::__static_text!(::core::concat!($literal))
    };

    ($literal:literal, $($rest:tt)+) => {
        $crate::Chain(
            $crate::__static_text!(::core::concat!($literal)),
            $crate::concat!($($rest)+),
        )
    };

    ($value:expr $(,)?) => {
        $value
    };

    ($value:expr, $($rest:tt)+) => {
        $crate::Chain($value, $crate::concat!($($rest)+))
    };
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
