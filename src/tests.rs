use std::{cell::Cell, rc::Rc, sync::Arc};

use crate::{Attributes, Attrs, Children, Html, Props, Raw, asx};

fn assert_html(expected: &str, html: impl Html) {
    assert_eq!(html.to_string(), expected);
}

#[test]
fn literals_are_escaped() {
    assert_html(
        "&lt;&amp;&quot;quoted &amp; &lt; &gt;42true",
        asx! {
            "<&\""
            @{"quoted & < >"}
            42
            true
        },
    );
}

#[test]
fn dynamic_expressions_are_escaped() {
    let value = String::from("<&\"");
    assert_html("&lt;&amp;&quot;", asx! { @{value} });
}

#[test]
fn injected_blocks_support_statements() {
    assert_html(
        "&lt;block&gt;",
        asx! {
            @{
                let left = "<";
                let right = ">";
                format!("{left}block{right}")
            }
        },
    );
}

#[test]
fn raw_strings_bypass_escaping() {
    assert_html(
        "&lt;safe&gt;<strong>trusted</strong>",
        asx! {
            "<safe>"
            @{Raw("<strong>trusted</strong>")}
        },
    );
}

#[test]
fn built_in_and_literal_elements_render_in_order() {
    assert_html(
        concat!(
            "<div><span>text</span><br></div>",
            "<input type=\"text\">",
            "<my-widget data-id=\"7\">body</my-widget>",
            "<my-void>"
        ),
        asx! {
            div {
                span { "text" }
                br;
            }
            input[type: "text"];
            "my-widget"[{"data-id"}: {7}] { "body" }
            "my-void";
        },
    );
}

#[test]
fn built_in_elements_ignore_local_bindings() {
    let html = "local value";

    assert_html("<html><body></body></html>", asx! { html { body {} } });
    assert_eq!(html, "local value");
}

#[test]
fn attributes_support_static_dynamic_and_quoted_forms() {
    let dynamic_title = String::from("<&\"");
    let attribute_name = String::from("data-dynamic");

    assert_html(
        concat!(
            "<div class=\"one two&lt;&amp;\" ",
            "style=\"color: red;display: block;\" ",
            "id=\"root\" data-static=\"&lt;&amp;&quot;&#39;\" ",
            "title=\"&lt;&amp;&quot;\" data-dynamic=\"value\" ",
            "hidden=\"true\"></div>"
        ),
        asx! {
            div[
                id: "root",
                class: "one",
                class: {"two<&"},
                style: "color: red",
                {"style"}: {"display: block"},
                {"data-static"}: "<&\"\x27",
                title: {dynamic_title},
                {attribute_name}: {"value"},
                hidden: {true},
                disabled: {false},
                omitted: {None::<&str>}
            ] {}
        },
    );
}

#[test]
fn attribute_projection_merges_class_style_and_other_attributes() {
    let projected = crate::__attrs!((), {
        id: "projected",
        class: "base",
        style: "color: red"
    })
    .1;

    assert_html(
        concat!(
            "<div class=\"local base\" ",
            "style=\"display: block;color: red;\" ",
            "id=\"projected\"></div>"
        ),
        asx! {
            div[
                class: "local",
                style: "display: block",
                ..projected
            ] {}
        },
    );
}

#[test]
fn projected_attributes_preserve_presence_rules() {
    let projected = crate::__attrs!((), {
        class: {Some("projected")},
        style: {None::<&str>},
        title: {Some("<&")},
        hidden: {false}
    })
    .1;

    assert_html(
        "<div class=\"local projected\" title=\"&lt;&amp;\"></div>",
        asx! { div[class: "local", ..projected] {} },
    );
}

#[test]
fn if_and_else_render_only_the_selected_branch() {
    let enabled = true;
    assert_html("yes", asx! { @if enabled { "yes" } else { "no" } });

    let enabled = false;
    assert_html("no", asx! { @if enabled { "yes" } else { "no" } });
}

#[test]
fn for_loops_render_each_iteration() {
    assert_html(
        "<li>1</li><li>2</li><li>3</li>",
        asx! {
            @for item in [1, 2, 3] {
                li { @{item} }
            }
        },
    );
}

