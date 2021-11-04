// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This matches bindgen::Builder output
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr,CString};

mod types;
pub use crate::types::*;

pub fn get_version() -> String {
    unsafe { CStr::from_ptr(beagleGetVersion()).to_str().unwrap().to_owned() }
}

pub fn get_citation() -> String {
    unsafe { CStr::from_ptr(beagleGetCitation()).to_str().unwrap().to_owned() }
}

pub fn get_resource_list() -> Vec<Resource> {
    unsafe {
        let res_list = *beagleGetResourceList();
        let slice: &[BeagleResource] = std::slice::from_raw_parts(res_list.list, res_list.length as usize);
        let mut res_vec = vec![];
        for res in slice {
            res_vec.push(Resource {
                name: CStr::from_ptr(res.name).to_str().unwrap().to_owned(),
                description: CStr::from_ptr(res.description).to_str().unwrap().to_owned(),
                supportFlags: Flags::from_bits(res.supportFlags).unwrap_or(Flags::empty()),
                requiredFlags: Flags::from_bits(res.requiredFlags).unwrap_or(Flags::empty()),
            });
        }
        res_vec
    }
}

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
            tipCount, partialsBufferCount, compactBufferCount,
            stateCount, patternCount, eigenBufferCount,
            matrixBufferCount, categoryCount, scaleBufferCount,
            std::ptr::null_mut(), 0, preferenceFlags.bits(),
            requirementFlags.bits(), &mut returnInfo);

        InstanceDetails {
            resourceNumber: returnInfo.resourceNumber,
            resourceName: CStr::from_ptr(returnInfo.resourceName).to_str().unwrap().to_owned(), 
            implName: CStr::from_ptr(returnInfo.implName).to_str().unwrap().to_owned(), 
            implDescription: CStr::from_ptr(returnInfo.implDescription).to_str().unwrap().to_owned(), 
            flags: Flags::from_bits(returnInfo.flags).unwrap_or(Flags::empty()),
        }
    }
}



#[cfg(test)]
mod tests {
}

