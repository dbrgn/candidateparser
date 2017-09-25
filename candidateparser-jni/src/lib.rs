//! JNI bindings for the candidateparser crate.
//!
//! This library is meant to be used in combination with the
//! `candidateparser-android` library.
extern crate android_logger;
extern crate candidateparser;
extern crate jni;
#[macro_use] extern crate log;

use std::ptr;

use candidateparser::IceCandidate;
use jni::JNIEnv;
use jni::errors::{Result as JniResult};
use jni::objects::{JClass, JString, JValue};
use jni::sys::{jobject, _jobject, jlong, jint};
use log::LogLevel;

/// Unwrap the object. If unwrapping fails, print the error message to the
/// Android logcat before panicking.
macro_rules! jni_unwrap {
    ($obj:expr, $msg:expr) => {
        $obj.unwrap_or_else(|e| {
            error!("Error: {}: {}", $msg, e);
            panic!($msg);
        });
    };
}

/// Create a new `null` jobject.
fn null_jobject() -> jobject {
    ptr::null_mut() as *mut _jobject
}

/// Build a vector of constructor arguments for the `IceCandidate` constructor.
fn ice_candidate_ctor_args<'a>(env: &'a JNIEnv, candidate: &IceCandidate) -> JniResult<[JValue<'a>; 7]> {
    Ok([
        // foundation
        JValue::Object(env.new_string(&candidate.foundation)?.into()),
        // componentId
        JValue::Long(candidate.component_id as jlong),
        // transport
        JValue::Object(env.new_string(candidate.transport.to_string())?.into()),
        // priority
        JValue::Long(candidate.priority as jlong),
        // connectionAddress
        JValue::Object(env.new_string(candidate.connection_address.to_string())?.into()),
        // port
        JValue::Int(candidate.port as jint),
        // candidateType
        JValue::Object(env.new_string(candidate.candidate_type.to_string())?.into()),
    ])
}

/// JNI bindings for `parseSdp` method in `ch.dbrgn.candidateparser.CandidateParser`.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system"
fn Java_ch_dbrgn_candidateparser_CandidateParser_parseSdp(env: JNIEnv,
                                                          _class: JClass,
                                                          input: JString)
                                                          -> jobject {
    android_logger::init_once(LogLevel::Info);

    // Convert parameter Java string to Rust string
    let sdp: String = jni_unwrap!(env.get_string(input),
                                  "Couldn't get java string").into();

    // Parse SDP
    let candidate = match candidateparser::parse(sdp.as_bytes()) {
        Some(cand) => cand,
        None => return null_jobject(),
    };

    // Create list of constructor parameters
    let args = match ice_candidate_ctor_args(&env, &candidate) {
        Ok(args) => args,
        Err(e) => {
            error!("Error: Could not build constructor args: {}", e);
            return null_jobject();
        },
    };

    // Create IceCandidate Java object
    let obj = jni_unwrap!(
        env.new_object(
            "ch/dbrgn/candidateparser/IceCandidate",
            "(Ljava/lang/String;JLjava/lang/String;JLjava/lang/String;ILjava/lang/String;)V",
            &args
        ), "Could not create new IceCandidate instance"
    );

    // Set optional fields
    if let Some(ip) = candidate.rel_addr {
        let ip_jni_string = jni_unwrap!(env.new_string(ip.to_string()), "Could not create JNIString from rel_addr");
        if let Err(e) = env.call_method(obj, "setRelAddr", "(Ljava/lang/String;)V",
                                        &[JValue::Object(ip_jni_string.into())]) {
            error!("Error: Could not call setRelAddr: {}", e);
            return null_jobject();
        }
    }
    if let Some(port) = candidate.rel_port {
        if let Err(e) = env.call_method(obj, "setRelPort", "(I)V", &[JValue::Int(port as i32)]) {
            error!("Error: Could not call setRelPort: {}", e);
            return null_jobject();
        }
    }
    if let Some(extensions) = candidate.extensions {
        for (k, v) in extensions {
            // Note: In theory the extension fields are byte arrays, not strings.
            // But since Java cannot use byte[] as map key, and because the Java
            // libwebrtc bindings return a String for the candidate SDP anyways,
            // we'll use strings and assume that they're valid UTF8.
            let key_jni_string = jni_unwrap!(env.new_string(String::from_utf8_lossy(&k)), "Could not create JNIString from extension key");
            let val_jni_string = jni_unwrap!(env.new_string(String::from_utf8_lossy(&v)), "Could not create JNIString from extension val");
            if let Err(e) = env.call_method(obj, "addExtension", "(Ljava/lang/String;Ljava/lang/String;)V",
                                            &[JValue::Object(key_jni_string.into()), JValue::Object(val_jni_string.into())]) {
                error!("Error: Could not call addExtension: {}", e);
                return null_jobject();
            }
        }
    }

    // Return instance
    obj.into_inner()
}