#[test]
fn while_loops_can_mutate_captured_state() {
    let mut index = 0;
    assert_html(
        "012",
        asx! {
            @while index < 3 {
                @{index}
                @({ index += 1; })
            }
        },
    );
}

#[test]
fn loop_bodies_can_break_from_injected_rust() {
    let mut index = 0;
    assert_html(
        "012",
        asx! {
            @loop {
                @({
                    if index == 3 {
                        break;
                    }
                    let current = index;
                    index += 1;
                    current
                })
            }
        },
    );
}

#[test]
fn let_bindings_are_available_to_following_syntax() {
    assert_html(
        "&lt;local&gt;",
        asx! {
            @let value = String::from("<local>");
            @{value}
        },
    );
}

#[test]
fn match_supports_patterns_and_guards() {
    let value = Some(4);
    assert_html(
        "large:4",
        asx! {
            @match value {
                Some(number) if number > 3 => { "large:" @{number} },
                Some(number) => { "small:" @{number} },
                None => { "none" },
            }
        },
    );
}

#[test]
fn asx_is_lazy_until_rendered() {
    let called = Rc::new(Cell::new(false));
    let probe = Rc::clone(&called);
    let html = asx! {
        @({
            probe.set(true);
            "rendered"
        })
    };

    assert!(!called.get());
    assert_html("rendered", html);
    assert!(called.get());
}

#[test]
fn primitive_html_implementations_render_consistently() {
    assert_html("true", true);
    assert_html("false", false);
    assert_html("-12", -12_i32);
    assert_html("1.5", 1.5_f64);
    assert_html("&lt;", char::from_u32(60).unwrap());
    assert_html("&lt;str&gt;", "<str>");
    assert_html("&lt;string&gt;", String::from("<string>"));
    assert_html("&lt;box&gt;", Box::<str>::from("<box>"));
    assert_html("&lt;rc&gt;", Rc::<str>::from("<rc>"));
    assert_html("&lt;arc&gt;", Arc::<str>::from("<arc>"));
    assert_html("&lt;fmt&gt;", format_args!("<{}>", "fmt"));
    assert_html("&lt;some&gt;", Some("<some>"));
    assert_html("", None::<&str>);
}

#[test]
fn write_appends_to_an_existing_buffer() {
    let mut output = String::from("prefix:");
    asx! { "<&" }.write(&mut output);
    assert_eq!(output, "prefix:&lt;&amp;");
}

mod components {
    use super::*;

    #[derive(Default)]
    pub struct ComponentProps {
        pub label: String,
        pub enabled: bool,
    }

    fn render<A, C>(
        props: Props<ComponentProps>,
        attrs: Attrs<A>,
        children: Children<C>,
    ) -> impl Html
    where
        A: Attributes,
        C: Html,
    {
        let component_label = props.0.label;
        let children = children.0;

        asx! {
            "component"[..attrs] {
                @{component_label}

                @{children}
            }
        }
    }

    pub fn zero() -> impl Html {
        render(Props::default(), Attrs(()), Children(()))
    }

    pub fn p(props: Props<ComponentProps>) -> impl Html {
        render(props, Attrs(()), Children(()))
    }

    pub fn a(attrs: Attrs<impl Attributes>) -> impl Html {
        render(Props::default(), attrs, Children(()))
    }

    pub fn c(children: Children<impl Html>) -> impl Html {
        render(Props::default(), Attrs(()), children)
    }

    pub fn pa(props: Props<ComponentProps>, attrs: Attrs<impl Attributes>) -> impl Html {
        render(props, attrs, Children(()))
    }

    pub fn ap(attrs: Attrs<impl Attributes>, props: Props<ComponentProps>) -> impl Html {
        render(props, attrs, Children(()))
    }

    pub fn pc(props: Props<ComponentProps>, children: Children<impl Html>) -> impl Html {
        render(props, Attrs(()), children)
    }

