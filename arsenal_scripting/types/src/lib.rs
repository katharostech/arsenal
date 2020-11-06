use std::collections::HashMap;

use safer_ffi::prelude::*;
use serde::{Deserialize, Serialize};

/// The information necessary to define a component including the component ID and the memory
/// layout.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentDefinition {
    /// The full path to the component in the module system. Must be unique among all other
    /// components
    pub path: String,
    /// The size and alignment of the component
    pub layout: Layout,
    /// The type of component this is
    pub component_type: ComponentType,
}

/// A type memory layout
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layout {
    pub size: usize,
    pub align: usize,
}

/// The type of component
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ComponentType {
    Pointer,
    Struct {
        fields: HashMap<String, ComponentDefinition>,
    },
    Primitive(Primitive),
}

/// A primitive type
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Primitive {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Char,
    Bool
}

impl Primitive {
    /// Get the memory layout of the primitive
    pub fn get_layout(&self) -> std::alloc::Layout {
        match self {
            Primitive::Char => std::alloc::Layout::from_size_align(1, 1).unwrap(),
            Primitive::Bool => std::alloc::Layout::from_size_align(1, 1).unwrap(),
            Primitive::U8   => std::alloc::Layout::from_size_align(1, 1).unwrap(),
            Primitive::U16  => std::alloc::Layout::from_size_align(2, 2).unwrap(),
            Primitive::U32  => std::alloc::Layout::from_size_align(4, 4).unwrap(),
            Primitive::U64  => std::alloc::Layout::from_size_align(8, 8).unwrap(),
            Primitive::U128 => std::alloc::Layout::from_size_align(16, 16).unwrap(),
            Primitive::I8   => std::alloc::Layout::from_size_align(1, 1).unwrap(),
            Primitive::I16  => std::alloc::Layout::from_size_align(2, 2).unwrap(),
            Primitive::I32  => std::alloc::Layout::from_size_align(4, 4).unwrap(),
            Primitive::I64  => std::alloc::Layout::from_size_align(8, 8).unwrap(),
            Primitive::I128 => std::alloc::Layout::from_size_align(16, 16).unwrap(),
            Primitive::F32  => std::alloc::Layout::from_size_align(4, 4).unwrap(),
            Primitive::F64  => std::alloc::Layout::from_size_align(8, 8).unwrap(),
        }
    }
}

#[derive_ReprC]
#[repr(C)]
pub struct LanguageAdapterInitArgsC {
    pub temp: bool
}
