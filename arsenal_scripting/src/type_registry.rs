// //! Provides a registry for mapping human friendly type names to their appropriate
// //! [`ScriptTypeId`]'s.

// use std::sync::{Arc, RwLock};

// use bevy::utils::HashMap;


// lazy_static::lazy_static! {
//     static ref TYPE_REGISTRY: TypeRegistry = TypeRegistry::new();
// }

// /// A type registry that stores all of the names of scripted types and their [`ScriptTypeId`]'s
// pub struct TypeRegistry {
//     /// Mapping of the type
//     registry: Arc<RwLock<HashMap<String, ()>>>,
// }

// impl TypeRegistry {
//     fn new() -> Self {
//         Self {
//             registry: Default::default(),
//         }
//     }

//     /// Register a type and get its type Id
//     pub fn register_type(name: &str) -> ScriptTypeId {}
// }
