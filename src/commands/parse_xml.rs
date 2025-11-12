use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, SyntaxShape, Type};

use crate::{HtmlBackend, NuDataFormat};

pub struct ParseXmlCommand;

impl SimplePluginCommand for ParseXmlCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo xml parse"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::new(self.name())
            .input_output_types(vec![
                (Type::String, Type::record()),
                (Type::Binary, Type::record()),
            ])
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
        let format = NuDataFormat::parse(call.get_flag_value("format"), NuDataFormat::Xml)?;
        let b = crate::ScraperBackend;
        let xml = b.parse_xml(input)?;
        let root = b.get_root_node(&xml)?;
        b.node2nu(&xml, root, format, call.head)
    }
}
