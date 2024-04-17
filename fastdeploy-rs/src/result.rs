#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString};

use fastdeploy_bind::*;

use crate::enum_variables::ResultType;
use crate::type_bridge::common::*;

type detect_result_t = *mut FD_C_DetectionResult;

#[derive(Debug)]
pub struct ClassifyResult {
    pub label_ids: Vec<i32>,
    pub scores: Vec<f32>,
    pub type_: ResultType,
}

impl ClassifyResult {
    pub fn build(label_ids: Vec<i32>, scores: Vec<f32>) -> Self {
        Self {
            label_ids,
            scores,
            type_: ResultType::CLASSIFY,
        }
    }
}

impl Default for ClassifyResult {
    fn default() -> Self {
        Self {
            label_ids: vec![],
            scores: vec![],
            type_: ResultType::CLASSIFY,
        }
    }
}

impl From<FD_C_ClassifyResult> for ClassifyResult {
    fn from(mut value: FD_C_ClassifyResult) -> Self {
        unsafe {
            Self {
                label_ids: fd_c_one_dim_array_int32_to_vec_i32(value.label_ids),
                scores: fd_c_one_dim_array_float_to_vec_float(value.scores),
                type_: ResultType::from_c_type(value.type_),
            }
        }
    }
}

pub struct ClassifyResultWrapper {
    pub ptr: *mut FD_C_ClassifyResult,
}

impl ClassifyResultWrapper {
    pub fn new() -> ClassifyResultWrapper {
        unsafe {
            ClassifyResultWrapper {
                ptr: FD_C_CreateClassifyResult(),
            }
        }
    }
}

impl Drop for ClassifyResultWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyClassifyResult(self.ptr);
        }
    }
}

pub struct OneDimClassifyResultWrapper {
    pub ptr: *mut FD_C_OneDimClassifyResult,
}

impl OneDimClassifyResultWrapper {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: FD_C_CreateOneDimClassifyResult(),
            }
        }
    }
}

impl Drop for OneDimClassifyResultWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimClassifyResult(self.ptr);
        }
    }
}


pub struct DetectResultWrapper {
    pub(crate) ptr: *mut FD_C_DetectionResult,
}

impl DetectResultWrapper {
    pub unsafe fn new() -> Self {
        Self {
            ptr: FD_C_CreateDetectionResult(),
        }
    }

    pub unsafe fn from_ptr(ptr: detect_result_t) -> Self {
        Self {
            ptr,
        }
    }
}

impl Drop for DetectResultWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyDetectionResult(self.ptr);
        }
    }
}

pub struct OneDimDetectResultWrapper {
    pub ptr: *mut FD_C_OneDimDetectionResult,
}

impl OneDimDetectResultWrapper {
    pub unsafe fn new() -> Self {
        Self {
            ptr: FD_C_CreateOneDimDetectionResult(),
        }
    }
}

impl Drop for OneDimDetectResultWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimDetectionResult(self.ptr);
        }
    }
}


#[derive(Debug, Clone)]
struct Mask {
    pub data: Vec<u8>,
    pub shape: Vec<i64>,
    pub type_: ResultType,
}

impl Mask {}

impl Default for Mask {
    fn default() -> Self {
        Self {
            data: vec![],
            shape: vec![],
            type_: ResultType::UNKNOWN_RESULT,
        }
    }
}

impl From<FD_C_Mask> for Mask {
    fn from(value: FD_C_Mask) -> Self {
        Self {
            data: fd_c_one_dim_array_uint8_to_vec_u8(value.data),
            shape: fd_c_one_dim_array_int64_to_vec_i64(value.shape),
            type_: ResultType::from_c_type(value.type_),
        }
    }
}

pub fn fd_c_two_dim_mask_to_vec_mask(masks: FD_C_OneDimMask) -> Vec<Mask> {
    if masks.data.is_null() {
        return vec![];
    }
    unsafe {
        let mut result = vec![];
        for i in 0..masks.size {
            result.push(Mask::from(*masks.data.wrapping_add(i)))
        }
        return result;
    }
}

