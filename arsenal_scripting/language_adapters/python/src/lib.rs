use safer_ffi::prelude::*;

use arsenal_scripting_types::*;

#[ffi_export]
fn init_adapter<'a>(args: &'a mut LanguageAdapterInitArgsC) {

}

#[ffi_export]
fn get_components() -> repr_c::Vec<u8> {
    let components = vec![ComponentDefinition {
        component_type: ComponentType::Primitive(Primitive::U8),
        layout: Layout { size: 1, align: 1 },
        path: String::from("python::example::Test"),
    }];

    serde_cbor::to_vec(&components).unwrap().into()
}
