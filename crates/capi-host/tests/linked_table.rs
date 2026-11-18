//! A program that links `liblunamodel` gets the C API table from `lm_c_api_table` instead of
//! the Python wheel's capsule, and the SDK works on it like on the capsule's table.

use luna_model_ffi::Model;
use lunamodel_capi_host::capi_table::lm_c_api_table;

#[test]
fn linked_table_initializes_the_sdk() {
    unsafe { luna_model_ffi::init(lm_c_api_table()) }.expect("init from the linked table");
    assert_eq!(
        luna_model_ffi::host_table_size(),
        std::mem::size_of::<lunamodel_capi::LunaModelCApi>()
    );

    let model = Model::new(Some("linked".into()), None).expect("create a model");
    assert_eq!(model.name().expect("read the name"), "linked");
}
