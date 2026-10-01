use crate::html::{Chain, Segments, Text};
use crate::{Attributes, Html};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(hidden)]
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
            crate::__static_text!(@raw "<"),
            Chain(
                self.name.clone(),
                Chain(
                    self.attrs,
                    Chain(
                        crate::__static_text!(@raw ">"),
                        Chain(
                            self.children,
                            Chain(
                                crate::__static_text!(@raw "</"),
                                Chain(self.name, crate::__static_text!(@raw ">")),
                            ),
                        ),
                    ),
                ),
            ),
        )
        .segments(x)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(hidden)]
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
            crate::__static_text!(@raw "<"),
            Chain(
                self.name,
                Chain(self.attrs, crate::__static_text!(@raw ">")),
            ),
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
                name: $crate::__static_text!(@raw ::core::stringify!($ident)),
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
            _children: $crate::Children<impl $crate::Html>,
        ) -> impl $crate::Html {
            $crate::elements::Void {
                name: $crate::__static_text!(@raw ::core::stringify!($ident)),
                attrs: attrs.0,
            }
        }
    };
}

__element! {
    /// `<a>`
    ///
    /// The **`<a>`** HTML element (or _anchor_ element), with its `href` attribute, creates a hyperlink to web pages, files, email addresses, locations in the same…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a)
    pub fn a;
}

__element! {
    /// `<abbr>`
    ///
    /// The **`<abbr>`** HTML element represents an abbreviation or acronym.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/abbr)
    pub fn abbr;
}

__element! {
    /// `<acronym>`
    ///
    /// The **`<acronym>`** HTML element allows authors to clearly indicate a sequence of characters that compose an acronym or abbreviation for a word.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/acronym)
    pub fn acronym;
}

__element! {
    /// `<address>`
    ///
    /// The **`<address>`** HTML element indicates that the enclosed HTML provides contact information for a person or people, or for an organization.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/address)
    pub fn address;
}

__void! {
    /// `<area>`
    ///
    /// The **`<area>`** HTML element defines an area inside an image map that has predefined clickable areas.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area)
    pub fn area;
}

__element! {
    /// `<article>`
    ///
    /// The **`<article>`** HTML element represents a self-contained composition in a document, page, application, or site, which is intended to be independently distributable or reusable…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/article)
    pub fn article;
}

__element! {
    /// `<aside>`
    ///
    /// The **`<aside>`** HTML element represents a portion of a document whose content is only indirectly related to the document's main content.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/aside)
    pub fn aside;
}

__element! {
    /// `<audio>`
    ///
    /// The **`<audio>`** HTML element is used to embed sound content in documents.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio)
    pub fn audio;
}

__element! {
    /// `<b>`
    ///
    /// The **`<b>`** HTML element is used to draw the reader's attention to the element's contents, which are not otherwise granted special importance.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/b)
    pub fn b;
}

__void! {
    /// `<base>`
    ///
    /// The **`<base>`** HTML element specifies the base URL to use for all _relative_ URLs in a document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/base)
    pub fn base;
}

__element! {
    /// `<bdi>`
    ///
    /// The **`<bdi>`** HTML element tells the browser's bidirectional algorithm to treat the text it contains in isolation from its surrounding text.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdi)
    pub fn bdi;
}

__element! {
    /// `<bdo>`
    ///
    /// The **`<bdo>`** HTML element overrides the current directionality of text, so that the text within is rendered in a different direction.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdo)
    pub fn bdo;
}

__element! {
    /// `<big>`
    ///
    /// The **`<big>`** HTML deprecated element renders the enclosed text at a font size one level larger than the surrounding text (`medium` becomes `large`, for example).
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/big)
    pub fn big;
}

__element! {
    /// `<blockquote>`
    ///
    /// The **`<blockquote>`** HTML element indicates that the enclosed text is an extended quotation.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote)
    pub fn blockquote;
}

__element! {
    /// `<body>`
    ///
    /// The **`<body>`** HTML element represents the content of an HTML document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/body)
    pub fn body;
}

__void! {
    /// `<br>`
    ///
    /// The **`<br>`** HTML element produces a line break in text (carriage-return).
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/br)
    pub fn br;
}

__element! {
    /// `<button>`
    ///
    /// The **`<button>`** HTML element is an interactive element activated by a user with a mouse, keyboard, finger, voice command, or other assistive technology.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button)
    pub fn button;
}

__element! {
    /// `<canvas>`
    ///
    /// Use the **HTML `<canvas>` element** with either the canvas scripting API or the WebGL API to draw graphics and animations.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas)
    pub fn canvas;
}

__element! {
    /// `<caption>`
    ///
    /// The **`<caption>`** HTML element specifies the caption (or title) of a table, providing the table an accessible name or accessible description.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/caption)
    pub fn caption;
}

__element! {
    /// `<center>`
    ///
    /// The **`<center>`** HTML element is a block-level element that displays its block-level or inline contents centered horizontally within its containing element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/center)
    pub fn center;
}

__element! {
    /// `<cite>`
    ///
    /// The **`<cite>`** HTML element is used to mark up the title of a creative work.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/cite)
    pub fn cite;
}

__element! {
    /// `<code>`
    ///
    /// The **`<code>`** HTML element displays its contents styled in a fashion intended to indicate that the text is a short fragment of computer code.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/code)
    pub fn code;
}

__void! {
    /// `<col>`
    ///
    /// The **`<col>`** HTML element defines one or more columns in a column group represented by its parent `<colgroup>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/col)
    pub fn col;
}

