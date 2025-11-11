use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature, SyntaxShape, Type, Value};
use scraper::Selector;

pub struct QueryXmlCommand;

impl SimplePluginCommand for QueryXmlCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo xml query"
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

pub struct QueryParseXmlCommand;

impl SimplePluginCommand for QueryParseXmlCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo xml query_parse"
    }

    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .input_output_types(vec![
                (Type::String, Type::list(Type::record())),
                (Type::Binary, Type::list(Type::record())),
            ])
            .required("css_selector", SyntaxShape::String, "css selector")
            .switch(
                "from-xml-compat",
                "generate the same output format as 'from xml'",
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
        let fxc: bool = call.has_flag("from-xml-compat").map_err(|e| {
            LabeledError::new(format!(
                "--from-xml-compat is a flag, not whatever you tried: {e}"
            ))
        })?;

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
                    crate::xml_element_to_nu(call.head, matched_html, fxc)
                })
                .collect::<Vec<Value>>(),
            call.head,
        ))
    }
}
