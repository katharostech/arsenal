use bevy::prelude::Plugin;

use std::path::Path;

/// The Arsenal scripting plugin for Bevy
pub struct ScriptingPlugin {
    /// The path to the game to run
    game_dir: String,
}

impl ScriptingPlugin {
    /// Initialize the scripting plugin by providing a path to the script dir
    pub fn new(script_path: &str) -> Self {
        ScriptingPlugin {
            game_dir: script_path.into(),
        }
    }
}

impl Plugin for ScriptingPlugin {
    /// Initialize game scripts and create necessary systems
    fn build(&self, _app: &mut bevy::prelude::AppBuilder) {
        let components = crate::load_components(&Path::new(&self.game_dir));

        dbg!(components);
    }
}
