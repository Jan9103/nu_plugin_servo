pub mod commands;
// pub mod custom_values;
pub mod plugin_interface;

use html5ever::{QualName, tendril::TendrilSink};
use nu_protocol::{Record, Span, Value};
use scraper::ElementRef;
use xml5ever::driver::XmlParseOpts;

pub fn parse_html<R>(document: &mut R) -> std::result::Result<scraper::Html, std::io::Error>
where
    R: std::io::Read,
{
    html5ever::parse_document(
        scraper::HtmlTreeSink::new(scraper::Html::new_document()),
        html5ever::ParseOpts {
            tree_builder: html5ever::tree_builder::TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .from_utf8()
    .read_from(document)
}
pub fn parse_xml<R>(document: &mut R) -> std::result::Result<scraper::Html, std::io::Error>
where
    R: std::io::Read,
{
    xml5ever::driver::parse_document(
        scraper::HtmlTreeSink::new(scraper::Html::new_document()),
        XmlParseOpts::default(),
    )
    .from_utf8()
    .read_from(document)
}

//pub fn parse_css(raw_css: &str) {
//    let mut parser_input = cssparser::ParserInput::new(raw_css);
//    let mut parser = cssparser::Parser::new(&mut parser_input);
//    parser.try_parse(thing)
//}

fn format_qual_name(qn: &QualName) -> String {
    if let Some(p) = &qn.prefix {
        let mut out = String::new();
        out.push_str(p);
        out.push(':');
        out.push_str(&qn.local);
        out
    } else {
        qn.local.to_string()
    }
}

pub fn html_element_to_nu(span: Span, element: ElementRef<'_>) -> Value {
    let mut out = Record::new();
    out.push(
        "tag",
        Value::string(format_qual_name(&element.value().name), span),
    );
    let mut attributes = Record::new();
    for attr in element.value().attrs() {
        if attr.0 == "id" || attr.0 == "classes" {
            continue;
        }
        attributes.push(attr.0, Value::string(attr.1, span));
    }
    out.push("attributes", Value::record(attributes, span));
    out.push(
        "id",
        if let Some(id) = element.value().id() {
            Value::string(id, span)
        } else {
            Value::nothing(span)
        },
    );
    out.push(
        "classes",
        Value::list(
            element
                .value()
                .classes()
                .map(|c| -> Value { Value::string(c, span) })
                .collect::<Vec<Value>>(),
            span,
        ),
    );
    out.push(
        "content",
        Value::list(
            element
                .children()
                .filter_map(|child| -> Option<Value> {
                    match child.value() {
                        scraper::Node::Document => None,
                        scraper::Node::Fragment => None,
                        scraper::Node::Doctype(_doctype) => None,
                        scraper::Node::Comment(_comment) => None, // TODO
                        scraper::Node::Text(text) => Some(Value::string(&text.text, span)),
                        scraper::Node::Element(_element) => {
                            // let a = ElementRef::wrap(child);
                            Some(html_element_to_nu(
                                span,
                                ElementRef::wrap(child)
                                    .expect("child of type Element is not of type Element"),
                            ))
                            // todo!()
                        }
                        scraper::Node::ProcessingInstruction(_processing_instruction) => None,
                    }
                })
                .collect::<Vec<Value>>(),
            span,
        ),
    );
    Value::record(out, span)
}
pub fn xml_element_to_nu(span: Span, element: ElementRef<'_>, text_as_elements: bool) -> Value {
    let mut out = Record::new();
    out.push(
        "tag",
        Value::string(format_qual_name(&element.value().name), span),
    );
    let mut attributes = Record::new();
    for attr in element.value().attrs() {
        attributes.push(attr.0, Value::string(attr.1, span));
    }
    out.push("attributes", Value::record(attributes, span));
    out.push(
        "content",
        Value::list(
            element
                .children()
                .filter_map(|child| -> Option<Value> {
                    match child.value() {
                        scraper::Node::Document => None,
                        scraper::Node::Fragment => None,
                        scraper::Node::Doctype(_doctype) => None,
                        scraper::Node::Comment(_comment) => None, // TODO
                        scraper::Node::Text(text) => {
                            if text_as_elements {
                                let mut r = Record::new();
                                r.push("tag", Value::nothing(span));
                                r.push("attributes", Value::nothing(span));
                                r.push("content", Value::string(&text.text, span));
                                Some(Value::record(r, span))
                            } else {
                                Some(Value::string(&text.text, span))
                            }
                        }
                        scraper::Node::Element(_element) => {
                            // let a = ElementRef::wrap(child);
                            Some(xml_element_to_nu(
                                span,
                                ElementRef::wrap(child)
                                    .expect("child of type Element is not of type Element"),
                                text_as_elements,
                            ))
                            // todo!()
                        }
                        scraper::Node::ProcessingInstruction(_processing_instruction) => None,
                    }
                })
                .collect::<Vec<Value>>(),
            span,
        ),
    );
    Value::record(out, span)
}
