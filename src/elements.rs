use crate::html::{Chain, Segments, Text};
use crate::{Attributes, Html};

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Element<N, A, C> {
    pub name: N,
    pub attrs: A,
    pub children: C,
}

impl<N, A, C> Html for Element<N, A, C>
where
    N: Html + Clone,
    A: Attributes,
    C: Html,
{
    type Segments<T: Segments> = <Chain<
        Text<1>,
        Chain<N, Chain<A, Chain<Text<1>, Chain<C, Chain<Text<2>, Chain<N, Text<1>>>>>>>,
    > as Html>::Segments<T>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        Chain(
            crate::text!(@raw "<"),
            Chain(
                self.name.clone(),
                Chain(
                    self.attrs,
                    Chain(
                        crate::text!(@raw ">"),
                        Chain(
                            self.children,
                            Chain(
                                crate::text!(@raw "</"),
                                Chain(self.name, crate::text!(@raw ">")),
                            ),
                        ),
                    ),
                ),
            ),
        )
        .segments(x)
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Void<N, A> {
    pub name: N,
    pub attrs: A,
}

impl<N, A> Html for Void<N, A>
where
    N: Html,
    A: Attributes,
{
    type Segments<T: Segments> = <Chain<Text<1>, Chain<N, Chain<A, Text<1>>>> as Html>::Segments<T>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        Chain(
            crate::text!(@raw "<"),
            Chain(self.name, Chain(self.attrs, crate::text!(@raw ">"))),
        )
        .segments(x)
    }
}

