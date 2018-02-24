//! Foreign Function Interface (FFI) bindings for the `emerald-rs` crate.
#![allow(non_snake_case)]

extern crate emerald_rs;
extern crate libc;

use emerald_rs::keystore::KeyFile;
use emerald_rs::Address;
use libc::{c_uchar, size_t, int32_t, uint32_t, uint8_t};
use std::{ptr, slice};
use std::ffi::{CStr, CString};
use std::io::{Cursor, Write};
use std::mem::transmute;


#[repr(C)]
pub enum KdfDepthLevel {
    Normal = 1024,
    High = 8096,
    Ultra = 262_144,
}

#[repr(C)]
pub struct c_keyfile {
    pub visible: bool,
    pub name: *const c_uchar,
    pub name_len: c_uint,
    pub description: *const c_uchar,
    pub description_len: c_uint,
    pub address: c_address,
    pub uuid: c_uuid,
    pub crypto: c_crypto,
}

#[no_mangle]
pub extern "C" fn signTransaction() -> bool {}

#[no_mangle]
pub extern "C" fn createKeyfile(
    passphrase: *const c_char,
    sec_level: KdfDepthLevel,
    name: *const c_char,
    description: *const c_char,
) -> *mut KeyFile {
    let pass: String = unsafe { CStr::from_ptr(passphrase).to_string_lossy().into_owned() };
    let name: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    let desc: String = unsafe { CStr::from_ptr(description).to_string_lossy().into_owned() };
    unsafe {
        transmute(Box::new(KeyFile::new(
            pass,
            sec_level,
            name,
            desc,
        )))
    }
}

#[no_mangle]
pub extern "C" fn deleteKeyfile(ptr: *mut KeyFile) {
    let _kf: Box<KeyFile> = unsafe { transmute(ptr) };
}