__element! {
    /// `<colgroup>`
    ///
    /// The **`<colgroup>`** HTML element defines a group of columns within a table.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/colgroup)
    pub fn colgroup;
}

__element! {
    /// `<data>`
    ///
    /// The **`<data>`** HTML element links a given piece of content with a machine-readable translation.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/data)
    pub fn data;
}

__element! {
    /// `<datalist>`
    ///
    /// The **`<datalist>`** HTML element contains a set of `<option>` elements that represent the permissible or recommended options available to choose from within other controls.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/datalist)
    pub fn datalist;
}

__element! {
    /// `<dd>`
    ///
    /// The **`<dd>`** HTML element provides the description, definition, or value for the preceding term (`<dt>`) in a description list (`<dl>`).
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dd)
    pub fn dd;
}

__element! {
    /// `<del>`
    ///
    /// The **`<del>`** HTML element represents a range of text that has been deleted from a document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del)
    pub fn del;
}

__element! {
    /// `<details>`
    ///
    /// The **`<details>`** HTML element creates a disclosure widget in which information is visible only when the widget is toggled into an open state.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details)
    pub fn details;
}

__element! {
    /// `<dfn>`
    ///
    /// The **`<dfn>`** HTML element indicates a term to be defined.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dfn)
    pub fn dfn;
}

__element! {
    /// `<dialog>`
    ///
    /// The **`<dialog>`** HTML element represents a modal or non-modal dialog box or other interactive component, such as a dismissible alert, inspector, or subwindow.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog)
    pub fn dialog;
}

__element! {
    /// `<dir>`
    ///
    /// The **`<dir>`** HTML element is used as a container for a directory of files and/or folders, potentially with styles and icons applied by the…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dir)
    pub fn dir;
}

__element! {
    /// `<div>`
    ///
    /// The **`<div>`** HTML element is the generic container for flow content.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/div)
    pub fn div;
}

__element! {
    /// `<dl>`
    ///
    /// The **`<dl>`** HTML element represents a description list.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dl)
    pub fn dl;
}

__element! {
    /// `<dt>`
    ///
    /// The **`<dt>`** HTML element specifies a term in a description or definition list, and as such must be used inside a `<dl>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dt)
    pub fn dt;
}

__element! {
    /// `<em>`
    ///
    /// The **`<em>`** HTML element marks text that has stress emphasis.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/em)
    pub fn em;
}

__void! {
    /// `<embed>`
    ///
    /// The **`<embed>`** HTML element embeds external content at the specified point in the document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed)
    pub fn embed;
}

__element! {
    /// `<fencedframe>`
    ///
    /// The **`<fencedframe>`** HTML element represents a nested browsing context, embedding another HTML page into the current one.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/fencedframe)
    pub fn fencedframe;
}

__element! {
    /// `<fieldset>`
    ///
    /// The **`<fieldset>`** HTML element is used to group several controls as well as labels (`<label>`) within a web form.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/fieldset)
    pub fn fieldset;
}

__element! {
    /// `<figcaption>`
    ///
    /// The **`<figcaption>`** HTML element represents a caption or legend describing the rest of the contents of its parent `<figure>` element, providing the `<figure>` an…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figcaption)
    pub fn figcaption;
}

__element! {
    /// `<figure>`
    ///
    /// The **`<figure>`** HTML element represents self-contained content, potentially with an optional caption, which is specified using the `<figcaption>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figure)
    pub fn figure;
}

__element! {
    /// `<font>`
    ///
    /// The **`<font>`** HTML element defines the font size, color and face for its content.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/font)
    pub fn font;
}

__element! {
    /// `<footer>`
    ///
    /// The **`<footer>`** HTML element represents a footer for its nearest ancestor sectioning content or sectioning root element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/footer)
    pub fn footer;
}

__element! {
    /// `<form>`
    ///
    /// The **`<form>`** HTML element represents a document section containing interactive controls for submitting information.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form)
    pub fn form;
}

__void! {
    /// `<frame>`
    ///
    /// The **`<frame>`** HTML element defines a particular area in which another HTML document can be displayed.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/frame)
    pub fn frame;
}

__element! {
    /// `<frameset>`
    ///
    /// The **`<frameset>`** HTML element is used to contain `<frame>` elements.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/frameset)
    pub fn frameset;
}

__element! {
    /// `<geolocation>`
    ///
    /// The **`<geolocation>`** HTML element creates an interactive control for the user to share their location data with the page.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/geolocation)
    pub fn geolocation;
}

__element! {
    /// `<head>`
    ///
    /// The **`<head>`** HTML element contains machine-readable information (metadata) about the document, like its title, scripts, and style sheets.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/head)
    pub fn head;
}

__element! {
    /// `<header>`
    ///
    /// The **`<header>`** HTML element represents introductory content, typically a group of introductory or navigational aids.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/header)
    pub fn header;
}

__element! {
    /// `<h1>`
    ///
    /// The **`<h1>`** to **`<h6>`** HTML elements represent six levels of section headings.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h1;
}

__element! {
    /// `<h2>`
    ///
    /// The **`<h1>`** to **`<h6>`** HTML elements represent six levels of section headings.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h2;
}

__element! {
    /// `<h3>`
    ///
    /// The **`<h1>`** to **`<h6>`** HTML elements represent six levels of section headings.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h3;
}

__element! {
    /// `<h4>`
    ///
    /// The **`<h1>`** to **`<h6>`** HTML elements represent six levels of section headings.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h4;
}

__element! {
    /// `<h5>`
    ///
    /// The **`<h1>`** to **`<h6>`** HTML elements represent six levels of section headings.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h5;
}