macro_rules! __element {
    (
        $(#[$meta:meta])*
        $vis:vis fn $ident:ident;
    ) => {
        #[allow(non_snake_case)]
        #[allow(rustdoc::invalid_html_tags)]
        $(#[$meta])*
        $vis fn $ident(
            attrs: $crate::Attrs<impl $crate::Attributes>,
            children: $crate::Children<impl $crate::Html>,
        ) -> impl $crate::Html {
            $crate::elements::Element {
                name: $crate::text!(@raw ::core::stringify!($ident)),
                attrs: attrs.0,
                children: children.0,
            }
        }
    };
}

macro_rules! __void {
    (
        $(#[$meta:meta])*
        $vis:vis fn $ident:ident;
    ) => {
        #[allow(non_snake_case)]
        #[allow(rustdoc::invalid_html_tags)]
        $(#[$meta])*
        $vis fn $ident(
            attrs: $crate::Attrs<impl $crate::Attributes>,
        ) -> impl $crate::Html {
            $crate::elements::Void {
                name: $crate::text!(@raw ::core::stringify!($ident)),
                attrs: attrs.0,
            }
        }
    };
}

__element! {
    /// Creates an HTML `<a>` element.
    ///
    /// The `<a>` HTML element creates a hyperlink to a URL, file, email address, or location within a page.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a)
    pub fn a;
}

__element! {
    /// Creates an HTML `<abbr>` element.
    ///
    /// The `<abbr>` HTML element represents an abbreviation or acronym.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/abbr)
    pub fn abbr;
}

__element! {
    /// Creates an HTML `<acronym>` element.
    ///
    /// The `<acronym>` HTML element allows authors to clearly indicate a sequence of characters that compose an acronym or abbreviation for a word.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/acronym)
    pub fn acronym;
}

__element! {
    /// Creates an HTML `<address>` element.
    ///
    /// The `<address>` HTML element indicates that the enclosed HTML provides contact information for a person or people, or for an organization.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/address)
    pub fn address;
}

__void! {
    /// Creates an HTML `<area>` element.
    ///
    /// The `<area>` HTML element defines an area inside an image map that has predefined clickable areas.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area)
    pub fn area;
}

__element! {
    /// Creates an HTML `<article>` element.
    ///
    /// The `<article>` HTML element represents a self-contained composition that can stand on its own or be reused independently.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/article)
    pub fn article;
}

__element! {
    /// Creates an HTML `<aside>` element.
    ///
    /// The `<aside>` HTML element represents a portion of a document whose content is only indirectly related to the document's main content.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/aside)
    pub fn aside;
}

__element! {
    /// Creates an HTML `<audio>` element.
    ///
    /// The `<audio>` HTML element is used to embed sound content in documents.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio)
    pub fn audio;
}

__element! {
    /// Creates an HTML `<b>` element.
    ///
    /// The `<b>` HTML element is used to draw the reader's attention to the element's contents, which are not otherwise granted special importance.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/b)
    pub fn b;
}

__void! {
    /// Creates an HTML `<base>` element.
    ///
    /// The `<base>` HTML element specifies the base URL to use for all _relative_ URLs in a document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/base)
    pub fn base;
}

__element! {
    /// Creates an HTML `<bdi>` element.
    ///
    /// The `<bdi>` HTML element tells the browser's bidirectional algorithm to treat the text it contains in isolation from its surrounding text.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdi)
    pub fn bdi;
}

__element! {
    /// Creates an HTML `<bdo>` element.
    ///
    /// The `<bdo>` HTML element overrides the current directionality of text, so that the text within is rendered in a different direction.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdo)
    pub fn bdo;
}

__element! {
    /// Creates an HTML `<big>` element.
    ///
    /// The `<big>` HTML deprecated element renders the enclosed text at a font size one level larger than the surrounding text (`medium` becomes `large`, for example).
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/big)
    pub fn big;
}

__element! {
    /// Creates an HTML `<blockquote>` element.
    ///
    /// The `<blockquote>` HTML element indicates that the enclosed text is an extended quotation.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote)
    pub fn blockquote;
}

__element! {
    /// Creates an HTML `<body>` element.
    ///
    /// The `<body>` HTML element represents the content of an HTML document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/body)
    pub fn body;
}

__void! {
    /// Creates an HTML `<br>` element.
    ///
    /// The `<br>` HTML element produces a line break in text (carriage-return).
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/br)
    pub fn br;
}

__element! {
    /// Creates an HTML `<button>` element.
    ///
    /// The `<button>` HTML element is an interactive element activated by a user with a mouse, keyboard, finger, voice command, or other assistive technology.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button)
    pub fn button;
}

__element! {
    /// Creates an HTML `<canvas>` element.
    ///
    /// The `<canvas>` HTML element provides a drawing surface for the Canvas and WebGL APIs.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas)
    pub fn canvas;
}

__element! {
    /// Creates an HTML `<caption>` element.
    ///
    /// The `<caption>` HTML element specifies the caption (or title) of a table, providing the table an accessible name or accessible description.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/caption)
    pub fn caption;
}

__element! {
    /// Creates an HTML `<center>` element.
    ///
    /// The `<center>` HTML element is a block-level element that displays its block-level or inline contents centered horizontally within its containing element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/center)
    pub fn center;
}

__element! {
    /// Creates an HTML `<cite>` element.
    ///
    /// The `<cite>` HTML element is used to mark up the title of a creative work.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/cite)
    pub fn cite;
}

__element! {
    /// Creates an HTML `<code>` element.
    ///
    /// The `<code>` HTML element displays its contents styled in a fashion intended to indicate that the text is a short fragment of computer code.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/code)
    pub fn code;
}

__void! {
    /// Creates an HTML `<col>` element.
    ///
    /// The `<col>` HTML element defines one or more columns in a column group represented by its parent `<colgroup>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/col)
    pub fn col;
}

__element! {
    /// Creates an HTML `<colgroup>` element.
    ///
    /// The `<colgroup>` HTML element defines a group of columns within a table.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/colgroup)
    pub fn colgroup;
}

__element! {
    /// Creates an HTML `<data>` element.
    ///
    /// The `<data>` HTML element links a given piece of content with a machine-readable translation.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/data)
    pub fn data;
}

__element! {
    /// Creates an HTML `<datalist>` element.
    ///
    /// The `<datalist>` HTML element contains a set of `<option>` elements that represent the permissible or recommended options available to choose from within other controls.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/datalist)
    pub fn datalist;
}

__element! {
    /// Creates an HTML `<dd>` element.
    ///
    /// The `<dd>` HTML element provides the description, definition, or value for the preceding term (`<dt>`) in a description list (`<dl>`).
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dd)
    pub fn dd;
}

__element! {
    /// Creates an HTML `<del>` element.
    ///
    /// The `<del>` HTML element represents a range of text that has been deleted from a document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del)
    pub fn del;
}

__element! {
    /// Creates an HTML `<details>` element.
    ///
    /// The `<details>` HTML element creates a disclosure widget in which information is visible only when the widget is toggled into an open state.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details)
    pub fn details;
}

__element! {
    /// Creates an HTML `<dfn>` element.
    ///
    /// The `<dfn>` HTML element indicates a term to be defined.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dfn)
    pub fn dfn;
}

__element! {
    /// Creates an HTML `<dialog>` element.
    ///
    /// The `<dialog>` HTML element represents a modal or non-modal dialog box or other interactive component, such as a dismissible alert, inspector, or subwindow.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog)
    pub fn dialog;
}

__element! {
    /// Creates an HTML `<dir>` element.
    ///
    /// The obsolete `<dir>` HTML element represents a directory of files or folders.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dir)
    pub fn dir;
}

__element! {
    /// Creates an HTML `<div>` element.
    ///
    /// The `<div>` HTML element is the generic container for flow content.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/div)
    pub fn div;
}

__element! {
    /// Creates an HTML `<dl>` element.
    ///
    /// The `<dl>` HTML element represents a description list.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dl)
    pub fn dl;
}

__element! {
    /// Creates an HTML `<dt>` element.
    ///
    /// The `<dt>` HTML element specifies a term in a description or definition list, and as such must be used inside a `<dl>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dt)
    pub fn dt;
}

__element! {
    /// Creates an HTML `<em>` element.
    ///
    /// The `<em>` HTML element marks text that has stress emphasis.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/em)
    pub fn em;
}

__void! {
    /// Creates an HTML `<embed>` element.
    ///
    /// The `<embed>` HTML element embeds external content at the specified point in the document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed)
    pub fn embed;
}

__element! {
    /// Creates an HTML `<fencedframe>` element.
    ///
    /// The `<fencedframe>` HTML element represents a nested browsing context, embedding another HTML page into the current one.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/fencedframe)
    pub fn fencedframe;
}

__element! {
    /// Creates an HTML `<fieldset>` element.
    ///
    /// The `<fieldset>` HTML element is used to group several controls as well as labels (`<label>`) within a web form.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/fieldset)
    pub fn fieldset;
}

__element! {
    /// Creates an HTML `<figcaption>` element.
    ///
    /// The `<figcaption>` HTML element provides a caption or legend for its parent `<figure>`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figcaption)
    pub fn figcaption;
}

__element! {
    /// Creates an HTML `<figure>` element.
    ///
    /// The `<figure>` HTML element represents self-contained content, potentially with an optional caption, which is specified using the `<figcaption>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figure)
    pub fn figure;
}

__element! {
    /// Creates an HTML `<font>` element.
    ///
    /// The `<font>` HTML element defines the font size, color and face for its content.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/font)
    pub fn font;
}

__element! {
    /// Creates an HTML `<footer>` element.
    ///
    /// The `<footer>` HTML element represents a footer for its nearest ancestor sectioning content or sectioning root element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/footer)
    pub fn footer;
}

__element! {
    /// Creates an HTML `<form>` element.
    ///
    /// The `<form>` HTML element represents a document section containing interactive controls for submitting information.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form)
    pub fn form;
}

__void! {
    /// Creates an HTML `<frame>` element.
    ///
    /// The `<frame>` HTML element defines a particular area in which another HTML document can be displayed.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/frame)
    pub fn frame;
}

__element! {
    /// Creates an HTML `<frameset>` element.
    ///
    /// The `<frameset>` HTML element is used to contain `<frame>` elements.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/frameset)
    pub fn frameset;
}

__element! {
    /// Creates an HTML `<geolocation>` element.
    ///
    /// The `<geolocation>` HTML element creates an interactive control for the user to share their location data with the page.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/geolocation)
    pub fn geolocation;
}

__element! {
    /// Creates an HTML `<head>` element.
    ///
    /// The `<head>` HTML element contains machine-readable information (metadata) about the document, like its title, scripts, and style sheets.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/head)
    pub fn head;
}

__element! {
    /// Creates an HTML `<header>` element.
    ///
    /// The `<header>` HTML element represents introductory content, typically a group of introductory or navigational aids.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/header)
    pub fn header;
}

__element! {
    /// Creates an HTML `<h1>` element.
    ///
    /// The `<h1>` to `<h6>` HTML elements represent six levels of section headings.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h1;
}

__element! {
    /// Creates an HTML `<h2>` element.
    ///
    /// The `<h1>` to `<h6>` HTML elements represent six levels of section headings.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h2;
}

__element! {
    /// Creates an HTML `<h3>` element.
    ///
    /// The `<h1>` to `<h6>` HTML elements represent six levels of section headings.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h3;
}

__element! {
    /// Creates an HTML `<h4>` element.
    ///
    /// The `<h1>` to `<h6>` HTML elements represent six levels of section headings.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h4;
}

__element! {
    /// Creates an HTML `<h5>` element.
    ///
    /// The `<h1>` to `<h6>` HTML elements represent six levels of section headings.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h5;
}

__element! {
    /// Creates an HTML `<h6>` element.
    ///
    /// The `<h1>` to `<h6>` HTML elements represent six levels of section headings.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h6;
}

__element! {
    /// Creates an HTML `<hgroup>` element.
    ///
    /// The `<hgroup>` HTML element represents a heading and related content.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/hgroup)
    pub fn hgroup;
}

__void! {
    /// Creates an HTML `<hr>` element.
    ///
    /// The `<hr>` HTML element represents a thematic break between sections of content.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/hr)
    pub fn hr;
}

__element! {
    /// Creates an HTML `<html>` element.
    ///
    /// The `<html>` HTML element represents the root (top-level element) of an HTML document, so it is also referred to as the *root element*. All other elements must be descendants of this element. There can be only one `<html>` element in a document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/html)
    pub fn html;
}

__element! {
    /// Creates an HTML `<i>` element.
    ///
    /// The `<i>` HTML element marks text that is set apart from the surrounding prose, such as an idiom, technical term, or taxonomic name.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/i)
    pub fn i;
}

__element! {
    /// Creates an HTML `<iframe>` element.
    ///
    /// The `<iframe>` HTML element represents a nested browsing context, embedding another document into the current one.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe)
    pub fn iframe;
}

__void! {
    /// Creates an HTML `<img>` element.
    ///
    /// The `<img>` HTML element embeds an image into the document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img)
    pub fn img;
}

__void! {
    /// Creates an HTML `<input>` element.
    ///
    /// The `<input>` HTML element creates an interactive form control for collecting user input.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input)
    pub fn input;
}

__element! {
    /// Creates an HTML `<ins>` element.
    ///
    /// The `<ins>` HTML element represents a range of text that has been added to a document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ins)
    pub fn ins;
}

__element! {
    /// Creates an HTML `<kbd>` element.
    ///
    /// The `<kbd>` HTML element represents user input (typically keyboard input).
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/kbd)
    pub fn kbd;
}

