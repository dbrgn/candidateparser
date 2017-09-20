extern crate candidateparser;
extern crate libc;

use libc::c_char;
use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::ptr;

#[repr(C)]
pub struct IceCandidateFFI {
    pub foundation: *const c_char,
    pub component_id: u32,
    pub transport: *const c_char,
    pub priority: u64,
    pub connection_address: *const c_char,
    pub port: u16,
    pub candidate_type: *const c_char,
    pub rel_addr: *const c_char, // Nullable (nullptr)
    pub rel_port: u16, // Nullable (0)
    //pub extensions: [*const c_char], // Nullable (nullptr) // TODO
}

#[no_mangle]
pub extern "C" fn parse(sdp: *const c_char) -> *const IceCandidateFFI {
    // Convert C string to Rust byte slice
    let cstr_sdp = unsafe {
        assert!(!sdp.is_null());
        CStr::from_ptr(sdp)
    };

    // Parse
    let parsed = match candidateparser::parse(cstr_sdp.to_bytes()) {
        Some(candidate) => candidate,
        None => return ptr::null(),
    };

    // Convert to FFI representation
    let transport_cstring: CString = parsed.transport.into();
    let candidate_type_cstring: CString = parsed.candidate_type.into();
    let boxed = Box::new(IceCandidateFFI {
        foundation: CString::new(parsed.foundation).unwrap().into_raw(),
        component_id: parsed.component_id,
        transport: transport_cstring.into_raw(),
        priority: parsed.priority,
        connection_address: CString::new(parsed.connection_address.to_string()).unwrap().into_raw(),
        port: parsed.port,
        candidate_type: candidate_type_cstring.into_raw(),
        rel_addr: match parsed.rel_addr {
            Some(addr) => CString::new(addr.to_string()).unwrap().into_raw(),
            None => ptr::null(),
        },
        rel_port: parsed.rel_port.unwrap_or(0),
    });

    Box::into_raw(boxed)
}