__element! {
    /// `<h6>`
    ///
    /// The **`<h1>`** to **`<h6>`** HTML elements represent six levels of section headings.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
    pub fn h6;
}

__element! {
    /// `<hgroup>`
    ///
    /// The **`<hgroup>`** HTML element represents a heading and related content.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/hgroup)
    pub fn hgroup;
}

__void! {
    /// `<hr>`
    ///
    /// The **`<hr>`** HTML element represents a thematic break between elements: for example, a change of scene in a story, or a shift of topic…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/hr)
    pub fn hr;
}

__element! {
    /// `<html>`
    ///
    /// The **`<html>`** HTML element represents the root (top-level element) of an HTML document, so it is also referred to as the *root element*. All other elements must be descendants of this element. There can be only one `<html>` element in a document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/html)
    pub fn html;
}

__element! {
    /// `<i>`
    ///
    /// The **`<i>`** HTML element represents a range of text that is set off from the normal text for some reason, such as idiomatic text…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/i)
    pub fn i;
}

__element! {
    /// `<iframe>`
    ///
    /// The **`<iframe>`** HTML element represents a nested browsing context, embedding another document into the current one.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe)
    pub fn iframe;
}

__void! {
    /// `<img>`
    ///
    /// The **`<img>`** HTML element embeds an image into the document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img)
    pub fn img;
}

__void! {
    /// `<input>`
    ///
    /// The **`<input>`** HTML element is used to create interactive controls for web-based forms in order to accept data from the user; a wide variety…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input)
    pub fn input;
}

__element! {
    /// `<ins>`
    ///
    /// The **`<ins>`** HTML element represents a range of text that has been added to a document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ins)
    pub fn ins;
}

__element! {
    /// `<kbd>`
    ///
    /// The **`<kbd>`** HTML element represents user input (typically keyboard input).
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/kbd)
    pub fn kbd;
}

__element! {
    /// `<label>`
    ///
    /// The **`<label>`** HTML element represents a caption for an item in a user interface.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/label)
    pub fn label;
}

__element! {
    /// `<legend>`
    ///
    /// The **`<legend>`** HTML element represents a caption for the content of its parent `<fieldset>`.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/legend)
    pub fn legend;
}

__element! {
    /// `<li>`
    ///
    /// The **`<li>`** HTML element is used to represent an item in a list.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/li)
    pub fn li;
}

__void! {
    /// `<link>`
    ///
    /// The **`<link>`** HTML element specifies relationships between the current document and an external resource.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link)
    pub fn link;
}

__element! {
    /// `<main>`
    ///
    /// The **`<main>`** HTML element represents the dominant content of the `<body>` of a document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/main)
    pub fn main;
}

__element! {
    /// `<map>`
    ///
    /// The **`<map>`** HTML element is used with `<area>` elements to define an image map (a clickable link area).
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/map)
    pub fn map;
}

__element! {
    /// `<mark>`
    ///
    /// The **`<mark>`** HTML element represents text which is **marked** or **highlighted** for reference or notation purposes due to the marked passage's relevance in the…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/mark)
    pub fn mark;
}

__element! {
    /// `<marquee>`
    ///
    /// The **`<marquee>`** HTML element is used to insert a scrolling area of text.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/marquee)
    pub fn marquee;
}

__element! {
    /// `<menu>`
    ///
    /// The **`<menu>`** HTML element is described in the HTML specification as a semantic alternative to `<ul>`, but treated by browsers (and exposed through the…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/menu)
    pub fn menu;
}

__void! {
    /// `<meta>`
    ///
    /// The **`<meta>`** HTML element represents Metadata that cannot be represented by other meta-related elements, such as `<base>`, `<link>`, `<script>`, `<style>`, or `<title>`.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta)
    pub fn meta;
}

__element! {
    /// `<meter>`
    ///
    /// The **`<meter>`** HTML element represents either a scalar value within a known range or a fractional value.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter)
    pub fn meter;
}

__element! {
    /// `<nav>`
    ///
    /// The **`<nav>`** HTML element represents a section of a page whose purpose is to provide navigation links, either within the current document or to…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/nav)
    pub fn nav;
}

__element! {
    /// `<nobr>`
    ///
    /// The **`<nobr>`** HTML element prevents the text it contains from automatically wrapping across multiple lines, potentially resulting in the user having to scroll horizontally…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/nobr)
    pub fn nobr;
}

__element! {
    /// `<noembed>`
    ///
    /// The **`<noembed>`** HTML element is an obsolete, non-standard way to provide alternative, or "fallback", content for browsers that do not support the `<embed>` element…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/noembed)
    pub fn noembed;
}

__element! {
    /// `<noframes>`
    ///
    /// The **`<noframes>`** HTML element provides content to be presented in browsers that don't support (or have disabled support for) the `<frame>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/noframes)
    pub fn noframes;
}

__element! {
    /// `<noscript>`
    ///
    /// The **`<noscript>`** HTML element defines a section of HTML to be inserted if a script type on the page is unsupported or if scripting…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/noscript)
    pub fn noscript;
}

__element! {
    /// `<object>`
    ///
    /// The **`<object>`** HTML element represents an external resource, which can be treated as an image, a nested browsing context, or a resource to be…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object)
    pub fn object;
}

__element! {
    /// `<ol>`
    ///
    /// The **`<ol>`** HTML element represents an ordered list of items — typically rendered as a numbered list.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol)
    pub fn ol;
}

__element! {
    /// `<optgroup>`
    ///
    /// The **`<optgroup>`** HTML element creates a grouping of options within a `<select>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/optgroup)
    pub fn optgroup;
}