__element! {
    /// Creates an HTML `<label>` element.
    ///
    /// The `<label>` HTML element represents a caption for an item in a user interface.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/label)
    pub fn label;
}

__element! {
    /// Creates an HTML `<legend>` element.
    ///
    /// The `<legend>` HTML element represents a caption for the content of its parent `<fieldset>`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/legend)
    pub fn legend;
}

__element! {
    /// Creates an HTML `<li>` element.
    ///
    /// The `<li>` HTML element is used to represent an item in a list.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/li)
    pub fn li;
}

__void! {
    /// Creates an HTML `<link>` element.
    ///
    /// The `<link>` HTML element specifies relationships between the current document and an external resource.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link)
    pub fn link;
}

__element! {
    /// Creates an HTML `<main>` element.
    ///
    /// The `<main>` HTML element represents the dominant content of the `<body>` of a document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/main)
    pub fn main;
}

__element! {
    /// Creates an HTML `<map>` element.
    ///
    /// The `<map>` HTML element is used with `<area>` elements to define an image map (a clickable link area).
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/map)
    pub fn map;
}

__element! {
    /// Creates an HTML `<mark>` element.
    ///
    /// The `<mark>` HTML element highlights text because it is relevant in the current context.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/mark)
    pub fn mark;
}

__element! {
    /// Creates an HTML `<marquee>` element.
    ///
    /// The `<marquee>` HTML element is used to insert a scrolling area of text.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/marquee)
    pub fn marquee;
}

__element! {
    /// Creates an HTML `<menu>` element.
    ///
    /// The `<menu>` HTML element represents an unordered list of items and is treated by browsers like `<ul>`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/menu)
    pub fn menu;
}

__void! {
    /// Creates an HTML `<meta>` element.
    ///
    /// The `<meta>` HTML element represents Metadata that cannot be represented by other meta-related elements, such as `<base>`, `<link>`, `<script>`, `<style>`, or `<title>`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta)
    pub fn meta;
}

__element! {
    /// Creates an HTML `<meter>` element.
    ///
    /// The `<meter>` HTML element represents either a scalar value within a known range or a fractional value.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter)
    pub fn meter;
}

__element! {
    /// Creates an HTML `<nav>` element.
    ///
    /// The `<nav>` HTML element represents a section containing navigation links.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/nav)
    pub fn nav;
}

__element! {
    /// Creates an HTML `<nobr>` element.
    ///
    /// The obsolete `<nobr>` HTML element prevents its text from wrapping automatically.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/nobr)
    pub fn nobr;
}

__element! {
    /// Creates an HTML `<noembed>` element.
    ///
    /// The obsolete `<noembed>` HTML element provides fallback content for browsers that do not support `<embed>`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/noembed)
    pub fn noembed;
}

