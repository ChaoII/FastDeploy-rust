#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString};

use fastdeploy_bind::*;

#[derive(Debug)]
pub struct DetectionResult {
    pub ptr: *mut FD_C_DetectionResult,
}

pub struct OneDimDetectionResult {
    pub ptr: *mut FD_C_OneDimDetectionResult,
}

impl OneDimDetectionResult {
    pub fn build(data: &mut Vec<DetectionResult>) -> Self {
        unsafe {
            let mut c = FD_C_OneDimDetectionResult {
                size: data.len(),
                data: (*data.as_mut_ptr()).ptr,
            };
            return OneDimDetectionResult {
                ptr: &mut c as *mut FD_C_OneDimDetectionResult,
            };
        }
    }
}

impl DetectionResult {
    pub fn new() -> DetectionResult {
        unsafe {
            DetectionResult {
                ptr: FD_C_CreateDetectionResult(),
            }
        }
    }
    pub fn str(&self) -> &str {
        unsafe {
            let s: &str = &String::from_utf8(vec![0x01; 10240]).unwrap();
            let c = CString::new(s).unwrap().into_raw();
            FD_C_DetectionResultStr(self.ptr, c);
            return CStr::from_ptr(c).to_str().unwrap();
        }
    }
}

impl Drop for DetectionResult {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyDetectionResult(self.ptr);
        }
    }
}