__element! {
    /// `<option>`
    ///
    /// The **`<option>`** HTML element is used to define an item contained in a `<select>`, an `<optgroup>`, or a `<datalist>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option)
    pub fn option;
}

__element! {
    /// `<output>`
    ///
    /// The **`<output>`** HTML element is a container element into which a site or app can inject the results of a calculation or the outcome…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/output)
    pub fn output;
}

__element! {
    /// `<p>`
    ///
    /// The **`<p>`** HTML element represents a paragraph.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/p)
    pub fn p;
}

__void! {
    /// `<param>`
    ///
    /// The **`<param>`** HTML element defines parameters for an `<object>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/param)
    pub fn param;
}

__element! {
    /// `<picture>`
    ///
    /// The **`<picture>`** HTML element contains zero or more `<source>` elements and one `<img>` element to offer alternative versions of an image for different display/device scenarios.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/picture)
    pub fn picture;
}

__element! {
    /// `<plaintext>`
    ///
    /// The **`<plaintext>`** HTML element renders everything following the start tag as raw text, ignoring any following HTML.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/plaintext)
    pub fn plaintext;
}

__element! {
    /// `<pre>`
    ///
    /// The **`<pre>`** HTML element represents preformatted text which is to be presented exactly as written in the HTML file.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/pre)
    pub fn pre;
}

__element! {
    /// `<progress>`
    ///
    /// The **`<progress>`** HTML element displays an indicator showing the completion progress of a task, typically displayed as a progress bar.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/progress)
    pub fn progress;
}

__element! {
    /// `<q>`
    ///
    /// The **`<q>`** HTML element indicates that the enclosed text is a short inline quotation.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q)
    pub fn q;
}

__element! {
    /// `<rb>`
    ///
    /// The **`<rb>`** HTML element is used to delimit the base text component of a annotation, i.e., the text that is being annotated.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rb)
    pub fn rb;
}

__element! {
    /// `<rp>`
    ///
    /// The **`<rp>`** HTML element is used to provide fall-back parentheses for browsers that do not support display of ruby annotations using the element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rp)
    pub fn rp;
}

__element! {
    /// `<rt>`
    ///
    /// The **`<rt>`** HTML element specifies the ruby text component of a ruby annotation, which is used to provide pronunciation, translation, or transliteration information for…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rt)
    pub fn rt;
}

__element! {
    /// `<rtc>`
    ///
    /// The **`<rtc>`** HTML element embraces semantic annotations of characters presented in a ruby of `<rb>` elements used inside of element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rtc)
    pub fn rtc;
}

__element! {
    /// `<ruby>`
    ///
    /// The **`<ruby>`** HTML element represents small annotations that are rendered above, below, or next to base text, usually used for showing the pronunciation of…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ruby)
    pub fn ruby;
}

__element! {
    /// `<s>`
    ///
    /// The **`<s>`** HTML element renders text with a strikethrough, or a line through it.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/s)
    pub fn s;
}

__element! {
    /// `<samp>`
    ///
    /// The **`<samp>`** HTML element is used to enclose inline text which represents sample (or quoted) output from a computer program.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/samp)
    pub fn samp;
}

__element! {
    /// `<script>`
    ///
    /// The **`<script>`** HTML element is used to embed executable code or data; this is typically used to embed or refer to JavaScript code.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script)
    pub fn script;
}

__element! {
    /// `<search>`
    ///
    /// The **`<search>`** HTML element is a container representing the parts of the document or application with form controls or other content related to performing…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/search)
    pub fn search;
}

__element! {
    /// `<section>`
    ///
    /// The **`<section>`** HTML element represents a generic standalone section of a document, which doesn't have a more specific semantic element to represent it.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/section)
    pub fn section;
}

__element! {
    /// `<select>`
    ///
    /// The **`<select>`** HTML element represents a control that provides a menu of options.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/select)
    pub fn select;
}

__element! {
    /// `<selectedcontent>`
    ///
    /// The **`<selectedcontent>`** HTML is used inside a `<select>` element to display the contents of its currently selected `<option>` within its first child `<button>`.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/selectedcontent)
    pub fn selectedcontent;
}

__element! {
    /// `<slot>`
    ///
    /// The **`<slot>`** HTML element is a placeholder inside a Web Component that you can fill with your own markup when the component is used.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/slot)
    pub fn slot;
}

__element! {
    /// `<small>`
    ///
    /// The **`<small>`** HTML element represents side-comments and small print, like copyright and legal text, independent of its styled presentation.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/small)
    pub fn small;
}

__void! {
    /// `<source>`
    ///
    /// The **`<source>`** HTML element specifies one or more media resources for the `<picture>`, `<audio>`, and `<video>` elements.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source)
    pub fn source;
}

__element! {
    /// `<span>`
    ///
    /// The **`<span>`** HTML element is a generic inline container for phrasing content, which does not inherently represent anything.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/span)
    pub fn span;
}

__element! {
    /// `<strike>`
    ///
    /// The **`<strike>`** HTML element places a strikethrough (horizontal line) over text.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/strike)
    pub fn strike;
}

__element! {
    /// `<strong>`
    ///
    /// The **`<strong>`** HTML element indicates that its contents have strong importance, seriousness, or urgency.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/strong)
    pub fn strong;
}

__element! {
    /// `<style>`
    ///
    /// The **`<style>`** HTML element contains style information for a document, or part of a document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/style)
    pub fn style;
}

__element! {
    /// `<sub>`
    ///
    /// The **`<sub>`** HTML element specifies inline text which should be displayed as subscript for solely typographical reasons.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sub)
    pub fn sub;
}

