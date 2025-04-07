use std::ffi::{c_char, CStr, CString};
use leanquery::{leanstore_create_table, leanstore_get_adapter_u8, leanstore_get_cr_manager, leanstore_init_config, leanstore_open, LeanStoreConfig};

fn main() {
    let config = unsafe { leanstore_init_config() };
    let db = unsafe { leanstore_open(config) };
    let adapter = unsafe { leanstore_get_adapter_u8() };
    println!("adapter u8: {:?}", adapter);
    let concurrency_manager = unsafe { leanstore_get_cr_manager(db) };
    println!("concurrency manager: {:?}", concurrency_manager);
    let mut table_name = CString::new("Table0").unwrap();
    let _ = unsafe { leanstore_create_table(concurrency_manager, adapter, db, 0, table_name.into_raw()); };
}
