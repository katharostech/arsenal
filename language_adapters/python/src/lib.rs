use safer_ffi::prelude::*;

#[ffi_export]
fn init_plugin() {
    println!("Initialized Python language adapter!");
}
