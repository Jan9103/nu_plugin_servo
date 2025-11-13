use html5ever::tendril::TendrilSink;
use nu_protocol::{LabeledError, Record, Span, Value};
use scraper::{ElementRef, Html, Selector};

use crate::{HtmlBackend, format_qual_name};

#[derive(Copy, Clone)]
pub struct ScraperBackend;

impl ScraperBackend {
    #[cfg(feature = "xml")]
    pub fn parse_xml(&self, document: &Value) -> Result<Html, nu_protocol::LabeledError> {
        let mut document: &[u8] = match document {
            Value::String { val, .. } => val.as_bytes(),
            Value::Binary { val, .. } => val,
            _ => {
                return Err(LabeledError::new("Input type neither string nor binary"));
            }
        };
        xml5ever::driver::parse_document(
            scraper::HtmlTreeSink::new(scraper::Html::new_document()),
            xml5ever::driver::XmlParseOpts::default(),
        )
        .from_utf8()
        .read_from(&mut document)
        .map_err(|err| LabeledError::new(format!("Failed to parse html: {err}")))
    }
}

impl HtmlBackend for ScraperBackend {
    type Document = Html;
    type Node<'a> = ElementRef<'a>;

    fn parse(&self, document: &Value) -> Result<Self::Document, nu_protocol::LabeledError> {
        let mut document: &[u8] = match document {
            Value::String { val, .. } => val.as_bytes(),
            Value::Binary { val, .. } => val,
            _ => {
                return Err(LabeledError::new("Input type neither string nor binary"));
            }
        };
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
        .read_from(&mut document)
        .map_err(|err| LabeledError::new(format!("Failed to parse html: {err}")))
    }

    fn get_root_node<'a>(
        &self,
        html: &'a Self::Document,
    ) -> Result<Self::Node<'a>, nu_protocol::LabeledError> {
        Ok(html.root_element())
    }

    fn css_query<'a>(
        &self,
        html: &'a Self::Document,
        selector: &str,
    ) -> Result<Vec<Self::Node<'a>>, nu_protocol::LabeledError> {
        let selector: Selector = Selector::parse(selector)
            .map_err(|err| LabeledError::new(format!("Failed to parse CSS: {err}")))?;
        Ok(html.select(&selector).collect())
    }

    fn inner_html(
        &self,
        _html: &Self::Document,
        node: Self::Node<'_>,
    ) -> Result<String, nu_protocol::LabeledError> {
        Ok(node.inner_html())
    }

    fn outer_html(
        &self,
        _html: &Self::Document,
        node: Self::Node<'_>,
    ) -> Result<String, nu_protocol::LabeledError> {
        Ok(node.html())
    }

    fn node2nu(
        &self,
        html: &Self::Document,
        node: Self::Node<'_>,
        format: crate::NuDataFormat,
        span: Span,
    ) -> Result<Value, LabeledError> {
        Ok(match format {
            crate::NuDataFormat::Html => node2html_nu(span, node),
            crate::NuDataFormat::FromXmlCompat => xml_element_to_nu(span, node, true),
            crate::NuDataFormat::Xml => xml_element_to_nu(span, node, false),
            crate::NuDataFormat::InnerHtml => Value::string(self.inner_html(html, node)?, span),
            crate::NuDataFormat::OuterHtml => Value::string(self.outer_html(html, node)?, span),
        })
    }
}

fn node2html_nu(span: Span, element: ElementRef<'_>) -> Value {
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
                            Some(node2html_nu(
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

fn xml_element_to_nu(span: Span, element: ElementRef<'_>, text_as_elements: bool) -> Value {
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
