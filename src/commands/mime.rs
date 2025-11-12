use std::str::FromStr;

use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Record, Signature, Type, Value};

pub struct ParseMimeCommand;

impl SimplePluginCommand for ParseMimeCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo mime parse"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::new(self.name()).input_output_type(
            Type::String,
            Type::Record(Box::new([
                (String::from("type"), Type::String),
                (String::from("subtype"), Type::String),
                (String::from("parameters"), Type::record()),
            ])),
        )
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
        let input: &String = match input {
            Value::String { val, .. } => val,
            _ => {
                return Err(LabeledError::new("Invalid input (expected string)"));
            }
        };
        let m = match mime::Mime::from_str(input) {
            Ok(v) => v,
            Err(err) => {
                return Err(LabeledError::new(format!("Failed to parse mime: {err}")));
            }
        };
        let span = call.head;

        let mut params = Record::new();
        for (k, v) in m.params() {
            params.push(k.to_string(), Value::string(v.to_string(), span));
        }

        Ok(Value::record(
            Record::from_raw_cols_vals(
                vec![
                    String::from("type"),
                    String::from("subtype"),
                    String::from("parameters"),
                ],
                vec![
                    Value::string(m.type_().to_string(), span),
                    Value::string(m.subtype().to_string(), span),
                    Value::record(params, span),
                ],
                span,
                span,
            )?,
            span,
        ))
    }
}