    pub fn cp(children: Children<impl Html>, props: Props<ComponentProps>) -> impl Html {
        render(props, Attrs(()), children)
    }

    pub fn ac(attrs: Attrs<impl Attributes>, children: Children<impl Html>) -> impl Html {
        render(Props::default(), attrs, children)
    }

    pub fn ca(children: Children<impl Html>, attrs: Attrs<impl Attributes>) -> impl Html {
        render(Props::default(), attrs, children)
    }

    pub fn pac(
        props: Props<ComponentProps>,
        attrs: Attrs<impl Attributes>,
        children: Children<impl Html>,
    ) -> impl Html {
        render(props, attrs, children)
    }

    pub fn pca(
        props: Props<ComponentProps>,
        children: Children<impl Html>,
        attrs: Attrs<impl Attributes>,
    ) -> impl Html {
        render(props, attrs, children)
    }

    pub fn apc(
        attrs: Attrs<impl Attributes>,
        props: Props<ComponentProps>,
        children: Children<impl Html>,
    ) -> impl Html {
        render(props, attrs, children)
    }

    pub fn acp(
        attrs: Attrs<impl Attributes>,
        children: Children<impl Html>,
        props: Props<ComponentProps>,
    ) -> impl Html {
        render(props, attrs, children)
    }

    pub fn cpa(
        children: Children<impl Html>,
        props: Props<ComponentProps>,
        attrs: Attrs<impl Attributes>,
    ) -> impl Html {
        render(props, attrs, children)
    }

    pub fn cap(
        children: Children<impl Html>,
        attrs: Attrs<impl Attributes>,
        props: Props<ComponentProps>,
    ) -> impl Html {
        render(props, attrs, children)
    }

    pub fn property_literal(props: Props<ComponentProps>) -> impl Html {
        props.0.enabled
    }
}

#[test]
fn components_support_every_argument_combination_and_order() {
    assert_html("<component></component>", asx! { @components::zero; });
    assert_html(
        "<component>P</component>",
        asx! { @components::p[.label: {String::from("P")}]; },
    );
    assert_html(
        "<component id=\"a\"></component>",
        asx! { @components::a[id: "a"]; },
    );
    assert_html("<component>C</component>", asx! { @components::c { "C" } });

    assert_html(
        "<component id=\"a\">P</component>",
        asx! { @components::pa[.label: {String::from("P")}, id: "a"]; },
    );
    assert_html(
        "<component id=\"a\">P</component>",
        asx! { @components::ap[.label: {String::from("P")}, id: "a"]; },
    );
    assert_html(
        "<component>PC</component>",
        asx! { @components::pc[.label: {String::from("P")}] { "C" } },
    );
    assert_html(
        "<component>PC</component>",
        asx! { @components::cp[.label: {String::from("P")}] { "C" } },
    );
    assert_html(
        "<component id=\"a\">C</component>",
        asx! { @components::ac[id: "a"] { "C" } },
    );
    assert_html(
        "<component id=\"a\">C</component>",
        asx! { @components::ca[id: "a"] { "C" } },
    );

    assert_html(
        "<component id=\"a\">PC</component>",
        asx! { @components::pac[.label: {String::from("P")}, id: "a"] { "C" } },
    );
    assert_html(
        "<component id=\"a\">PC</component>",
        asx! { @components::pca[.label: {String::from("P")}, id: "a"] { "C" } },
    );
    assert_html(
        "<component id=\"a\">PC</component>",
        asx! { @components::apc[.label: {String::from("P")}, id: "a"] { "C" } },
    );
    assert_html(
        "<component id=\"a\">PC</component>",
        asx! { @components::acp[.label: {String::from("P")}, id: "a"] { "C" } },
    );
    assert_html(
        "<component id=\"a\">PC</component>",
        asx! { @components::cpa[.label: {String::from("P")}, id: "a"] { "C" } },
    );
    assert_html(
        "<component id=\"a\">PC</component>",
        asx! { @components::cap[.label: {String::from("P")}, id: "a"] { "C" } },
    );

    assert_html(
        "true",
        asx! { @components::property_literal[.enabled: true]; },
    );
}
