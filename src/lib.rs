pub mod commands;

#[cfg(feature = "blitz_backend")]
mod blitz_backend;
#[cfg(feature = "blitz_backend")]
pub use blitz_backend::BlitzBackend;
#[cfg(feature = "scraper_backend")]
mod scraper_backend;
#[cfg(feature = "scraper_backend")]
pub use scraper_backend::ScraperBackend;

pub mod plugin_interface;

use html5ever::QualName;
use nu_protocol::{LabeledError, Span, Value};

#[cfg(not(any(feature = "blitz_backend", feature = "scraper_backend")))]
compile_error!("You should enable either blitz_backend or scraper_backend (or both)");

fn format_qual_name(qn: &QualName) -> String {
    if let Some(p) = &qn.prefix {
        let mut out = String::new();
        out.push_str(p);
        out.push(':');
        out.push_str(&qn.local);
        out
    } else {
        qn.local.to_string()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum NuDataFormat {
    Html,
    Xml,
    FromXmlCompat,
    InnerHtml,
    OuterHtml,
}
impl NuDataFormat {
    pub fn parse(arg: Option<Value>, default: Self) -> Result<Self, LabeledError> {
        match arg {
            Some(Value::String { val, .. }) if val.as_str() == "html" => Ok(Self::Html),
            Some(Value::String { val, .. }) if val.as_str() == "xml" => Ok(Self::Xml),
            Some(Value::String { val, .. }) if val.as_str() == "from xml" => {
                Ok(Self::FromXmlCompat)
            }
            Some(Value::String { val, .. }) if val.as_str() == "inner html" => Ok(Self::InnerHtml),
            Some(Value::String { val, .. }) if val.as_str() == "outer html" => Ok(Self::OuterHtml),
            Some(Value::Nothing { .. }) | None => Ok(default),
            _ => Err(LabeledError::new("Invalid '--format' argument")),
        }
    }
}

pub trait HtmlBackend {
    type Document;
    type Node<'a>;

    fn parse(&self, document: &Value) -> Result<Self::Document, LabeledError>;

    fn get_root_node<'a>(&self, html: &'a Self::Document) -> Result<Self::Node<'a>, LabeledError>;

    fn css_query<'a>(
        &self,
        html: &'a Self::Document,
        selector: &str,
    ) -> Result<Vec<Self::Node<'a>>, LabeledError>;

    fn inner_html(
        &self,
        html: &Self::Document,
        node: Self::Node<'_>,
    ) -> Result<String, LabeledError>;

    fn outer_html(
        &self,
        html: &Self::Document,
        node: Self::Node<'_>,
    ) -> Result<String, LabeledError>;

    fn node2nu(
        &self,
        html: &Self::Document,
        node: Self::Node<'_>,
        format: NuDataFormat,
        span: Span,
    ) -> Result<Value, LabeledError>;
}