__element! {
    /// Creates an HTML `<noframes>` element.
    ///
    /// The `<noframes>` HTML element provides content to be presented in browsers that don't support (or have disabled support for) the `<frame>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/noframes)
    pub fn noframes;
}

__element! {
    /// Creates an HTML `<noscript>` element.
    ///
    /// The `<noscript>` HTML element provides fallback content when scripting is unavailable or disabled.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/noscript)
    pub fn noscript;
}

__element! {
    /// Creates an HTML `<object>` element.
    ///
    /// The `<object>` HTML element embeds an external resource such as an image, document, or nested browsing context.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object)
    pub fn object;
}

__element! {
    /// Creates an HTML `<ol>` element.
    ///
    /// The `<ol>` HTML element represents an ordered list of items — typically rendered as a numbered list.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol)
    pub fn ol;
}

__element! {
    /// Creates an HTML `<optgroup>` element.
    ///
    /// The `<optgroup>` HTML element creates a grouping of options within a `<select>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/optgroup)
    pub fn optgroup;
}

__element! {
    /// Creates an HTML `<option>` element.
    ///
    /// The `<option>` HTML element is used to define an item contained in a `<select>`, an `<optgroup>`, or a `<datalist>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option)
    pub fn option;
}

__element! {
    /// Creates an HTML `<output>` element.
    ///
    /// The `<output>` HTML element contains the result of a calculation or user action.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/output)
    pub fn output;
}

__element! {
    /// Creates an HTML `<p>` element.
    ///
    /// The `<p>` HTML element represents a paragraph.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/p)
    pub fn p;
}

__void! {
    /// Creates an HTML `<param>` element.
    ///
    /// The `<param>` HTML element defines parameters for an `<object>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/param)
    pub fn param;
}

__element! {
    /// Creates an HTML `<picture>` element.
    ///
    /// The `<picture>` HTML element contains zero or more `<source>` elements and one `<img>` element to offer alternative versions of an image for different display/device scenarios.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/picture)
    pub fn picture;
}

__element! {
    /// Creates an HTML `<plaintext>` element.
    ///
    /// The `<plaintext>` HTML element renders everything following the start tag as raw text, ignoring any following HTML.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/plaintext)
    pub fn plaintext;
}

__element! {
    /// Creates an HTML `<pre>` element.
    ///
    /// The `<pre>` HTML element represents preformatted text which is to be presented exactly as written in the HTML file.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/pre)
    pub fn pre;
}

__element! {
    /// Creates an HTML `<progress>` element.
    ///
    /// The `<progress>` HTML element displays an indicator showing the completion progress of a task, typically displayed as a progress bar.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/progress)
    pub fn progress;
}

__element! {
    /// Creates an HTML `<q>` element.
    ///
    /// The `<q>` HTML element indicates that the enclosed text is a short inline quotation.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q)
    pub fn q;
}

__element! {
    /// Creates an HTML `<rb>` element.
    ///
    /// The `<rb>` HTML element identifies the base text of a ruby annotation.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rb)
    pub fn rb;
}

__element! {
    /// Creates an HTML `<rp>` element.
    ///
    /// The `<rp>` HTML element provides fallback parentheses for browsers that do not display ruby annotations.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rp)
    pub fn rp;
}

__element! {
    /// Creates an HTML `<rt>` element.
    ///
    /// The `<rt>` HTML element provides pronunciation, translation, or transliteration text for a ruby annotation.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rt)
    pub fn rt;
}

__element! {
    /// Creates an HTML `<rtc>` element.
    ///
    /// The `<rtc>` HTML element embraces semantic annotations of characters presented in a ruby of `<rb>` elements used inside of element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rtc)
    pub fn rtc;
}

__element! {
    /// Creates an HTML `<ruby>` element.
    ///
    /// The `<ruby>` HTML element adds small annotations alongside base text, commonly to show pronunciation.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ruby)
    pub fn ruby;
}

__element! {
    /// Creates an HTML `<s>` element.
    ///
    /// The `<s>` HTML element renders text with a strikethrough, or a line through it.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/s)
    pub fn s;
}

__element! {
    /// Creates an HTML `<samp>` element.
    ///
    /// The `<samp>` HTML element is used to enclose inline text which represents sample (or quoted) output from a computer program.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/samp)
    pub fn samp;
}

__element! {
    /// Creates an HTML `<script>` element.
    ///
    /// The `<script>` HTML element is used to embed executable code or data; this is typically used to embed or refer to JavaScript code.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script)
    pub fn script;
}

__element! {
    /// Creates an HTML `<search>` element.
    ///
    /// The `<search>` HTML element groups controls and content used to perform or filter a search.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/search)
    pub fn search;
}

__element! {
    /// Creates an HTML `<section>` element.
    ///
    /// The `<section>` HTML element represents a generic standalone section of a document, which doesn't have a more specific semantic element to represent it.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/section)
    pub fn section;
}

__element! {
    /// Creates an HTML `<select>` element.
    ///
    /// The `<select>` HTML element represents a control that provides a menu of options.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/select)
    pub fn select;
}

__element! {
    /// Creates an HTML `<selectedcontent>` element.
    ///
    /// The `<selectedcontent>` HTML is used inside a `<select>` element to display the contents of its currently selected `<option>` within its first child `<button>`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/selectedcontent)
    pub fn selectedcontent;
}

__element! {
    /// Creates an HTML `<slot>` element.
    ///
    /// The `<slot>` HTML element is a placeholder inside a Web Component that you can fill with your own markup when the component is used.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/slot)
    pub fn slot;
}

__element! {
    /// Creates an HTML `<small>` element.
    ///
    /// The `<small>` HTML element represents side-comments and small print, like copyright and legal text, independent of its styled presentation.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/small)
    pub fn small;
}

__void! {
    /// Creates an HTML `<source>` element.
    ///
    /// The `<source>` HTML element specifies one or more media resources for the `<picture>`, `<audio>`, and `<video>` elements.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source)
    pub fn source;
}

__element! {
    /// Creates an HTML `<span>` element.
    ///
    /// The `<span>` HTML element is a generic inline container for phrasing content, which does not inherently represent anything.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/span)
    pub fn span;
}

