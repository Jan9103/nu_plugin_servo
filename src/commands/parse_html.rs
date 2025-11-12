use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature, SyntaxShape, Type, Value};

use crate::{html_element_to_nu, xml_element_to_nu};

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

        Ok(match format.as_str() {
            "html" => html_element_to_nu(call.head, html.root_element()),
            "xml" => xml_element_to_nu(call.head, html.root_element(), false),
            "from xml" => xml_element_to_nu(call.head, html.root_element(), true),
            _ => panic!("impossible format value in parse_html"),
        })
    }
}
