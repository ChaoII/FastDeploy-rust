#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use fastdeploy_bind::*;

use crate::enum_variables::ResultType;
use crate::type_bridge::common::*;
use crate::type_bridge::CstrWrapper;

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
    pub fn to_raw_ptr(&self) -> FD_C_ClassifyResult {
        FD_C_ClassifyResult {
            label_ids: vec_i32_to_fd_c_one_dim_array_int32(self.label_ids.clone()),
            scores: vec_f32_to_fd_c_one_dim_array_float(self.scores.clone()),
            type_: self.type_ as FD_C_ResultType,
        }
    }
}

impl From<FD_C_ClassifyResult> for ClassifyResult {
    fn from(mut value: FD_C_ClassifyResult) -> Self {
        unsafe {
            Self {
                label_ids: fd_c_one_dim_array_int32_to_vec_i32(value.label_ids),
                scores: fd_c_one_dim_array_float_to_vec_f32(value.scores),
                type_: ResultType::from(value.type_),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SegmentationResult {
    pub label_map: Vec<u8>,
    pub score_map: Vec<f32>,
    pub shape: Vec<i64>,
    pub contain_score_map: bool,
    pub type_: ResultType,
}

impl From<FD_C_SegmentationResult> for SegmentationResult {
    fn from(value: FD_C_SegmentationResult) -> Self {
        Self {
            label_map: fd_c_one_dim_array_uint8_to_vec_u8(value.label_map),
            score_map: fd_c_one_dim_array_float_to_vec_f32(value.score_map),
            shape: fd_c_one_dim_array_int64_to_vec_i64(value.shape),
            contain_score_map: fd_c_bool_to_bool(value.contain_score_map),
            type_: ResultType::from(value.type_),
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
    #[inline]
    fn drop(&mut self) {
        unsafe { FD_C_DestroyOneDimDetectionResult(self.ptr); }
    }
}


#[derive(Debug, Clone)]
struct Mask {
    pub data: Vec<u8>,
    pub shape: Vec<i64>,
    pub type_: ResultType,
}

impl Mask {
    pub fn to_raw_ptr(self) -> FD_C_Mask {
        FD_C_Mask {
            data: vec_u8_to_fd_c_one_dim_array_uint8(self.data),
            shape: vec_i64_to_fd_c_one_dim_array_int64(self.shape),
            type_: self.type_ as FD_C_ResultType,
        }
    }
}

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
            type_: ResultType::from(value.type_),
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

pub fn vec_mask_to_fd_c_two_dim_mask(masks: Vec<Mask>) -> FD_C_OneDimMask {
    unsafe {
        let mut s = Vec::with_capacity(masks.len());
        for i in 0..masks.len() {
            s.push(masks[i].clone().to_raw_ptr());
        }
        return FD_C_OneDimMask {
            size: masks.len(),
            data: s.as_mut_ptr(),
        };
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


impl DetectionResult {
    pub fn to_raw_ptr(&self) -> FD_C_DetectionResult {
        FD_C_DetectionResult {
            boxes: vec_f32_to_fd_c_two_dim_array_float(self.boxes.clone()),
            rotated_boxes: vec_f32_to_fd_c_two_dim_array_float(self.rotated_boxes.clone()),
            scores: vec_f32_to_fd_c_one_dim_array_float(self.scores.clone()),
            label_ids: vec_i32_to_fd_c_one_dim_array_int32(self.label_ids.clone()),
            masks: vec_mask_to_fd_c_two_dim_mask(self.masks.clone()),
            contain_masks: bool_to_fd_c_bool(self.contain_masks.clone()),
            type_: self.type_ as FD_C_ResultType,
        }
    }
}

impl From<FD_C_DetectionResult> for DetectionResult {
    fn from(mut value: FD_C_DetectionResult) -> Self {
        Self {
            boxes: fd_c_two_dim_array_float_to_vec_float(value.boxes),
            rotated_boxes: fd_c_two_dim_array_float_to_vec_float(value.rotated_boxes),
            scores: fd_c_one_dim_array_float_to_vec_f32(value.scores),
            label_ids: fd_c_one_dim_array_int32_to_vec_i32(value.label_ids),
            masks: fd_c_two_dim_mask_to_vec_mask(value.masks),
            contain_masks: fd_c_bool_to_bool(value.contain_masks),
            type_: ResultType::from(value.type_),
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

#[derive(Debug, Clone)]
pub struct OCRResult {
    pub boxes: Vec<Vec<i32>>,
    pub text: Vec<String>,
    pub rec_scores: Vec<f32>,
    pub cls_scores: Vec<f32>,
    pub cls_labels: Vec<i32>,
    pub table_boxes: Vec<Vec<i32>>,
    pub table_structure: Vec<String>,
    pub table_html: String,
    pub type_: ResultType,
}

impl From<FD_C_OCRResult> for OCRResult {
    fn from(mut value: FD_C_OCRResult) -> Self {
        Self {
            boxes: fd_c_two_dim_array_int32_to_vec_i32(value.boxes),
            text: fd_one_dim_array_c_str_to_vec_string(value.text),
            rec_scores: fd_c_one_dim_array_float_to_vec_f32(value.rec_scores),
            cls_scores: fd_c_one_dim_array_float_to_vec_f32(value.cls_scores),
            cls_labels: fd_c_one_dim_array_int32_to_vec_i32(value.cls_labels),
            table_boxes: fd_c_two_dim_array_int32_to_vec_i32(value.table_boxes),
            table_structure: fd_one_dim_array_c_str_to_vec_string(value.table_structure),
            table_html: CstrWrapper::from(value.table_html).to_str().unwrap().to_string(),
            type_: ResultType::from(value.type_),
        }
    }
}


pub struct OcrResultWrapper {
    pub ptr: *mut FD_C_OCRResult,
}

impl OcrResultWrapper {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: FD_C_CreateOCRResult(),
            }
        }
    }
}

impl Drop for OcrResultWrapper {
    fn drop(&mut self) {
        println!("delete ocr result");
        unsafe {
            // FD_C_DestroyOCRResult(self.ptr);
        }
    }
}

pub struct OneDimOcrResultWrapper {
    pub ptr: *mut FD_C_OneDimOCRResult,
}

impl OneDimOcrResultWrapper {
    pub fn new() -> Self {
        Self {
            ptr: Self::default().ptr,
        }
    }
}

impl Default for OneDimOcrResultWrapper {
    fn default() -> Self {
        unsafe {
            Self {
                ptr: FD_C_CreateOneDimOCRResult(),
            }
        }
    }
}

impl Drop for OneDimOcrResultWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimOCRResult(self.ptr);
        }
    }
}


pub struct SegmentationResultWrapper {
    pub ptr: *mut FD_C_SegmentationResult,
}

impl SegmentationResultWrapper {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: FD_C_CreateSegmentationResult(),
            }
        }
    }
}

impl Drop for SegmentationResultWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroySegmentationResult(self.ptr);
        }
    }
}

pub struct OneDimSegmentationResult {
    pub ptr: Box<FD_C_OneDimSegmentationResult>,
}

impl OneDimSegmentationResult {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: Box::new(FD_C_OneDimSegmentationResult { size: 0, data: SegmentationResultWrapper::new().ptr }),
            }
        }
    }
}