__element! {
    /// Creates an HTML `<strike>` element.
    ///
    /// The `<strike>` HTML element places a strikethrough (horizontal line) over text.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/strike)
    pub fn strike;
}

__element! {
    /// Creates an HTML `<strong>` element.
    ///
    /// The `<strong>` HTML element indicates that its contents have strong importance, seriousness, or urgency.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/strong)
    pub fn strong;
}

__element! {
    /// Creates an HTML `<style>` element.
    ///
    /// The `<style>` HTML element contains style information for a document, or part of a document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/style)
    pub fn style;
}

__element! {
    /// Creates an HTML `<sub>` element.
    ///
    /// The `<sub>` HTML element specifies inline text which should be displayed as subscript for solely typographical reasons.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sub)
    pub fn sub;
}

__element! {
    /// Creates an HTML `<summary>` element.
    ///
    /// The `<summary>` HTML element specifies a summary, caption, or legend for a `<details>` element's disclosure box.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/summary)
    pub fn summary;
}

__element! {
    /// Creates an HTML `<sup>` element.
    ///
    /// The `<sup>` HTML element specifies inline text which is to be displayed as superscript for solely typographical reasons.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sup)
    pub fn sup;
}

__element! {
    /// Creates an HTML `<table>` element.
    ///
    /// The `<table>` HTML element represents tabular data—that is, information presented in a two-dimensional table comprised of rows and columns of cells containing data.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/table)
    pub fn table;
}

__element! {
    /// Creates an HTML `<tbody>` element.
    ///
    /// The `<tbody>` HTML element encapsulates a set of table rows (`<tr>` elements), indicating that they comprise the body of a table's (main) data.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tbody)
    pub fn tbody;
}

__element! {
    /// Creates an HTML `<td>` element.
    ///
    /// The `<td>` HTML element defines a cell of a table that contains data and may be used as a child of the `<tr>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td)
    pub fn td;
}

__element! {
    /// Creates an HTML `<template>` element.
    ///
    /// The `<template>` HTML element stores markup that is not rendered immediately and can be instantiated later.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template)
    pub fn template;
}

__element! {
    /// Creates an HTML `<textarea>` element.
    ///
    /// The `<textarea>` HTML element provides a multiline plain-text editing control.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea)
    pub fn textarea;
}

__element! {
    /// Creates an HTML `<tfoot>` element.
    ///
    /// The `<tfoot>` HTML element groups table rows that form a table footer.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tfoot)
    pub fn tfoot;
}

__element! {
    /// Creates an HTML `<th>` element.
    ///
    /// The `<th>` HTML element defines a header cell within a table row.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th)
    pub fn th;
}

__element! {
    /// Creates an HTML `<thead>` element.
    ///
    /// The `<thead>` HTML element groups table rows that form a table header.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/thead)
    pub fn thead;
}

__element! {
    /// Creates an HTML `<time>` element.
    ///
    /// The `<time>` HTML element represents a specific period in time.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/time)
    pub fn time;
}

__element! {
    /// Creates an HTML `<title>` element.
    ///
    /// The `<title>` HTML element defines the document's title that is shown in a Browser's title bar or a page's tab.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/title)
    pub fn title;
}

__element! {
    /// Creates an HTML `<tr>` element.
    ///
    /// The `<tr>` HTML element defines a row of cells in a table.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tr)
    pub fn tr;
}

__void! {
    /// Creates an HTML `<track>` element.
    ///
    /// The `<track>` HTML element is used as a child of the media elements, `<audio>` and `<video>`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track)
    pub fn track;
}

__element! {
    /// Creates an HTML `<tt>` element.
    ///
    /// The `<tt>` HTML element creates inline text which is presented using the user agent default monospace font face.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tt)
    pub fn tt;
}

__element! {
    /// Creates an HTML `<u>` element.
    ///
    /// The `<u>` HTML element represents a span of inline text which should be rendered in a way that indicates that it has a non-textual annotation.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/u)
    pub fn u;
}

__element! {
    /// Creates an HTML `<ul>` element.
    ///
    /// The `<ul>` HTML element represents an unordered list of items, typically rendered as a bulleted list.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ul)
    pub fn ul;
}

__element! {
    /// Creates an HTML `<var>` element.
    ///
    /// The `<var>` HTML element represents the name of a variable in a mathematical expression or a programming context.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/var)
    pub fn var;
}

__element! {
    /// Creates an HTML `<video>` element.
    ///
    /// The `<video>` HTML element embeds a media player which supports video playback into the document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video)
    pub fn video;
}

__void! {
    /// Creates an HTML `<wbr>` element.
    ///
    /// The `<wbr>` HTML element marks a position where the browser may break a line.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/wbr)
    pub fn wbr;
}

__element! {
    /// Creates an HTML `<xmp>` element.
    ///
    /// The `<xmp>` HTML element renders text between the start and end tags without interpreting the HTML in between and using a monospaced font.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/xmp)
    pub fn xmp;
}

__element! {
    /// Creates an SVG `<animate>` element.
    ///
    /// The `<animate>` SVG element provides a way to animate an attribute of an element over time.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/animate)
    pub fn animate;
}

__element! {
    /// Creates an SVG `<animateMotion>` element.
    ///
    /// The `<animateMotion>` SVG element provides a way to define how an element moves along a motion path.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/animateMotion)
    pub fn animateMotion;
}

__element! {
    /// Creates an SVG `<animateTransform>` element.
    ///
    /// The `<animateTransform>` SVG element animates translation, scaling, rotation, or skew transformations on its target.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/animateTransform)
    pub fn animateTransform;
}

__element! {
    /// Creates an SVG `<circle>` element.
    ///
    /// The `<circle>` SVG element is an SVG basic shape, used to draw circles based on a center point and a radius.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/circle)
    pub fn circle;
}

__element! {
    /// Creates an SVG `<clipPath>` element.
    ///
    /// The `<clipPath>` SVG element defines a clipping path, to be used by the property.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/clipPath)
    pub fn clipPath;
}

