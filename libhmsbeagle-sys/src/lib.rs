// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This matches bindgen::Builder output
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr,CString};

mod types;
pub use crate::types::*;

pub fn create_instance(tipCount: i32, partialsBufferCount: i32,
                       compactBufferCount: i32, stateCount: i32,
                       patternCount: i32, eigenBufferCount: i32,
                       matrixBufferCount: i32, categoryCount: i32,
                       scaleBufferCount: i32,
                       // Let's ignore resource restrictions for now...
                       preferenceFlags: Flags,
                       requirementFlags: Flags) -> InstanceDetails {
    unsafe {
        let mut returnInfo = BeagleInstanceDetails {
            resourceNumber: 0,
            resourceName: std::ptr::null_mut(),
            implName: std::ptr::null_mut(),
            implDescription: std::ptr::null_mut(),
            flags: 0,
        };

        // create an instance of the BEAGLE library
        let instance = beagleCreateInstance(
            tipCount, partialsBufferCount, compactBufferCount, stateCount, patternCount, eigenBufferCount,
            matrixBufferCount, categoryCount, scaleBufferCount, std::ptr::null_mut(), 0,
            preferenceFlags.bits(), requirementFlags.bits(), &mut returnInfo);

        InstanceDetails {
            resourceNumber: returnInfo.resourceNumber,
            resourceName: CStr::from_ptr(returnInfo.resourceName).to_str().unwrap().to_owned(), 
            implName: CStr::from_ptr(returnInfo.implName).to_str().unwrap().to_owned(), 
            implDescription: CStr::from_ptr(returnInfo.implDescription).to_str().unwrap().to_owned(), 
            flags: Flags::from_bits(returnInfo.flags).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {

}
