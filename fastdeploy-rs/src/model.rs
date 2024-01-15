#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

use fastdeploy_bind::*;

use crate::enum_variables::ModelFormat;
use crate::result::{DetectionResult, OneDimDetectionResult};
use crate::runtime_option::RuntimeOption;
use crate::type_bridge::{Mat, OneDimMat};

pub struct PPYOLOE {
    ptr: *mut FD_C_PPYOLOEWrapper,
}

impl PPYOLOE {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return PPYOLOE {
                ptr: FD_C_CreatePPYOLOEWrapper(CString::new(model_file).unwrap().into_raw(),
                                               CString::new(param_file).unwrap().into_raw(),
                                               CString::new(config_file).unwrap().into_raw(),
                                               runtime_option.ptr,
                                               model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_PPYOLOEWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
        unsafe {
            FD_C_PPYOLOEWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::build(detect_result).ptr);
        }
    }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PPYOLOEWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for PPYOLOE {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyPPYOLOEWrapper(self.ptr); }
    }
}