__element! {
    /// `<summary>`
    ///
    /// The **`<summary>`** HTML element specifies a summary, caption, or legend for a `<details>` element's disclosure box.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/summary)
    pub fn summary;
}

__element! {
    /// `<sup>`
    ///
    /// The **`<sup>`** HTML element specifies inline text which is to be displayed as superscript for solely typographical reasons.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sup)
    pub fn sup;
}

__element! {
    /// `<table>`
    ///
    /// The **`<table>`** HTML element represents tabular data—that is, information presented in a two-dimensional table comprised of rows and columns of cells containing data.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/table)
    pub fn table;
}

__element! {
    /// `<tbody>`
    ///
    /// The **`<tbody>`** HTML element encapsulates a set of table rows (`<tr>` elements), indicating that they comprise the body of a table's (main) data.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tbody)
    pub fn tbody;
}

__element! {
    /// `<td>`
    ///
    /// The **`<td>`** HTML element defines a cell of a table that contains data and may be used as a child of the `<tr>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td)
    pub fn td;
}

__element! {
    /// `<template>`
    ///
    /// The **`<template>`** HTML element serves as a mechanism for holding HTML fragments, which can either be used later via JavaScript, generated immediately and inserted…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template)
    pub fn template;
}

__element! {
    /// `<textarea>`
    ///
    /// The **`<textarea>`** HTML element represents a multi-line plain-text editing control, useful when you want to allow users to enter a sizeable amount of free-form…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea)
    pub fn textarea;
}

__element! {
    /// `<tfoot>`
    ///
    /// The **`<tfoot>`** HTML element encapsulates a set of table rows (`<tr>` elements), indicating that they comprise the foot of a table with information about…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tfoot)
    pub fn tfoot;
}

__element! {
    /// `<th>`
    ///
    /// The **`<th>`** HTML element defines a cell as the header of a group of table cells and may be used as a child of…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th)
    pub fn th;
}

__element! {
    /// `<thead>`
    ///
    /// The **`<thead>`** HTML element encapsulates a set of table rows (`<tr>` elements), indicating that they comprise the head of a table with information about…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/thead)
    pub fn thead;
}

__element! {
    /// `<time>`
    ///
    /// The **`<time>`** HTML element represents a specific period in time.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/time)
    pub fn time;
}

__element! {
    /// `<title>`
    ///
    /// The **`<title>`** HTML element defines the document's title that is shown in a Browser's title bar or a page's tab.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/title)
    pub fn title;
}

__element! {
    /// `<tr>`
    ///
    /// The **`<tr>`** HTML element defines a row of cells in a table.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tr)
    pub fn tr;
}

__void! {
    /// `<track>`
    ///
    /// The **`<track>`** HTML element is used as a child of the media elements, `<audio>` and `<video>`.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track)
    pub fn track;
}

__element! {
    /// `<tt>`
    ///
    /// The **`<tt>`** HTML element creates inline text which is presented using the user agent default monospace font face.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tt)
    pub fn tt;
}

__element! {
    /// `<u>`
    ///
    /// The **`<u>`** HTML element represents a span of inline text which should be rendered in a way that indicates that it has a non-textual annotation.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/u)
    pub fn u;
}

__element! {
    /// `<ul>`
    ///
    /// The **`<ul>`** HTML element represents an unordered list of items, typically rendered as a bulleted list.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ul)
    pub fn ul;
}

__element! {
    /// `<var>`
    ///
    /// The **`<var>`** HTML element represents the name of a variable in a mathematical expression or a programming context.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/var)
    pub fn var;
}

__element! {
    /// `<video>`
    ///
    /// The **`<video>`** HTML element embeds a media player which supports video playback into the document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video)
    pub fn video;
}

__void! {
    /// `<wbr>`
    ///
    /// The **`<wbr>`** HTML element represents a word break opportunity—a position within text where the browser may optionally break a line, though its line-breaking rules…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/wbr)
    pub fn wbr;
}

__element! {
    /// `<xmp>`
    ///
    /// The **`<xmp>`** HTML element renders text between the start and end tags without interpreting the HTML in between and using a monospaced font.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/xmp)
    pub fn xmp;
}

__element! {
    /// `<animate>`
    ///
    /// The **`<animate>`** SVG element provides a way to animate an attribute of an element over time.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/animate)
    pub fn animate;
}

__element! {
    /// `<animateMotion>`
    ///
    /// The **`<animateMotion>`** SVG element provides a way to define how an element moves along a motion path.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/animateMotion)
    pub fn animateMotion;
}

__element! {
    /// `<animateTransform>`
    ///
    /// The **`<animateTransform>`** SVG element animates a transformation attribute on its target element, thereby allowing animations to control translation, scaling, rotation, and/or skewing.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/animateTransform)
    pub fn animateTransform;
}

__element! {
    /// `<circle>`
    ///
    /// The **`<circle>`** SVG element is an SVG basic shape, used to draw circles based on a center point and a radius.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/circle)
    pub fn circle;
}

__element! {
    /// `<clipPath>`
    ///
    /// The **`<clipPath>`** SVG element defines a clipping path, to be used by the property.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/clipPath)
    pub fn clipPath;
}

__element! {
    /// `<defs>`
    ///
    /// The **`<defs>`** SVG element is used to store graphical objects that will be used at a later time.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/defs)
    pub fn defs;
}

__element! {
    /// `<desc>`
    ///
    /// The **`<desc>`** SVG element provides an accessible, long-text description of any SVG container element or graphics element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/desc)
    pub fn desc;
}

