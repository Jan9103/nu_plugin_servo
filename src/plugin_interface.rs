pub struct NuPluginServo;

impl nu_plugin::Plugin for NuPluginServo {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            // +------+
            // | HTML |
            // +------+
            Box::new(crate::commands::parse_html::ParseHtmlCommand),
            Box::new(crate::commands::query_html::QueryHtmlCommand),
            // +-----+
            // | XML |
            // +-----+
            #[cfg(feature = "xml")]
            Box::new(crate::commands::parse_xml::ParseXmlCommand),
            #[cfg(feature = "xml")]
            Box::new(crate::commands::query_xml::QueryXmlCommand),
            // +-----+
            // | URL |
            // +-----+
            #[cfg(feature = "data_url")]
            Box::new(crate::commands::parse_data_url::ParseDataUrlCommand),
            // +------+
            // | MIME |
            // +------+
            #[cfg(feature = "mime")]
            Box::new(crate::commands::mime::ParseMimeCommand),
        ]
    }
}
