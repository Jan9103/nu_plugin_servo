use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature, SyntaxShape, Type, Value};
use scraper::Selector;

use crate::{html_element_to_nu, xml_element_to_nu};

pub struct QueryHtmlCommand;

impl SimplePluginCommand for QueryHtmlCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo html query"
    }

    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .input_output_types(vec![
                (Type::String, Type::list(Type::String)),
                (Type::Binary, Type::list(Type::String)),
            ])
            .required("css_selector", SyntaxShape::String, "css selector")
            .switch(
                "inner-html",
                "get the inner html of the found object (exclude the element itself)",
                None,
            )
    }

    fn description(&self) -> &str {
        ""
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_protocol::LabeledError> {
        let target_inner: bool = call.has_flag("inner-html")?;
        let css_selector: String = call.req::<String>(0)?;
        let css_selector: Selector = Selector::parse(&css_selector)
            .map_err(|err| LabeledError::new(format!("Failed to parse CSS: {err}")))?;

        let mut document: &[u8] = match input {
            Value::String { val, .. } => val.as_bytes(),
            Value::Binary { val, .. } => val,
            _ => {
                return Err(LabeledError::new("Input type neither string nor binary"));
            }
        };

        let html = match crate::parse_html(&mut document) {
            Ok(v) => v,
            Err(e) => {
                return Err(nu_protocol::LabeledError::new(format!("invalid HTML: {e}")));
            }
        };

        Ok(Value::list(
            html.select(&css_selector)
                .map(|matched_html| -> Value {
                    Value::string(
                        if target_inner {
                            matched_html.inner_html()
                        } else {
                            matched_html.html()
                        },
                        call.head,
                    )
                })
                .collect::<Vec<Value>>(),
            call.head,
        ))
    }
}

pub struct QueryParseHtmlCommand;

impl SimplePluginCommand for QueryParseHtmlCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo html query_parse"
    }

    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .input_output_types(vec![
                (Type::String, Type::list(Type::record())),
                (Type::Binary, Type::list(Type::record())),
            ])
            .required("css_selector", SyntaxShape::String, "css selector")
            .named("format", SyntaxShape::String, r#"one of: "html" (default), "xml" (servo xml parse), "from xml" (nu's builtin xml parser)"#, None)
    }

    fn description(&self) -> &str {
        ""
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_protocol::LabeledError> {
        let format: String = match call.get_flag_value("format") {
            Some(Value::String { val, .. })
                if ["html", "xml", "from xml"].contains(&val.as_str()) =>
            {
                val
            }
            None => "html".into(),
            _ => {
                return Err(LabeledError::new(
                    "Invalid '--format' argument - see '--help'",
                ));
            }
        };
        let css_selector: String = call.req::<String>(0)?;
        let css_selector: Selector = Selector::parse(&css_selector)
            .map_err(|err| LabeledError::new(format!("Failed to parse CSS: {err}")))?;

        let mut document: &[u8] = match input {
            Value::String { val, .. } => val.as_bytes(),
            Value::Binary { val, .. } => val,
            _ => {
                return Err(LabeledError::new("Input type neither string nor binary"));
            }
        };

        let html = match crate::parse_html(&mut document) {
            Ok(v) => v,
            Err(e) => {
                return Err(nu_protocol::LabeledError::new(format!("invalid HTML: {e}")));
            }
        };

        Ok(Value::list(
            html.select(&css_selector)
                .map(|matched_html| -> Value {
                    match format.as_str() {
                        "html" => html_element_to_nu(call.head, matched_html),
                        "xml" => xml_element_to_nu(call.head, matched_html, false),
                        "from xml" => xml_element_to_nu(call.head, matched_html, true),
                        _ => panic!("impossible format value in parse_html"),
                    }
                })
                .collect::<Vec<Value>>(),
            call.head,
        ))
    }
}
