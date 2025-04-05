use std::ffi::{c_char, CStr};
use leanquery::{leanstore_init_config, leanstore_open, LeanStoreConfig};

fn main() {
    let config = unsafe { leanstore_init_config() };
    let db = unsafe { leanstore_open(config) };
}
