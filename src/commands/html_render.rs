use anyrender::render_to_buffer;
use anyrender_vello::VelloImageRenderer;
use blitz_paint::paint_scene;
use blitz_traits::shell::{ColorScheme, Viewport};
use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature, SyntaxShape, Type, Value};

use crate::{BlitzBackend, HtmlBackend};

pub struct HtmlRenderCommand;

impl SimplePluginCommand for HtmlRenderCommand {
    type Plugin = crate::plugin_interface::NuPluginServo;

    fn name(&self) -> &str {
        "servo html render_as"
    }

    fn signature(&self) -> Signature {
        Signature::new(self.name())
            .input_output_type(Type::String, Type::Binary)
            .required("format", SyntaxShape::String, "One of: ['png']")
            .named("width", SyntaxShape::Int, "", None)
            .named("scale", SyntaxShape::Float, "", None)
            .named(
                "colorscheme",
                SyntaxShape::String,
                "'light' or 'dark'",
                None,
            )
            .named(
                "baseurl",
                SyntaxShape::String,
                "URI from where it was fetched (enables networking and fetches extra resources)",
                None,
            )
            .named("ppm", SyntaxShape::Int, "PixelPerMeter", None)
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
        let format: String = match call.nth(0) {
            Some(Value::String { val, .. }) if ["png"].contains(&val.as_str()) => val,
            None => {
                return Err(LabeledError::new("Missing required argument: `format`"));
            }
            _ => {
                return Err(LabeledError::new("Invalid argument value for `format`"));
            }
        };
        let width: u32 = match call.get_flag_value("width") {
            Some(Value::Int { val, .. }) => val as u32,
            None => 1200,
            _ => {
                return Err(LabeledError::new("Invalid argument value for `--width`"));
            }
        };
        let ppm: u32 = match call.get_flag_value("ppm") {
            Some(Value::Int { val, .. }) => val as u32,
            None => (144.0 * 39.3701) as u32,
            _ => {
                return Err(LabeledError::new("Invalid argument value for `--ppm`"));
            }
        };
        let scale: f64 = match call.get_flag_value("scale") {
            Some(Value::Float { val, .. }) => val,
            Some(Value::Int { val, .. }) => val as f64,
            None => 1.0f64,
            _ => {
                return Err(LabeledError::new("Invalid argument value for `--width`"));
            }
        };
        let color_scheme: ColorScheme = match call.get_flag_value("colorscheme") {
            Some(Value::String { val, .. }) if val.as_str() == "light" => ColorScheme::Light,
            Some(Value::String { val, .. }) if val.as_str() == "dark" => ColorScheme::Dark,
            None => ColorScheme::Light,
            _ => {
                return Err(LabeledError::new(
                    "Invalid argument value for `--colorscheme`",
                ));
            }
        };
        let base_url: Option<String> = match call.get_flag_value("baseurl") {
            Some(Value::String { val, .. }) => Some(val),
            None => None,
            _ => {
                return Err(LabeledError::new("Invalid argument value for `--baseurl`"));
            }
        };
        let height: u32 = 800;
        let viewport = Viewport::new(
            width * (scale as u32),
            height * (scale as u32),
            scale as f32,
            color_scheme,
        );

        // load HTML
        let b = BlitzBackend;
        let mut dom = if let Some(base_url) = base_url {
            b.parse_with_web(input, base_url, viewport)
        } else {
            b.parse(input)
        }?;

        // render
        dom.resolve(0.0);
        // let mut scene: vello::Scene = vello::Scene::new();
        // let mut scene_painter = anyrender_vello::VelloScenePainter::new(&mut scene);
        let computed_height: f32 = dom.root_element().final_layout.size.height;
        let render_width: u32 = (width as f64 * scale) as u32;
        let render_height: u32 =
            ((computed_height as f64).max(height as f64).min(4000.0) * scale) as u32;
        let buffer: Vec<u8> = render_to_buffer::<VelloImageRenderer, _>(
            |scene| paint_scene(scene, &dom, scale, render_width, render_height),
            render_width,
            render_height,
        );

        let out: Vec<u8> = match format.as_str() {
            "png" => {
                let mut out: Vec<u8> = Vec::new();
                let mut encoder = png::Encoder::new(&mut out, render_width, render_height);
                encoder.set_color(png::ColorType::Rgb); // no alpha in the buffer, so no alpha here
                encoder.set_depth(png::BitDepth::Eight);
                encoder.set_pixel_dims(Some(png::PixelDimensions {
                    xppu: ppm,
                    yppu: ppm,
                    unit: png::Unit::Meter,
                }));
                let mut writer = encoder.write_header().unwrap();
                writer.write_image_data(&buffer).unwrap();
                writer.finish().unwrap();
                out
            }
            _ => {
                unreachable!("`servo html render <format>`: format registered, but not implemented")
            }
        };

        Ok(Value::binary(out, call.head))
    }
}
