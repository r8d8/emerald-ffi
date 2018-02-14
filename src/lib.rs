//! Foreign Function Interface (FFI) bindings for the `emerald-rs` crate.
#![allow(non_snake_case)]

extern crate emerald_rs;
extern crate libc;

use libc::{c_char, size_t, uint8_t, int32_t, uint32_t};
use std::{ptr, slice};
use std::ffi::CStr;
use std::io::{Cursor, Write};
use emerald_rs::keystore::KeyFile;
use std::mem::transmute;


#[no_mangle]
pub extern "C" fn signTransaction() -> bool {

}

#[no_mangle]
pub extern "C" fn generateAddress() -> bool {
    emerald_rs::Address
}

#[no_mangle]
pub extern "C" fn createKeyfile(passphrase: &str,
                                sec_level: &KdfDepthLevel,
                                name: String,
                                description: String) -> *mut KeyFile {

    unsafe {
        transmute(Box::new(KeyFile::new(passphrase, sec_level, name, description)))
    }
}

#[no_mangle]
pub extern fn deleteKeyfile(ptr: *mut KeyFile) {
    let _kf: Box<KeyFile> = unsafe{ transmute(ptr) };
}
