//! Tools that may be used by scripting systems

use arsenal_scripting_types::ComponentDefinition;

use std::path::Path;

/// Get an export of all of the components in the game, including all of the components added by
/// different language adapters and scripts.
pub fn load_components(game_dir: &Path) -> Vec<ComponentDefinition> {
    let mut components = Vec::new();
    let modules = crate::utils::load_modules(game_dir);

    for module in modules.values() {
        components.extend(
            serde_cbor::from_slice::<Vec<ComponentDefinition>>(
                module.get_components().as_ref().as_slice(),
            )
            .expect("Could not parse components from language adapter"),
        );
    }

    components
}
