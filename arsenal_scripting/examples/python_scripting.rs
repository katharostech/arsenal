use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugin(arsenal_scripting::ScriptingPlugin::new(
            "./examples/python_scripting",
        ))
        .run();
}