#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub boxes: Vec<Vec<f32>>,
    pub rotated_boxes: Vec<Vec<f32>>,
    pub scores: Vec<f32>,
    pub label_ids: Vec<i32>,
    pub masks: Vec<Mask>,
    pub contain_masks: bool,
    pub type_: ResultType,
}

impl DetectionResult {}

impl From<FD_C_DetectionResult> for DetectionResult {
    fn from(mut value: FD_C_DetectionResult) -> Self {
        Self {
            boxes: fd_c_two_dim_array_float_to_vec_float(value.boxes),
            rotated_boxes: fd_c_two_dim_array_float_to_vec_float(value.rotated_boxes),
            scores: fd_c_one_dim_array_float_to_vec_float(value.scores),
            label_ids: fd_c_one_dim_array_int32_to_vec_i32(value.label_ids),
            masks: fd_c_two_dim_mask_to_vec_mask(value.masks),
            contain_masks: fd_c_bool_to_bool(value.contain_masks),
            type_: ResultType::from_c_type(value.type_),
        }
    }
}

impl Default for DetectionResult {
    fn default() -> Self {
        Self {
            boxes: vec![],
            rotated_boxes: vec![],
            scores: vec![],
            label_ids: vec![],
            masks: vec![],
            contain_masks: false,
            type_: ResultType::UNKNOWN_RESULT,
        }
    }
}

#[derive(Debug)]
pub struct RecognizerResult {
    pub text: String,
    pub score: f32,
}

impl RecognizerResult {
    pub fn new(text: String, score: f32) -> Self {
        Self {
            text,
            score,
        }
    }
}


pub struct OcrResultWrapper {
    pub ptr: Box<FD_C_OCRResult>,
}

impl Default for OcrResultWrapper {
    fn default() -> Self {
        unsafe {
            Self {
                ptr: Box::new(*FD_C_CreateOCRResult()),
            }
        }
    }
}

impl Drop for OcrResultWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOCRResult(self.ptr.as_mut());
        }
    }
}

pub struct OneDimOcrResultWrapper {
    pub ptr: Box<FD_C_OneDimOCRResult>,
}

impl OneDimOcrResultWrapper {}

impl Default for OneDimOcrResultWrapper {
    fn default() -> Self {
        unsafe {
            Self {
                ptr: Box::new(FD_C_OneDimOCRResult { size: 0, data: OcrResultWrapper::default().ptr.as_mut() }),
            }
        }
    }
}

impl Drop for OneDimOcrResultWrapper {
    fn drop(&mut self) {
        // FD_C_DestroyOneDimOcrResult(self.ptr.as_mut());
    }
}


pub struct SegmentationResult {
    pub ptr: *mut FD_C_SegmentationResult,
}

impl SegmentationResult {
    pub fn new() -> SegmentationResult {
        unsafe {
            SegmentationResult {
                ptr: FD_C_CreateSegmentationResult(),
            }
        }
    }
    pub fn str(&self) -> &str {
        unsafe {
            let s: &str = &String::from_utf8(vec![0x01; 10240]).unwrap();
            let c = CString::new(s).unwrap().into_raw();
            FD_C_SegmentationResultStr(self.ptr, c);
            return CStr::from_ptr(c).to_str().unwrap();
        }
    }
}

impl Drop for SegmentationResult {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroySegmentationResult(self.ptr);
        }
    }
}

pub struct OneDimSegmentationResult {
    pub ptr: *mut FD_C_OneDimSegmentationResult,
}

impl OneDimSegmentationResult {
    pub fn new(data: &mut Vec<SegmentationResult>) -> OneDimSegmentationResult {
        unsafe {
            let mut c = FD_C_OneDimSegmentationResult { size: data.len(), data: (*data.as_mut_ptr()).ptr };
            OneDimSegmentationResult {
                ptr: &mut c as *mut FD_C_OneDimSegmentationResult,
            }
        }
    }
}


