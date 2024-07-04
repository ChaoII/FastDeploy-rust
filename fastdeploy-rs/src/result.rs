#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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

impl From<FD_C_ClassifyResult> for ClassifyResult {
    fn from(value: FD_C_ClassifyResult) -> Self {
        Self {
            label_ids: c_1_int32_to_vec(value.label_ids),
            scores: c_1_float_to_vec(value.scores),
            type_: ResultType::from(value.type_),
        }
    }
}

impl Into<FD_C_ClassifyResult> for ClassifyResult {
    fn into(self) -> FD_C_ClassifyResult {
        FD_C_ClassifyResult {
            label_ids: vec_to_c_1_int32(self.label_ids.clone()),
            scores: vec_to_c_1_float(self.scores.clone()),
            type_: self.type_ as FD_C_ResultType,
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
            label_map: c_1_uint8_to_vec(value.label_map),
            score_map: c_1_float_to_vec(value.score_map),
            shape: c_1_int64_to_vec(value.shape),
            contain_score_map: c_bool_to_bool(value.contain_score_map),
            type_: ResultType::from(value.type_),
        }
    }
}

impl Into<FD_C_SegmentationResult> for SegmentationResult {
    fn into(self) -> FD_C_SegmentationResult {
        FD_C_SegmentationResult {
            label_map: vec_to_c_1_uint8(self.label_map),
            score_map: vec_to_c_1_float(self.score_map),
            shape: vec_to_c_1_int64(self.shape),
            contain_score_map: bool_to_c_bool(self.contain_score_map),
            type_: self.type_ as FD_C_ResultType,
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

pub struct OneDimDetectResult {
    pub ptr: *mut FD_C_OneDimDetectionResult,
}

impl OneDimDetectResult {
    pub unsafe fn new() -> Self {
        Self {
            ptr: FD_C_CreateOneDimDetectionResult(),
        }
    }
}

impl Drop for OneDimDetectResult {
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
            data: vec_to_c_1_uint8(self.data),
            shape: vec_to_c_1_int64(self.shape),
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
            data: c_1_uint8_to_vec(value.data),
            shape: c_1_int64_to_vec(value.shape),
            type_: ResultType::from(value.type_),
        }
    }
}

impl Into<FD_C_Mask> for Mask {
    fn into(self) -> FD_C_Mask {
        FD_C_Mask {
            data: vec_to_c_1_uint8(self.data),
            shape: vec_to_c_1_int64(self.shape),
            type_: self.type_ as FD_C_ResultType,
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
    let mut s = Vec::with_capacity(masks.len());
    for i in 0..masks.len() {
        s.push(masks[i].clone().to_raw_ptr());
    }
    return FD_C_OneDimMask {
        size: masks.len(),
        data: s.as_mut_ptr(),
    };
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


impl Into<FD_C_DetectionResult> for DetectionResult {
    fn into(self) -> FD_C_DetectionResult {
        unsafe {
            // 转换 masks
            let mut msk: Vec<FD_C_Mask> = self.masks.clone().into_iter().map(|w| w.into()).collect();
            let masks = FD_C_OneDimMask { data: msk.as_mut_ptr(), size: self.masks.len() };
            // 构造 FD_C_DetectionResult
            let detection_result = FD_C_DetectionResult {
                boxes: vec_to_c_2_float(self.boxes),
                rotated_boxes: vec_to_c_2_float(self.rotated_boxes),
                scores: vec_to_c_1_float(self.scores),
                label_ids: vec_to_c_1_int32(self.label_ids),
                masks,
                contain_masks: bool_to_c_bool(self.contain_masks),
                type_: self.type_ as FD_C_ResultType,
            };
            detection_result
        }
    }
}

impl From<FD_C_DetectionResult> for DetectionResult {
    fn from(value: FD_C_DetectionResult) -> Self {
        Self {
            boxes: c_2_float_to_vec(value.boxes),
            rotated_boxes: c_2_float_to_vec(value.rotated_boxes),
            scores: c_1_float_to_vec(value.scores),
            label_ids: c_1_int32_to_vec(value.label_ids),
            masks: fd_c_two_dim_mask_to_vec_mask(value.masks),
            contain_masks: c_bool_to_bool(value.contain_masks),
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

impl DetectionResult {
    pub fn release_raw_ptr(result: FD_C_DetectionResult) {
        unsafe {
            // 重新将裸指针转换为 Box，并自动释放内存
            let _boxes = Box::from_raw(&result.boxes as *const _ as *mut FD_C_OneDimArrayFloat);
            let _rotated_boxes = Box::from_raw(&result.rotated_boxes as *const _ as *mut FD_C_OneDimArrayFloat);
            let _scores = Box::from_raw(&result.scores as *const _ as *mut FD_C_OneDimArrayFloat);
            let _label_ids = Box::from_raw(&result.label_ids as *const _ as *mut FD_C_OneDimArrayInt32);
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
    fn from(value: FD_C_OCRResult) -> Self {
        Self {
            boxes: c_2_int32_to_vec(value.boxes),
            text: c_1_str_to_vec(value.text),
            rec_scores: c_1_float_to_vec(value.rec_scores),
            cls_scores: c_1_float_to_vec(value.cls_scores),
            cls_labels: c_1_int32_to_vec(value.cls_labels),
            table_boxes: c_2_int32_to_vec(value.table_boxes),
            table_structure: c_1_str_to_vec(value.table_structure),
            table_html: cstr_to_string(value.table_html),
            type_: ResultType::from(value.type_),
        }
    }
}

impl Into<FD_C_OCRResult> for OCRResult {
    fn into(self) -> FD_C_OCRResult {
        FD_C_OCRResult {
            boxes: vec_to_c_2_int32(self.boxes),
            text: vec_to_c_1_cstr(self.text),
            rec_scores: vec_to_c_1_float(self.rec_scores),
            cls_scores: vec_to_c_1_float(self.cls_scores),
            cls_labels: vec_to_c_1_int32(self.cls_labels),
            table_boxes: vec_to_c_2_int32(self.table_boxes),
            table_structure: vec_to_c_1_cstr(self.table_structure),
            table_html: string_to_cstr(&self.table_html),
            type_: self.type_ as FD_C_ResultType,
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
            FD_C_DestroyOCRResult(self.ptr);
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
    pub ptr: *mut FD_C_OneDimSegmentationResult,
}

impl OneDimSegmentationResult {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: FD_C_CreateOneDimSegmentationResult(),
            }
        }
    }
}

impl Drop for OneDimSegmentationResult {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimSegmentationResult(self.ptr);
        }
    }
}


