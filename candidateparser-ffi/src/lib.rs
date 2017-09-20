//! C FFI bindings for *candidateparser* library.
//!
//! The header file is provided by this library, you can find it in the crate source code under
//! [`src/candidateparser.h`](https://github.com/dbrgn/candidateparser/blob/master/candidateparser-ffi/candidateparser.h).
//!
//! You can find an example C program under
//! [`src/example.c`](https://github.com/dbrgn/candidateparser/blob/master/candidateparser-ffi/example.c).
extern crate candidateparser;
extern crate libc;

use libc::c_char;
use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::ptr;

/// A wrapper around the `IceCandidate` data that is C compatible.
#[derive(Debug)]
#[repr(C)]
pub struct IceCandidateFFI {
    pub foundation: *const c_char,
    pub component_id: u32,
    pub transport: *const c_char,
    pub priority: u64,
    pub connection_address: *const c_char,
    pub port: u16,
    pub candidate_type: *const c_char,
    /// The address is optional. If no value is defined, this will contain a
    /// null pointer.
    pub rel_addr: *const c_char,
    /// This port is optional. If no address is defined, this will contain the
    /// value `0`.
    pub rel_port: u16, // Nullable (0)
    //pub extensions: [*const c_char], // Nullable (nullptr) // TODO
}

/// Parse an ICE candidate SDP string and return a pointer to an
/// [`IceCandidateFFI`](struct.IceCandidateFFI.html) struct.
///
/// Make sure to always call the [`free_ice_candidate`](fn.free_ice_candidate.html)
/// function after you're done processing the data, to prevent memory leaks!
#[no_mangle]
pub extern "C" fn parse_ice_candidate_sdp(sdp: *const c_char) -> *const IceCandidateFFI {
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

/// Free the memory associated with the [`IceCandidateFFI`](struct.IceCandidateFFI.html) struct.
///
/// Make sure to always call this function after you're done processing the
/// data, otherwise you'll end up with memory leaks!
#[no_mangle]
pub extern "C" fn free_ice_candidate(ptr: *const IceCandidateFFI) {
    if ptr.is_null() { return; }
    let candidate: Box<IceCandidateFFI> = unsafe { Box::from_raw(ptr as *mut IceCandidateFFI) };
    unsafe { CString::from_raw(candidate.foundation as *mut c_char) };
    unsafe { CString::from_raw(candidate.transport as *mut c_char) };
    unsafe { CString::from_raw(candidate.connection_address as *mut c_char) };
    unsafe { CString::from_raw(candidate.candidate_type as *mut c_char) };
    if !candidate.rel_addr.is_null() {
        unsafe { CString::from_raw(candidate.rel_addr as *mut c_char) };
    }
    // Resources will be freed here
}