__element! {
    /// `<ellipse>`
    ///
    /// The **`<ellipse>`** SVG element is an SVG basic shape, used to create ellipses based on a center coordinate, and both their x and y radius.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/ellipse)
    pub fn ellipse;
}

__element! {
    /// `<feBlend>`
    ///
    /// The **`<feBlend>`** SVG filter primitive composes two objects together ruled by a certain blending mode.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feBlend)
    pub fn feBlend;
}

__element! {
    /// `<feColorMatrix>`
    ///
    /// The **`<feColorMatrix>`** SVG filter element changes colors based on a transformation matrix.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feColorMatrix)
    pub fn feColorMatrix;
}

__element! {
    /// `<feComponentTransfer>`
    ///
    /// The **`<feComponentTransfer>`** SVG filter primitive performs color-component-wise remapping of data for each pixel.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feComponentTransfer)
    pub fn feComponentTransfer;
}

__element! {
    /// `<feComposite>`
    ///
    /// The **`<feComposite>`** SVG filter primitive performs the combination of two input images pixel-wise in image space using one of the Porter-Duff compositing operations: `over`…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feComposite)
    pub fn feComposite;
}

__element! {
    /// `<feConvolveMatrix>`
    ///
    /// The **`<feConvolveMatrix>`** SVG filter primitive applies a matrix convolution filter effect.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feConvolveMatrix)
    pub fn feConvolveMatrix;
}

__element! {
    /// `<feDiffuseLighting>`
    ///
    /// The **`<feDiffuseLighting>`** SVG filter primitive lights an image using the alpha channel as a bump map.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feDiffuseLighting)
    pub fn feDiffuseLighting;
}

__element! {
    /// `<feDisplacementMap>`
    ///
    /// The **`<feDisplacementMap>`** SVG filter primitive uses the pixel values from the image from to spatially displace the image from .
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feDisplacementMap)
    pub fn feDisplacementMap;
}

__element! {
    /// `<feDistantLight>`
    ///
    /// The **`<feDistantLight>`** SVG element defines a distant light source that can be used within a lighting filter primitive: `<feDiffuseLighting>` or `<feSpecularLighting>`.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feDistantLight)
    pub fn feDistantLight;
}

__element! {
    /// `<feDropShadow>`
    ///
    /// The **`<feDropShadow>`** SVG filter primitive creates a drop shadow of the input image.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feDropShadow)
    pub fn feDropShadow;
}

__element! {
    /// `<feFlood>`
    ///
    /// The **`<feFlood>`** SVG filter primitive fills the filter subregion with the color and opacity defined by and .
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feFlood)
    pub fn feFlood;
}

__element! {
    /// `<feFuncA>`
    ///
    /// The **`<feFuncA>`** SVG filter primitive defines the transfer function for the alpha component of the input graphic of its parent `<feComponentTransfer>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feFuncA)
    pub fn feFuncA;
}

__element! {
    /// `<feFuncB>`
    ///
    /// The **`<feFuncB>`** SVG filter primitive defines the transfer function for the blue component of the input graphic of its parent `<feComponentTransfer>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feFuncB)
    pub fn feFuncB;
}

__element! {
    /// `<feFuncG>`
    ///
    /// The **`<feFuncG>`** SVG filter primitive defines the transfer function for the green component of the input graphic of its parent `<feComponentTransfer>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feFuncG)
    pub fn feFuncG;
}

__element! {
    /// `<feFuncR>`
    ///
    /// The **`<feFuncR>`** SVG filter primitive defines the transfer function for the red component of the input graphic of its parent `<feComponentTransfer>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feFuncR)
    pub fn feFuncR;
}

__element! {
    /// `<feGaussianBlur>`
    ///
    /// The **`<feGaussianBlur>`** SVG filter primitive blurs the input image by the amount specified in , which defines the bell-curve.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feGaussianBlur)
    pub fn feGaussianBlur;
}

__element! {
    /// `<feImage>`
    ///
    /// The **`<feImage>`** SVG filter primitive fetches image data from an external source and provides the pixel data as output (meaning if the external source…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feImage)
    pub fn feImage;
}

__element! {
    /// `<feMerge>`
    ///
    /// The **`<feMerge>`** SVG element allows filter effects to be applied concurrently instead of sequentially.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feMerge)
    pub fn feMerge;
}

__element! {
    /// `<feMergeNode>`
    ///
    /// The **`<feMergeNode>`** SVG takes the result of another filter to be processed by its parent .
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feMergeNode)
    pub fn feMergeNode;
}

__element! {
    /// `<feMorphology>`
    ///
    /// The **`<feMorphology>`** SVG filter primitive is used to erode or dilate the input image.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feMorphology)
    pub fn feMorphology;
}

__element! {
    /// `<feOffset>`
    ///
    /// The **`<feOffset>`** SVG filter primitive enables offsetting an input image relative to its current position.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feOffset)
    pub fn feOffset;
}

__element! {
    /// `<fePointLight>`
    ///
    /// The **`<fePointLight>`** SVG element defines a light source which allows you to create a point light effect.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/fePointLight)
    pub fn fePointLight;
}

__element! {
    /// `<feSpecularLighting>`
    ///
    /// The **`<feSpecularLighting>`** SVG filter primitive lights a source graphic using the alpha channel as a bump map.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feSpecularLighting)
    pub fn feSpecularLighting;
}

__element! {
    /// `<feSpotLight>`
    ///
    /// The **`<feSpotLight>`** SVG element defines a light source that can be used to create a spotlight effect.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feSpotLight)
    pub fn feSpotLight;
}

