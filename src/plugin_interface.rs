pub struct NuPluginServo;

impl nu_plugin::Plugin for NuPluginServo {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(crate::commands::query_html::QueryHtmlCommand),
            Box::new(crate::commands::query_html::QueryParseHtmlCommand),
            Box::new(crate::commands::parse_html::ParseHtmlCommand),
        ]
    }
}
