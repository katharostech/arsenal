use bevy::ecs::ComponentId;
use safer_ffi::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ffi::FFIObj;

use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AdapterInfo {
    pub module_path: String,
    pub components_path: String,
    pub systems_path: String,
}

/// All of the information about a component type necessary to be used for scripting
pub struct ScriptTypeInfo {
    /// The way this component is represented to scripts from a data layout perspective
    pub kind: TypeKind,
    /// The definitions of the methods associated to to the [`method_pointers`] with the same index.
    pub method_definitions: Vec<ScriptMethodDefinition>,
    /// The methods associated to this component
    pub method_pointers:
        Vec<extern "C" fn(FFIObj /*this*/, c_slice::Ref<FFIObj> /*args*/) -> FFIObj>,
}

/// The kind of component from a data layout perspective
pub enum TypeKind {
    /// An opaque pointer to the component data
    ///
    /// Components of this kind can only be modified through associated methods
    Pointer,
}

/// the definition for a script type's method
pub struct ScriptMethodDefinition {
    pub name: Cow<'static, str>,
    pub arguments: Vec<ScriptMethodArg>,
    pub return_type: ComponentId,
}

/// An argument to a script type's method
pub struct ScriptMethodArg {
    pub name: Cow<'static, str>,
    pub type_id: ComponentId,
}
