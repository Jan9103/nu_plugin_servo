use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature, Type, Value};

use crate::html_element_to_nu;

pub struct ParseHtmlCommand;

impl SimplePluginCommand for ParseHtmlCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo html parse"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::new(self.name()).input_output_types(vec![
            (Type::String, Type::record()),
            (Type::Binary, Type::record()),
        ])
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

        Ok(html_element_to_nu(call.head, html.root_element()))
    }
}
