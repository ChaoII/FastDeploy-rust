#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString};

use fastdeploy_bind::*;

use crate::enum_variables::ResultType;
use crate::type_bridge::common::*;

#[derive(Debug)]
pub struct ClassifyResult {
    pub ptr: *mut FD_C_ClassifyResult,
}

pub struct OneDimClassifyResult {
    pub ptr: *mut FD_C_OneDimClassifyResult,
}


impl ClassifyResult {
    pub fn new() -> ClassifyResult {
        unsafe {
            ClassifyResult {
                ptr: FD_C_CreateClassifyResult(),
            }
        }
    }
    pub fn str(&self) -> &str {
        unsafe {
            let s: &str = &String::from_utf8(vec![0x01; 10240]).unwrap();
            let c = CString::new(s).unwrap().into_raw();
            FD_C_ClassifyResultStr(self.ptr, c);
            return CStr::from_ptr(c).to_str().unwrap();
        }
    }
}

impl OneDimClassifyResult {
    pub fn build(data: &mut Vec<ClassifyResult>) -> Self {
        unsafe {
            let mut c = FD_C_OneDimClassifyResult {
                size: data.len(),
                data: (*data.as_mut_ptr()).ptr,
            };
            return OneDimClassifyResult {
                ptr: &mut c as *mut FD_C_OneDimClassifyResult,
            };
        }
    }
}

impl Drop for ClassifyResult {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyClassifyResult(self.ptr);
        }
    }
}

#[derive(Debug)]
pub struct DetectionResult {
    pub boxes: Vec<Vec<f32>>,
    pub rotated_boxes: Vec<Vec<f32>>,
    pub scores: Vec<f32>,
    pub label_ids: Vec<i32>,
    pub masks: Vec<Mask1>,
    pub contain_masks: bool,
    pub type_: ResultType,
    pub ptr: *mut FD_C_DetectionResult,
}

#[derive(Debug)]
struct Mask1 {
    pub data: Vec<u8>,
    pub shape: Vec<i64>,
    pub type_: ResultType,
}

impl Mask1 {
    pub fn new() -> Mask1 {
        Mask1 {
            data: vec![],
            shape: vec![],
            type_: ResultType::UNKNOWN_RESULT,
        }
    }
    pub fn from_c_ptr(mask: FD_C_Mask) -> Mask1 {
        Mask1 {
            data: fd_c_one_dim_array_uint8_to_vec_u8(mask.data),
            shape: fd_c_one_dim_array_int64_to_vec_i64(mask.shape),
            type_: ResultType::from_c_type(mask.type_),
        }
    }
}

pub fn fd_c_two_dim_mask_to_vec_mask(masks: FD_C_OneDimMask) -> Vec<Mask1> {
    if masks.data.is_null() {
        return vec![];
    }
    unsafe {
        let mut result = vec![];
        for i in 0..masks.size {
            result.push(Mask1::from_c_ptr(*masks.data.wrapping_add(i)))
        }
        return result;
    }
}

impl DetectionResult {
    pub fn new() -> DetectionResult {
        DetectionResult {
            boxes: vec![],
            rotated_boxes: vec![],
            scores: vec![],
            label_ids: vec![],
            masks: vec![],
            contain_masks: false,
            type_: ResultType::UNKNOWN_RESULT,
            ptr: unsafe { FD_C_CreateDetectionResult() },
        }
    }
    pub fn from_c_ptr(c_detection_result: *mut FD_C_DetectionResult) -> DetectionResult {
        if c_detection_result.is_null() {
            return DetectionResult::new();
        }
        let mut detection_result = DetectionResult::new();
        unsafe {
            detection_result.boxes = fd_c_two_dim_array_float_to_vec_float(&mut (*c_detection_result).boxes);
            detection_result.rotated_boxes = fd_c_two_dim_array_float_to_vec_float(&mut (*c_detection_result).rotated_boxes);
            detection_result.scores = fd_c_one_dim_array_float_to_vec_float((*c_detection_result).scores);
            detection_result.label_ids = fd_c_one_dim_array_int32_to_vec_i32((*c_detection_result).label_ids);
            detection_result.masks = fd_c_two_dim_mask_to_vec_mask((*c_detection_result).masks);
            detection_result.contain_masks = fd_c_bool_to_bool((*c_detection_result).contain_masks);
            detection_result.type_ = ResultType::from_c_type((*c_detection_result).type_);
            detection_result.ptr = c_detection_result;
        }
        return detection_result;
    }

    pub fn convert_c_array_to_rust_slice(c_detection_result: *mut FD_C_OneDimDetectionResult) -> Vec<DetectionResult> {
        unsafe {
            if c_detection_result.is_null() || (*c_detection_result).data.is_null() {
                return vec![];
            }

            let mut detection_results = vec![];

            for i in 0..(*c_detection_result).size {
                println!("{}", i);
                detection_results.push(DetectionResult::from_c_ptr((*c_detection_result).data.wrapping_add(i)));
            }

            return detection_results;
        }
    }
}

// impl Drop for DetectionResult {
//     fn drop(&mut self) {
//         unsafe {
//             if self.ptr.is_null() {
//                 return;
//             }
//             FD_C_DestroyDetectionResult(self.ptr);
//             self.ptr = std::ptr::null_mut();
//         }
//     }
// }


pub struct OcrResult {
    pub ptr: *mut FD_C_OCRResult,
}

impl OcrResult {
    pub fn new() -> OcrResult {
        unsafe {
            OcrResult {
                ptr: FD_C_CreateOCRResult(),
            }
        }
    }
    pub fn str(&self) -> &str {
        unsafe {
            let s: &str = &String::from_utf8(vec![0x01; 10240]).unwrap();
            let c = CString::new(s).unwrap().into_raw();
            FD_C_OCRResultStr(self.ptr, c);
            return CStr::from_ptr(c).to_str().unwrap();
        }
    }
}

impl Drop for OcrResult {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOCRResult(self.ptr);
        }
    }
}

pub struct OneDimOcrResult {
    pub ptr: *mut FD_C_OneDimOCRResult,
}

impl OneDimOcrResult {
    pub fn new(data: &mut Vec<OcrResult>) -> OneDimOcrResult {
        unsafe {
            let mut c = FD_C_OneDimOCRResult { size: data.len(), data: (*data.as_mut_ptr()).ptr };
            OneDimOcrResult {
                ptr: &mut c as *mut FD_C_OneDimOCRResult,
            }
        }
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


