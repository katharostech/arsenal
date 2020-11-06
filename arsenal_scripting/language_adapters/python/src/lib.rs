use safer_ffi::prelude::*;

use arsenal_scripting_types::*;

#[ffi_export]
fn init_adapter(args: &LanguageAdapterInitArgsC) {
    println!("Initialized Python language adapter!");
    let components_path = &args.adapter_info.components_path;

    println!("Components path: {:?}", components_path);

    (args.callback_test)()
}
