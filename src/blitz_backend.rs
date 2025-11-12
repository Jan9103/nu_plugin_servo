use blitz_dom::{DocumentConfig, Node};
use blitz_html::HtmlDocument;
use nu_protocol::{LabeledError, Record, Span, Value};

use crate::{HtmlBackend, format_qual_name};

#[derive(Copy, Clone)]
pub struct BlitzBackend;

impl HtmlBackend for BlitzBackend {
    type Document = HtmlDocument;
    type Node<'a> = &'a blitz_dom::Node;

    fn parse(&self, html: &Value) -> Result<Self::Document, nu_protocol::LabeledError> {
        let html: &String = match html {
            Value::String { val, .. } => val,
            _ => {
                return Err(LabeledError::new("Input type neither string nor binary"));
            }
        };
        Ok(HtmlDocument::from_html(html, DocumentConfig::default()))
    }

    fn get_root_node<'a>(
        &self,
        html: &'a Self::Document,
    ) -> Result<Self::Node<'a>, nu_protocol::LabeledError> {
        Ok(html.root_node())
    }

    fn css_query<'a>(
        &self,
        html: &'a Self::Document,
        selector: &str,
    ) -> Result<Vec<Self::Node<'a>>, nu_protocol::LabeledError> {
        match html.query_selector_all(selector) {
            Ok(sv) => Ok(sv
                .iter()
                .map(|i| html.get_node(*i).unwrap())
                .collect::<Vec<&'a Node>>()),
            Err(err) => Err(LabeledError::new(format!("Invalid html: {err:?}"))),
        }
    }

    fn inner_html(
        &self,
        html: &Self::Document,
        node: Self::Node<'_>,
    ) -> Result<String, nu_protocol::LabeledError> {
        if node.is_text_node() {
            return Ok(node.text_content());
        }

        let mut out = String::new();
        for child_node_id in node.children.iter() {
            let child_node = html.get_node(*child_node_id).unwrap();
            out.push_str(&child_node.outer_html());
        }

        Ok(out)
    }

    fn outer_html(
        &self,
        _html: &Self::Document,
        node: Self::Node<'_>,
    ) -> Result<String, nu_protocol::LabeledError> {
        Ok(node.outer_html())
    }

    fn node2nu(
        &self,
        html: &Self::Document,
        node: Self::Node<'_>,
        format: crate::NuDataFormat,
        span: Span,
    ) -> Result<Value, LabeledError> {
        Ok(match format {
            crate::NuDataFormat::Html => {
                node2html_nu(html, node, span).unwrap_or(Value::nothing(span))
            }
            crate::NuDataFormat::FromXmlCompat => {
                node2xml_nu(html, node, span, true).unwrap_or(Value::nothing(span))
            }
            crate::NuDataFormat::Xml => {
                node2xml_nu(html, node, span, false).unwrap_or(Value::nothing(span))
            }
            crate::NuDataFormat::InnerHtml => Value::string(self.inner_html(html, node)?, span),
            crate::NuDataFormat::OuterHtml => Value::string(self.outer_html(html, node)?, span),
        })
    }
}

fn node2html_nu(html: &HtmlDocument, node: &blitz_dom::Node, span: Span) -> Option<Value> {
    let e = match &node.data {
        blitz_dom::NodeData::Document => {
            if let Some(cn) = node.children.first() {
                let cn = html.get_node(*cn);
                return node2html_nu(html, cn.unwrap(), span);
            }
            return None;
            // return Some(Value::list(
            //     node.children
            //         .iter()
            //         .filter_map(|child_node_id| -> Option<Value> {
            //             let child_node = html.get_node(*child_node_id).unwrap();
            //             node2html_nu(html, child_node, span)
            //         })
            //         .collect::<Vec<Value>>(),
            //     span,
            // ));
        }
        blitz_dom::NodeData::Element(element_data) => element_data,
        blitz_dom::NodeData::AnonymousBlock(element_data) => element_data,
        blitz_dom::NodeData::Text(text_node_data) => {
            return Some(Value::string(text_node_data.content.clone(), span));
        }
        blitz_dom::NodeData::Comment => {
            return None;
        }
    };

    let mut out = Record::new();

    out.push("tag", Value::string(format_qual_name(&e.name), span));

    let mut attributes = Record::new();
    for attr in e.attrs().iter() {
        let name = format_qual_name(&attr.name);
        if name == "id" || name == "classes" {
            continue;
        }
        attributes.push(name, Value::string(attr.value.clone(), span));
    }
    out.push("attributes", Value::record(attributes, span));

    out.push(
        "id",
        if let Some(id) = &e.id {
            Value::string(id.to_string(), span)
        } else {
            Value::nothing(span)
        },
    );

    out.push(
        "classes",
        Value::list(
            if let Some(attr) = e
                .attrs()
                .iter()
                .find(|i| format_qual_name(&i.name) == "classes")
            {
                attr.value
                    .split_ascii_whitespace()
                    .map(|c| Value::string(c, span))
                    .collect::<Vec<Value>>()
            } else {
                Vec::new()
            },
            span,
        ),
    );

    out.push(
        "content",
        Value::list(
            node.children
                .iter()
                .filter_map(|child_node_id| -> Option<Value> {
                    let child_node = html.get_node(*child_node_id).unwrap();
                    node2html_nu(html, child_node, span)
                })
                .collect::<Vec<Value>>(),
            span,
        ),
    );

    Some(Value::record(out, span))
}

fn node2xml_nu(
    html: &HtmlDocument,
    node: &blitz_dom::Node,
    span: Span,
    text_as_elements: bool,
) -> Option<Value> {
    let e = match &node.data {
        blitz_dom::NodeData::Document => {
            if let Some(cn) = node.children.first() {
                let cn = html.get_node(*cn);
                return node2html_nu(html, cn.unwrap(), span);
            }
            return None;
            // return Some(Value::list(
            //     node.children
            //         .iter()
            //         .filter_map(|child_node_id| -> Option<Value> {
            //             let child_node = html.get_node(*child_node_id).unwrap();
            //             node2html_nu(html, child_node, span)
            //         })
            //         .collect::<Vec<Value>>(),
            //     span,
            // ));
        }
        blitz_dom::NodeData::Element(element_data) => element_data,
        blitz_dom::NodeData::AnonymousBlock(element_data) => element_data,
        blitz_dom::NodeData::Text(text_node_data) => {
            if text_as_elements {
                let mut r = Record::new();
                r.push("tag", Value::nothing(span));
                r.push("attributes", Value::nothing(span));
                r.push(
                    "content",
                    Value::string(text_node_data.content.clone(), span),
                );
                return Some(Value::record(r, span));
            }
            return Some(Value::string(text_node_data.content.clone(), span));
        }
        blitz_dom::NodeData::Comment => {
            return None;
        }
    };

    let mut out = Record::new();

    out.push("tag", Value::string(format_qual_name(&e.name), span));

    let mut attributes = Record::new();
    for attr in e.attrs().iter() {
        let name = format_qual_name(&attr.name);
        attributes.push(name, Value::string(attr.value.clone(), span));
    }
    out.push("attributes", Value::record(attributes, span));

    out.push(
        "content",
        Value::list(
            node.children
                .iter()
                .filter_map(|child_node_id| -> Option<Value> {
                    let child_node = html.get_node(*child_node_id).unwrap();
                    node2xml_nu(html, child_node, span, text_as_elements)
                })
                .collect::<Vec<Value>>(),
            span,
        ),
    );

    Some(Value::record(out, span))
}
