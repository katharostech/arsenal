//! Bindings to Bevy's transforms

use bevy::{
    ecs::ComponentId, math::Quat, math::Vec3, transform::components::Transform, utils::HashMap,
};
use safer_ffi::prelude::*;

use std::any::TypeId;

use crate::types::*;

/// Add the transform types to the given vector
pub fn add_types(types: &mut HashMap<ComponentId, ScriptTypeInfo>) {
    // Transform component
    types.insert(
        ComponentId::RustTypeId(TypeId::of::<Transform>()),
        ScriptTypeInfo {
            kind: TypeKind::Pointer,
            method_definitions: vec![
                ScriptMethodDefinition {
                    name: "get_translation".into(),
                    arguments: vec![],
                    return_type: ComponentId::RustTypeId(TypeId::of::<Vec3>()),
                },
                ScriptMethodDefinition {
                    name: "get_rotation".into(),
                    arguments: vec![],
                    return_type: ComponentId::RustTypeId(TypeId::of::<Quat>()),
                },
                ScriptMethodDefinition {
                    name: "get_scale".into(),
                    arguments: vec![],
                    return_type: ComponentId::RustTypeId(TypeId::of::<Vec3>()),
                },
            ],
            method_pointers: vec![get_translation, get_rotation, get_scale],
        },
    );

    // Vec3 type used for translation, and scale
    types.insert(
        ComponentId::RustTypeId(TypeId::of::<Vec3>()),
        ScriptTypeInfo {
            kind: TypeKind::Pointer,
            method_definitions: vec![
                ScriptMethodDefinition {
                    name: "get_x".into(),
                    arguments: vec![],
                    return_type: ComponentId::RustTypeId(TypeId::of::<f32>()),
                },
                ScriptMethodDefinition {
                    name: "get_y".into(),
                    arguments: vec![],
                    return_type: ComponentId::RustTypeId(TypeId::of::<f32>()),
                },
                ScriptMethodDefinition {
                    name: "get_z".into(),
                    arguments: vec![],
                    return_type: ComponentId::RustTypeId(TypeId::of::<f32>()),
                },
            ],
            method_pointers: vec![get_x, get_y, get_z],
        },
    );

    // Quat type used for rotations
    types.insert(
        ComponentId::RustTypeId(TypeId::of::<Vec3>()),
        ScriptTypeInfo {
            kind: TypeKind::Pointer,
            method_definitions: vec![],
            method_pointers: vec![],
        },
    );
}

//
// Transform bindings
//

extern "C" fn get_translation(this: FFIObj, _args: c_slice::Ref<FFIObj>) -> FFIObj {
    let transform: &Transform = this.downcast_ref().expect("Could not downcast arg");

    FFIObj::new::<Vec3>(transform.translation)
}

extern "C" fn get_rotation(this: FFIObj, _args: c_slice::Ref<FFIObj>) -> FFIObj {
    let transform: &Transform = this.downcast_ref().expect("Could not downcast arg");

    FFIObj::new(transform.rotation)
}

extern "C" fn get_scale(this: FFIObj, _args: c_slice::Ref<FFIObj>) -> FFIObj {
    let transform: &Transform = this.downcast_ref().expect("Could not downcast arg");

    FFIObj::new(transform.scale)
}

//
// Vec3 bindings
//

extern "C" fn get_x(this: FFIObj, _args: c_slice::Ref<FFIObj>) -> FFIObj {
    let vec: &Vec3 = this.downcast_ref().expect("Could not downcast arg");

    FFIObj::new(vec.x())
}

extern "C" fn get_y(this: FFIObj, _args: c_slice::Ref<FFIObj>) -> FFIObj {
    let vec: &Vec3 = this.downcast_ref().expect("Could not downcast arg");

    FFIObj::new(vec.y())
}

extern "C" fn get_z(this: FFIObj, _args: c_slice::Ref<FFIObj>) -> FFIObj {
    let vec: &Vec3 = this.downcast_ref().expect("Could not downcast arg");

    FFIObj::new(vec.z())
}

//
// Quat bindings
//

// TODO: Quat bindings