__element! {
    /// Creates an SVG `<defs>` element.
    ///
    /// The `<defs>` SVG element is used to store graphical objects that will be used at a later time.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/defs)
    pub fn defs;
}

__element! {
    /// Creates an SVG `<desc>` element.
    ///
    /// The `<desc>` SVG element provides an accessible, long-text description of any SVG container element or graphics element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/desc)
    pub fn desc;
}

__element! {
    /// Creates an SVG `<ellipse>` element.
    ///
    /// The `<ellipse>` SVG element is an SVG basic shape, used to create ellipses based on a center coordinate, and both their x and y radius.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/ellipse)
    pub fn ellipse;
}

__element! {
    /// Creates an SVG `<feBlend>` element.
    ///
    /// The `<feBlend>` SVG filter primitive composes two objects together ruled by a certain blending mode.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feBlend)
    pub fn feBlend;
}

__element! {
    /// Creates an SVG `<feColorMatrix>` element.
    ///
    /// The `<feColorMatrix>` SVG filter element changes colors based on a transformation matrix.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feColorMatrix)
    pub fn feColorMatrix;
}

__element! {
    /// Creates an SVG `<feComponentTransfer>` element.
    ///
    /// The `<feComponentTransfer>` SVG filter primitive performs color-component-wise remapping of data for each pixel.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feComponentTransfer)
    pub fn feComponentTransfer;
}

__element! {
    /// Creates an SVG `<feComposite>` element.
    ///
    /// The `<feComposite>` SVG filter primitive combines two input images pixel by pixel using a selected compositing operation.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feComposite)
    pub fn feComposite;
}

__element! {
    /// Creates an SVG `<feConvolveMatrix>` element.
    ///
    /// The `<feConvolveMatrix>` SVG filter primitive applies a matrix convolution filter effect.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feConvolveMatrix)
    pub fn feConvolveMatrix;
}

__element! {
    /// Creates an SVG `<feDiffuseLighting>` element.
    ///
    /// The `<feDiffuseLighting>` SVG filter primitive lights an image using the alpha channel as a bump map.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feDiffuseLighting)
    pub fn feDiffuseLighting;
}

__element! {
    /// Creates an SVG `<feDisplacementMap>` element.
    ///
    /// The `<feDisplacementMap>` SVG filter primitive uses the pixel values from the image from to spatially displace the image from .
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feDisplacementMap)
    pub fn feDisplacementMap;
}

__element! {
    /// Creates an SVG `<feDistantLight>` element.
    ///
    /// The `<feDistantLight>` SVG element defines a distant light source that can be used within a lighting filter primitive: `<feDiffuseLighting>` or `<feSpecularLighting>`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feDistantLight)
    pub fn feDistantLight;
}

__element! {
    /// Creates an SVG `<feDropShadow>` element.
    ///
    /// The `<feDropShadow>` SVG filter primitive creates a drop shadow of the input image.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feDropShadow)
    pub fn feDropShadow;
}

__element! {
    /// Creates an SVG `<feFlood>` element.
    ///
    /// The `<feFlood>` SVG filter primitive fills the filter subregion with the color and opacity defined by and .
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feFlood)
    pub fn feFlood;
}

__element! {
    /// Creates an SVG `<feFuncA>` element.
    ///
    /// The `<feFuncA>` SVG filter primitive defines the transfer function for the alpha component of the input graphic of its parent `<feComponentTransfer>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feFuncA)
    pub fn feFuncA;
}

__element! {
    /// Creates an SVG `<feFuncB>` element.
    ///
    /// The `<feFuncB>` SVG filter primitive defines the transfer function for the blue component of the input graphic of its parent `<feComponentTransfer>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feFuncB)
    pub fn feFuncB;
}

__element! {
    /// Creates an SVG `<feFuncG>` element.
    ///
    /// The `<feFuncG>` SVG filter primitive defines the transfer function for the green component of the input graphic of its parent `<feComponentTransfer>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feFuncG)
    pub fn feFuncG;
}

__element! {
    /// Creates an SVG `<feFuncR>` element.
    ///
    /// The `<feFuncR>` SVG filter primitive defines the transfer function for the red component of the input graphic of its parent `<feComponentTransfer>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feFuncR)
    pub fn feFuncR;
}

__element! {
    /// Creates an SVG `<feGaussianBlur>` element.
    ///
    /// The `<feGaussianBlur>` SVG filter primitive blurs the input image by the amount specified in , which defines the bell-curve.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feGaussianBlur)
    pub fn feGaussianBlur;
}

__element! {
    /// Creates an SVG `<feImage>` element.
    ///
    /// The `<feImage>` SVG filter primitive loads image data from an external source for use in a filter.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feImage)
    pub fn feImage;
}

__element! {
    /// Creates an SVG `<feMerge>` element.
    ///
    /// The `<feMerge>` SVG element allows filter effects to be applied concurrently instead of sequentially.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feMerge)
    pub fn feMerge;
}

__element! {
    /// Creates an SVG `<feMergeNode>` element.
    ///
    /// The `<feMergeNode>` SVG takes the result of another filter to be processed by its parent .
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feMergeNode)
    pub fn feMergeNode;
}

__element! {
    /// Creates an SVG `<feMorphology>` element.
    ///
    /// The `<feMorphology>` SVG filter primitive is used to erode or dilate the input image.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feMorphology)
    pub fn feMorphology;
}

__element! {
    /// Creates an SVG `<feOffset>` element.
    ///
    /// The `<feOffset>` SVG filter primitive enables offsetting an input image relative to its current position.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feOffset)
    pub fn feOffset;
}

__element! {
    /// Creates an SVG `<fePointLight>` element.
    ///
    /// The `<fePointLight>` SVG element defines a point light source for a lighting filter.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/fePointLight)
    pub fn fePointLight;
}

__element! {
    /// Creates an SVG `<feSpecularLighting>` element.
    ///
    /// The `<feSpecularLighting>` SVG filter primitive lights a source graphic using the alpha channel as a bump map.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feSpecularLighting)
    pub fn feSpecularLighting;
}