__element! {
    /// `<feTile>`
    ///
    /// The **`<feTile>`** SVG filter primitive allows you to fill a target rectangle with a repeated, tiled pattern of an input image.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feTile)
    pub fn feTile;
}

__element! {
    /// `<feTurbulence>`
    ///
    /// The **`<feTurbulence>`** SVG filter primitive creates an image using the Perlin turbulence function.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feTurbulence)
    pub fn feTurbulence;
}

__element! {
    /// `<filter>`
    ///
    /// The **`<filter>`** SVG element defines a custom filter effect by grouping atomic filter primitives.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/filter)
    pub fn filter;
}

__element! {
    /// `<foreignObject>`
    ///
    /// The **`<foreignObject>`** SVG element includes elements from a different XML namespace.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/foreignObject)
    pub fn foreignObject;
}

__element! {
    /// `<g>`
    ///
    /// The **`<g>`** SVG element is a container used to group other SVG elements.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/g)
    pub fn g;
}

__element! {
    /// `<image>`
    ///
    /// The **`<image>`** SVG element includes images inside SVG documents.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/image)
    pub fn image;
}

__element! {
    /// `<line>`
    ///
    /// The **`<line>`** SVG element is an SVG basic shape used to create a line connecting two points.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/line)
    pub fn line;
}

__element! {
    /// `<linearGradient>`
    ///
    /// The **`<linearGradient>`** SVG element lets authors define linear gradients to apply to other SVG elements.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/linearGradient)
    pub fn linearGradient;
}

__element! {
    /// `<marker>`
    ///
    /// The **`<marker>`** SVG element defines a graphic used for drawing arrowheads or polymarkers on a given `<path>`, `<line>`, `<polyline>` or `<polygon>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/marker)
    pub fn marker;
}

__element! {
    /// `<mask>`
    ///
    /// The **`<mask>`** SVG element defines a mask for compositing the current object into the background.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/mask)
    pub fn mask;
}

__element! {
    /// `<metadata>`
    ///
    /// The **`<metadata>`** SVG element adds metadata to SVG content.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/metadata)
    pub fn metadata;
}

__element! {
    /// `<mpath>`
    ///
    /// The **`<mpath>`** SVG sub-element for the `<animateMotion>` element provides the ability to reference an external `<path>` element as the definition of a motion path.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/mpath)
    pub fn mpath;
}

__element! {
    /// `<path>`
    ///
    /// The **`<path>`** SVG element is the generic element to define a shape.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/path)
    pub fn path;
}

__element! {
    /// `<pattern>`
    ///
    /// The **`<pattern>`** SVG element defines a graphics object which can be redrawn at repeated x- and y-coordinate intervals ("tiled") to cover an area.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/pattern)
    pub fn pattern;
}

__element! {
    /// `<polygon>`
    ///
    /// The **`<polygon>`** SVG element defines a closed shape consisting of a set of connected straight line segments.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/polygon)
    pub fn polygon;
}

__element! {
    /// `<polyline>`
    ///
    /// The **`<polyline>`** SVG element is an SVG basic shape that creates straight lines connecting several points.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/polyline)
    pub fn polyline;
}

__element! {
    /// `<radialGradient>`
    ///
    /// The **`<radialGradient>`** SVG element lets authors define radial gradients that can be applied to fill or stroke of graphical elements.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/radialGradient)
    pub fn radialGradient;
}

__element! {
    /// `<rect>`
    ///
    /// The **`<rect>`** SVG element is a basic SVG shape that draws rectangles, defined by their position, width, and height.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/rect)
    pub fn rect;
}

__element! {
    /// `<set>`
    ///
    /// The **`<set>`** SVG element provides a method of setting the value of an attribute for a specified duration.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/set)
    pub fn set;
}

__element! {
    /// `<stop>`
    ///
    /// The **`<stop>`** SVG element defines a color and its position to use on a gradient.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/stop)
    pub fn stop;
}

__element! {
    /// `<svg>`
    ///
    /// The **`<svg>`** SVG element is a container that defines a new coordinate system and viewport.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/svg)
    pub fn svg;
}

__element! {
    /// `<switch>`
    ///
    /// The **`<switch>`** SVG element evaluates any and attributes on its direct child elements in order, and then renders the first child where these attributes…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/switch)
    pub fn switch;
}

__element! {
    /// `<symbol>`
    ///
    /// The **`<symbol>`** SVG element is used to define graphical template objects which can be instantiated by a `<use>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/symbol)
    pub fn symbol;
}

__element! {
    /// `<text>`
    ///
    /// The **`<text>`** SVG element draws a graphics element consisting of text.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/text)
    pub fn text;
}

__element! {
    /// `<textPath>`
    ///
    /// The **`<textPath>`** SVG element is used to render text along the shape of a `<path>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/textPath)
    pub fn textPath;
}

__element! {
    /// `<tspan>`
    ///
    /// The **`<tspan>`** SVG element defines a subtext within a element or another `<tspan>` element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/tspan)
    pub fn tspan;
}

__element! {
    /// `<view>`
    ///
    /// The **`<view>`** SVG element defines a particular view of an SVG document.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/view)
    pub fn view;
}

__element! {
    /// `<annotation>`
    ///
    /// The **`<annotation>`** MathML element contains an annotation to the MathML expression in a textual format, for example LaTeX.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/annotation)
    pub fn annotation;
}

__element! {
    /// `<maction>`
    ///
    /// The **`<maction>`** MathML element allows you to bind actions to mathematical expressions.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/maction)
    pub fn maction;
}

__element! {
    /// `<math>`
    ///
    /// The **`<math>`** MathML element is the top-level MathML element, used to write a single mathematical formula.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/math)
    pub fn math;
}

