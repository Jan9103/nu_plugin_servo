use crate::{HtmlBackend, NuDataFormat};
use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, SyntaxShape, Type};

pub struct ParseHtmlCommand;

impl SimplePluginCommand for ParseHtmlCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo html parse"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::new(self.name())
            .input_output_types(vec![(Type::String, Type::record())])
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
        let format = NuDataFormat::parse(call.get_flag_value("format"), NuDataFormat::Html)?;

        #[cfg(feature = "blitz_backend")]
        let b = crate::BlitzBackend;
        #[cfg(not(feature = "blitz_backend"))]
        let b = crate::ScraperBackend;

        let html = b.parse(input)?;
        let root = b.get_root_node(&html)?;
        b.node2nu(&html, root, format, call.head)
    }
}