__element! {
    /// Creates an SVG `<feSpotLight>` element.
    ///
    /// The `<feSpotLight>` SVG element defines a light source that can be used to create a spotlight effect.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feSpotLight)
    pub fn feSpotLight;
}

__element! {
    /// Creates an SVG `<feTile>` element.
    ///
    /// The `<feTile>` SVG filter primitive fills a target rectangle with a repeated tile of an input image.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feTile)
    pub fn feTile;
}

__element! {
    /// Creates an SVG `<feTurbulence>` element.
    ///
    /// The `<feTurbulence>` SVG filter primitive creates an image using the Perlin turbulence function.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feTurbulence)
    pub fn feTurbulence;
}

__element! {
    /// Creates an SVG `<filter>` element.
    ///
    /// The `<filter>` SVG element defines a custom filter effect by grouping atomic filter primitives.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/filter)
    pub fn filter;
}

__element! {
    /// Creates an SVG `<foreignObject>` element.
    ///
    /// The `<foreignObject>` SVG element includes elements from a different XML namespace.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/foreignObject)
    pub fn foreignObject;
}

__element! {
    /// Creates an SVG `<g>` element.
    ///
    /// The `<g>` SVG element is a container used to group other SVG elements.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/g)
    pub fn g;
}

__element! {
    /// Creates an SVG `<image>` element.
    ///
    /// The `<image>` SVG element includes images inside SVG documents.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/image)
    pub fn image;
}

__element! {
    /// Creates an SVG `<line>` element.
    ///
    /// The `<line>` SVG element is an SVG basic shape used to create a line connecting two points.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/line)
    pub fn line;
}

__element! {
    /// Creates an SVG `<linearGradient>` element.
    ///
    /// The `<linearGradient>` SVG element lets authors define linear gradients to apply to other SVG elements.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/linearGradient)
    pub fn linearGradient;
}

__element! {
    /// Creates an SVG `<marker>` element.
    ///
    /// The `<marker>` SVG element defines a graphic used for drawing arrowheads or polymarkers on a given `<path>`, `<line>`, `<polyline>` or `<polygon>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/marker)
    pub fn marker;
}

__element! {
    /// Creates an SVG `<mask>` element.
    ///
    /// The `<mask>` SVG element defines a mask for compositing the current object into the background.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/mask)
    pub fn mask;
}

__element! {
    /// Creates an SVG `<metadata>` element.
    ///
    /// The `<metadata>` SVG element adds metadata to SVG content.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/metadata)
    pub fn metadata;
}

__element! {
    /// Creates an SVG `<mpath>` element.
    ///
    /// The `<mpath>` SVG sub-element for the `<animateMotion>` element provides the ability to reference an external `<path>` element as the definition of a motion path.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/mpath)
    pub fn mpath;
}

__element! {
    /// Creates an SVG `<path>` element.
    ///
    /// The `<path>` SVG element is the generic element to define a shape.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/path)
    pub fn path;
}

__element! {
    /// Creates an SVG `<pattern>` element.
    ///
    /// The `<pattern>` SVG element defines a graphics object which can be redrawn at repeated x- and y-coordinate intervals ("tiled") to cover an area.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/pattern)
    pub fn pattern;
}

__element! {
    /// Creates an SVG `<polygon>` element.
    ///
    /// The `<polygon>` SVG element defines a closed shape consisting of a set of connected straight line segments.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/polygon)
    pub fn polygon;
}

__element! {
    /// Creates an SVG `<polyline>` element.
    ///
    /// The `<polyline>` SVG element is an SVG basic shape that creates straight lines connecting several points.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/polyline)
    pub fn polyline;
}

__element! {
    /// Creates an SVG `<radialGradient>` element.
    ///
    /// The `<radialGradient>` SVG element lets authors define radial gradients that can be applied to fill or stroke of graphical elements.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/radialGradient)
    pub fn radialGradient;
}

__element! {
    /// Creates an SVG `<rect>` element.
    ///
    /// The `<rect>` SVG element is a basic SVG shape that draws rectangles, defined by their position, width, and height.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/rect)
    pub fn rect;
}

__element! {
    /// Creates an SVG `<set>` element.
    ///
    /// The `<set>` SVG element provides a method of setting the value of an attribute for a specified duration.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/set)
    pub fn set;
}

__element! {
    /// Creates an SVG `<stop>` element.
    ///
    /// The `<stop>` SVG element defines a color and its position to use on a gradient.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/stop)
    pub fn stop;
}

__element! {
    /// Creates an SVG `<svg>` element.
    ///
    /// The `<svg>` SVG element is a container that defines a new coordinate system and viewport.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/svg)
    pub fn svg;
}

__element! {
    /// Creates an SVG `<switch>` element.
    ///
    /// The `<switch>` SVG element renders the first child whose conditional processing attributes evaluate to true.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/switch)
    pub fn switch;
}

__element! {
    /// Creates an SVG `<symbol>` element.
    ///
    /// The `<symbol>` SVG element is used to define graphical template objects which can be instantiated by a `<use>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/symbol)
    pub fn symbol;
}

__element! {
    /// Creates an SVG `<text>` element.
    ///
    /// The `<text>` SVG element draws a graphics element consisting of text.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/text)
    pub fn text;
}

__element! {
    /// Creates an SVG `<textPath>` element.
    ///
    /// The `<textPath>` SVG element is used to render text along the shape of a `<path>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/textPath)
    pub fn textPath;
}

__element! {
    /// Creates an SVG `<tspan>` element.
    ///
    /// The `<tspan>` SVG element defines a subtext within a element or another `<tspan>` element.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/tspan)
    pub fn tspan;
}

__element! {
    /// Creates an SVG `<view>` element.
    ///
    /// The `<view>` SVG element defines a particular view of an SVG document.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/view)
    pub fn view;
}

__element! {
    /// Creates a MathML `<annotation>` element.
    ///
    /// The `<annotation>` MathML element contains an annotation to the MathML expression in a textual format, for example LaTeX.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/annotation)
    pub fn annotation;
}

