use safer_ffi::prelude::*;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct AdapterInfo {
    pub module_path: String,
    pub components_path: String,
    pub systems_path: String,
}

#[derive_ReprC]
#[repr(C)]
pub struct AdapterInfoC {
    pub module_path: repr_c::String,
    pub components_path: repr_c::String,
    pub systems_path: repr_c::String,
}

impl From<AdapterInfo> for AdapterInfoC {
    fn from(x: AdapterInfo) -> Self {
        Self {
            components_path: x.components_path.into(),
            module_path: x.module_path.into(),
            systems_path: x.systems_path.into(),
        }
    }
}

#[derive_ReprC]
#[repr(C)]
pub struct LanguageAdapterInitArgsC {
    pub adapter_info: AdapterInfoC,
    pub callback_test: extern "C" fn(),
}
