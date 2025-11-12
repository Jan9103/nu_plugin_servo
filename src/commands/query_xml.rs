use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature, SyntaxShape, Type, Value};

use crate::{HtmlBackend, NuDataFormat};

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
            .named("format", SyntaxShape::String, "", None)
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
        let selector: String = call.req::<String>(0)?;
        let format = NuDataFormat::parse(call.get_flag_value("format"), NuDataFormat::Xml)?;

        let b = crate::ScraperBackend;

        let xml = b.parse_xml(input)?;
        Ok(Value::list(
            b.css_query(&xml, &selector)?
                .iter()
                .map(|node| -> Result<Value, LabeledError> {
                    b.node2nu(&xml, *node, format, call.head)
                })
                .collect::<Result<Vec<Value>, LabeledError>>()?,
            call.head,
        ))
    }
}