__element! {
    /// Creates a MathML `<maction>` element.
    ///
    /// The `<maction>` MathML element binds actions to mathematical expressions.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/maction)
    pub fn maction;
}

__element! {
    /// Creates a MathML `<math>` element.
    ///
    /// The `<math>` MathML element is the top-level MathML element, used to write a single mathematical formula.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/math)
    pub fn math;
}

__element! {
    /// Creates a MathML `<menclose>` element.
    ///
    /// The `<menclose>` MathML element renders its content inside an enclosing notation specified by the `notation` attribute.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/menclose)
    pub fn menclose;
}

__element! {
    /// Creates a MathML `<merror>` element.
    ///
    /// The `<merror>` MathML element is used to display contents as error messages.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/merror)
    pub fn merror;
}

__element! {
    /// Creates a MathML `<mfenced>` element.
    ///
    /// The `<mfenced>` MathML element adds configurable fences and separators around its content.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mfenced)
    pub fn mfenced;
}

__element! {
    /// Creates a MathML `<mfrac>` element.
    ///
    /// The `<mfrac>` MathML element is used to display fractions.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mfrac)
    pub fn mfrac;
}

__element! {
    /// Creates a MathML `<mi>` element.
    ///
    /// The `<mi>` MathML element represents an identifier such as a function name, variable, or symbolic constant.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mi)
    pub fn mi;
}

__element! {
    /// Creates a MathML `<mmultiscripts>` element.
    ///
    /// The `<mmultiscripts>` MathML element attaches multiple prescripts and postscripts to an expression.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mmultiscripts)
    pub fn mmultiscripts;
}

__element! {
    /// Creates a MathML `<mn>` element.
    ///
    /// The `<mn>` MathML element represents a numeric literal.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mn)
    pub fn mn;
}

__element! {
    /// Creates a MathML `<mo>` element.
    ///
    /// The `<mo>` MathML element represents a mathematical operator.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mo)
    pub fn mo;
}

__element! {
    /// Creates a MathML `<mover>` element.
    ///
    /// The `<mover>` MathML element is used to attach an accent or a limit over an expression.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mover)
    pub fn mover;
}

__element! {
    /// Creates a MathML `<mpadded>` element.
    ///
    /// The `<mpadded>` MathML element is used to add extra padding and to set the general adjustment of position and size of enclosed contents.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mpadded)
    pub fn mpadded;
}

__element! {
    /// Creates a MathML `<mphantom>` element.
    ///
    /// The `<mphantom>` MathML element is rendered invisibly, but dimensions (such as height, width, and baseline position) are still kept.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mphantom)
    pub fn mphantom;
}

__element! {
    /// Creates a MathML `<mprescripts>` element.
    ///
    /// The `<mprescripts>` MathML element marks the beginning of prescripts inside `<mmultiscripts>`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mprescripts)
    pub fn mprescripts;
}

__element! {
    /// Creates a MathML `<mroot>` element.
    ///
    /// The `<mroot>` MathML element is used to display roots with an explicit index.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mroot)
    pub fn mroot;
}

__element! {
    /// Creates a MathML `<mrow>` element.
    ///
    /// The `<mrow>` MathML element groups one or more mathematical subexpressions.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mrow)
    pub fn mrow;
}

__element! {
    /// Creates a MathML `<ms>` element.
    ///
    /// The `<ms>` MathML element represents a string literal for programming languages and computer algebra systems.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/ms)
    pub fn ms;
}

__element! {
    /// Creates a MathML `<mspace>` element.
    ///
    /// The `<mspace>` MathML element is used to display a blank space, whose size is set by its attributes.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mspace)
    pub fn mspace;
}

__element! {
    /// Creates a MathML `<msqrt>` element.
    ///
    /// The `<msqrt>` MathML element is used to display square roots (no index is displayed).
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/msqrt)
    pub fn msqrt;
}

__element! {
    /// Creates a MathML `<mstyle>` element.
    ///
    /// The `<mstyle>` MathML element is used to change the style of its children.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mstyle)
    pub fn mstyle;
}

__element! {
    /// Creates a MathML `<msub>` element.
    ///
    /// The `<msub>` MathML element is used to attach a subscript to an expression.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/msub)
    pub fn msub;
}

__element! {
    /// Creates a MathML `<msubsup>` element.
    ///
    /// The `<msubsup>` MathML element is used to attach both a subscript and a superscript, together, to an expression.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/msubsup)
    pub fn msubsup;
}

__element! {
    /// Creates a MathML `<msup>` element.
    ///
    /// The `<msup>` MathML element is used to attach a superscript to an expression.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/msup)
    pub fn msup;
}

__element! {
    /// Creates a MathML `<mtable>` element.
    ///
    /// The `<mtable>` MathML element represents a table or matrix.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mtable)
    pub fn mtable;
}

__element! {
    /// Creates a MathML `<mtd>` element.
    ///
    /// The `<mtd>` MathML element represents a cell in a table or a matrix.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mtd)
    pub fn mtd;
}

__element! {
    /// Creates a MathML `<mtext>` element.
    ///
    /// The `<mtext>` MathML element is used to render arbitrary text with _no_ notational meaning, such as comments or annotations.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mtext)
    pub fn mtext;
}

__element! {
    /// Creates a MathML `<mtr>` element.
    ///
    /// The `<mtr>` MathML element represents a row in a table or a matrix.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mtr)
    pub fn mtr;
}

__element! {
    /// Creates a MathML `<munder>` element.
    ///
    /// The `<munder>` MathML element is used to attach an accent or a limit under an expression.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/munder)
    pub fn munder;
}

__element! {
    /// Creates a MathML `<munderover>` element.
    ///
    /// The `<munderover>` MathML element is used to attach accents or limits both under and over an expression.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/munderover)
    pub fn munderover;
}

__element! {
    /// Creates a MathML `<semantics>` element.
    ///
    /// The `<semantics>` MathML element associates a mathematical expression with annotations such as source notation or semantic metadata.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/semantics)
    pub fn semantics;
}
