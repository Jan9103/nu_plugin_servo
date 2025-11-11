use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_servo as np;

fn main() {
    serve_plugin(
        &np::plugin_interface::NuPluginServo {},
        MsgPackSerializer {},
    )
}
