use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature, SyntaxShape, Type, Value};

use crate::{HtmlBackend, NuDataFormat};

pub struct QueryHtmlCommand;
impl SimplePluginCommand for QueryHtmlCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo html query"
    }

    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .input_output_types(vec![(Type::String, Type::list(Type::Any))])
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
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let format = NuDataFormat::parse(call.get_flag_value("format"), NuDataFormat::Html)?;
        let selector: String = call.req::<String>(0)?;

        #[cfg(feature = "blitz_backend")]
        let b = crate::BlitzBackend;
        #[cfg(not(feature = "blitz_backend"))]
        let b = crate::ScraperBackend;

        let html = b.parse(input)?;
        Ok(Value::list(
            b.css_query(&html, &selector)?
                .iter()
                .map(|node| -> Result<Value, LabeledError> {
                    b.node2nu(&html, *node, format, call.head)
                })
                .collect::<Result<Vec<Value>, LabeledError>>()?,
            call.head,
        ))
    }
}
