use data_url::DataUrl;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Record, Signature, Type, Value};

pub struct ParseDataUrlCommand;

impl SimplePluginCommand for ParseDataUrlCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo data-url parse"
    }

    fn signature(&self) -> Signature {
        Signature::new(self.name()).input_output_type(
            Type::String,
            Type::Record(Box::new([
                (
                    String::from("mime"),
                    Type::Record(Box::new([
                        (String::from("type"), Type::String),
                        (String::from("subtype"), Type::String),
                        (String::from("parameters"), Type::record()),
                    ])),
                ),
                (String::from("body"), Type::Binary),
                (String::from("fragment"), Type::Any),
            ])),
        )
    }

    fn description(&self) -> &str {
        r#"
            fragment is either null or a percent-encoded string (plugins can't annotate "oneof"? and for some reason i the crate does not expose the not precent-encoded one)
        "#
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let input: &String = match input {
            Value::String { val, .. } => val,
            _ => {
                return Err(LabeledError::new("Invalid input (expected string)"));
            }
        };
        let url = match DataUrl::process(input) {
            Ok(v) => v,
            Err(err) => {
                return Err(LabeledError::new(format!(
                    "Failed to parse data-url: {err}"
                )));
            }
        };
        let (body, fragment) = url.decode_to_vec().unwrap();
        let mime = url.mime_type();

        let span = call.head;

        let mut params = Record::new();
        for (k, v) in mime.parameters.iter() {
            params.push(k.clone(), Value::string(v.clone(), span));
        }

        Ok(Value::record(
            Record::from_raw_cols_vals(
                vec![
                    String::from("mime"),
                    String::from("body"),
                    String::from("fragment"),
                ],
                vec![
                    Value::record(
                        Record::from_raw_cols_vals(
                            vec![
                                String::from("type"),
                                String::from("subtype"),
                                String::from("parameters"),
                            ],
                            vec![
                                Value::string(mime.type_.clone(), span),
                                Value::string(mime.subtype.clone(), span),
                                Value::record(params, span),
                            ],
                            span,
                            span,
                        )?,
                        span,
                    ),
                    Value::binary(body, span),
                    (if let Some(i) = fragment {
                        Value::string(i.to_percent_encoded(), span)
                    } else {
                        Value::nothing(span)
                    }),
                ],
                span,
                span,
            )?,
            span,
        ))
    }
}