__element! {
    /// `<menclose>`
    ///
    /// The **`<menclose>`** MathML element renders its content inside an enclosing notation specified by the `notation` attribute.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/menclose)
    pub fn menclose;
}

__element! {
    /// `<merror>`
    ///
    /// The **`<merror>`** MathML element is used to display contents as error messages.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/merror)
    pub fn merror;
}

__element! {
    /// `<mfenced>`
    ///
    /// The **`<mfenced>`** MathML element provides the possibility to add custom opening and closing brackets (such as parentheses) and separators (such as commas or semicolons)…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mfenced)
    pub fn mfenced;
}

__element! {
    /// `<mfrac>`
    ///
    /// The **`<mfrac>`** MathML element is used to display fractions.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mfrac)
    pub fn mfrac;
}

__element! {
    /// `<mi>`
    ///
    /// The **`<mi>`** MathML element indicates that the content should be rendered as an **identifier**, such as a function name, variable or symbolic constant.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mi)
    pub fn mi;
}

__element! {
    /// `<mmultiscripts>`
    ///
    /// The **`<mmultiscripts>`** MathML element is used to attach an arbitrary number of subscripts and superscripts to an expression at once, generalizing the element.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mmultiscripts)
    pub fn mmultiscripts;
}

__element! {
    /// `<mn>`
    ///
    /// The **`<mn>`** MathML element represents a **numeric** literal which is normally a sequence of digits with a possible separator (a dot or a comma).
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mn)
    pub fn mn;
}

__element! {
    /// `<mo>`
    ///
    /// The **`<mo>`** MathML element represents an **operator** in a broad sense.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mo)
    pub fn mo;
}

__element! {
    /// `<mover>`
    ///
    /// The **`<mover>`** MathML element is used to attach an accent or a limit over an expression.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mover)
    pub fn mover;
}

__element! {
    /// `<mpadded>`
    ///
    /// The **`<mpadded>`** MathML element is used to add extra padding and to set the general adjustment of position and size of enclosed contents.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mpadded)
    pub fn mpadded;
}

__element! {
    /// `<mphantom>`
    ///
    /// The **`<mphantom>`** MathML element is rendered invisibly, but dimensions (such as height, width, and baseline position) are still kept.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mphantom)
    pub fn mphantom;
}

__element! {
    /// `<mprescripts>`
    ///
    /// The **`<mprescripts>`** MathML element is used within an element to indicate the start of the pre-scripts elements (subscripts and superscripts that are placed **before**…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mprescripts)
    pub fn mprescripts;
}

__element! {
    /// `<mroot>`
    ///
    /// The **`<mroot>`** MathML element is used to display roots with an explicit index.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mroot)
    pub fn mroot;
}

__element! {
    /// `<mrow>`
    ///
    /// The **`<mrow>`** MathML element is used to group sub-expressions, which usually contain one or more operators with their respective operands (such as and ).
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mrow)
    pub fn mrow;
}

__element! {
    /// `<ms>`
    ///
    /// The **`<ms>`** MathML element represents a **string** literal meant to be interpreted by programming languages and computer algebra systems.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/ms)
    pub fn ms;
}

__element! {
    /// `<mspace>`
    ///
    /// The **`<mspace>`** MathML element is used to display a blank space, whose size is set by its attributes.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mspace)
    pub fn mspace;
}

__element! {
    /// `<msqrt>`
    ///
    /// The **`<msqrt>`** MathML element is used to display square roots (no index is displayed).
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/msqrt)
    pub fn msqrt;
}

__element! {
    /// `<mstyle>`
    ///
    /// The **`<mstyle>`** MathML element is used to change the style of its children.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mstyle)
    pub fn mstyle;
}

__element! {
    /// `<msub>`
    ///
    /// The **`<msub>`** MathML element is used to attach a subscript to an expression.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/msub)
    pub fn msub;
}

__element! {
    /// `<msubsup>`
    ///
    /// The **`<msubsup>`** MathML element is used to attach both a subscript and a superscript, together, to an expression.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/msubsup)
    pub fn msubsup;
}

__element! {
    /// `<msup>`
    ///
    /// The **`<msup>`** MathML element is used to attach a superscript to an expression.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/msup)
    pub fn msup;
}

__element! {
    /// `<mtable>`
    ///
    /// The **`<mtable>`** MathML element allows you to create tables or matrices.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mtable)
    pub fn mtable;
}

__element! {
    /// `<mtd>`
    ///
    /// The **`<mtd>`** MathML element represents a cell in a table or a matrix.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mtd)
    pub fn mtd;
}

__element! {
    /// `<mtext>`
    ///
    /// The **`<mtext>`** MathML element is used to render arbitrary text with _no_ notational meaning, such as comments or annotations.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mtext)
    pub fn mtext;
}

__element! {
    /// `<mtr>`
    ///
    /// The **`<mtr>`** MathML element represents a row in a table or a matrix.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/mtr)
    pub fn mtr;
}

__element! {
    /// `<munder>`
    ///
    /// The **`<munder>`** MathML element is used to attach an accent or a limit under an expression.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/munder)
    pub fn munder;
}

__element! {
    /// `<munderover>`
    ///
    /// The **`<munderover>`** MathML element is used to attach accents or limits both under and over an expression.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/munderover)
    pub fn munderover;
}

__element! {
    /// `<semantics>`
    ///
    /// The **`<semantics>`** MathML element associates annotations with a MathML expression, for example its text source as a lightweight markup language or mathematical meaning expressed…
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/semantics)
    pub fn semantics;
}
