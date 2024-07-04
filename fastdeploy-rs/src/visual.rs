use fastdeploy_bind::{FD_C_VisDetection, FD_C_VisDetectionWithLabel};
use fastdeploy_bind::{FD_C_VisClassification, FD_C_VisClassificationWithLabel};
use fastdeploy_bind::FD_C_VisOcr;
use fastdeploy_bind::FD_C_VisSegmentation;

use crate::result::ClassifyResult;
use crate::result::DetectionResult;
use crate::result::OCRResult;
use crate::result::SegmentationResult;
use crate::type_bridge::common::vec_to_c_1_cstr;
use crate::type_bridge::Mat;

pub mod detection {
    use fastdeploy_bind::FD_C_DetectionResult;

    use super::*;

    pub fn vis_detection(img: &Mat, result: DetectionResult, score_threshold: f32, line_size: i32, font_size: f32) -> Mat {
        unsafe {
            let mut a = result.clone().into();
            let s = &mut a as *mut FD_C_DetectionResult;
            let c = FD_C_VisDetection(img.ptr, s, score_threshold, line_size, font_size);
            // DetectionResult::release_raw_ptr(a);
            Mat {
                ptr: c,
            }
        }
    }

    pub fn vis_detection_with_label(img: &Mat, result: DetectionResult, labels: Vec<String>, score_threshold: f32, line_size: i32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisDetectionWithLabel(img.ptr, &mut result.into(), &mut vec_to_c_1_cstr(labels), score_threshold, line_size, font_size),
            }
        }
    }
}

pub mod classify {
    use super::*;

    pub fn vis_classify(img: &Mat, result: ClassifyResult, top_k: i32, score_threshold: f32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisClassification(img.ptr, &mut result.into(), top_k, score_threshold, font_size),
            }
        }
    }

    pub fn vis_classify_with_label(img: &Mat, result: ClassifyResult, labels: Vec<String>, top_k: i32, score_threshold: f32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisClassificationWithLabel(img.ptr, &mut result.into(), &mut vec_to_c_1_cstr(labels), top_k, score_threshold, font_size),
            }
        }
    }
}

pub mod ocr {
    use super::*;

    pub fn vis_ocr(img: &Mat, result: OCRResult) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisOcr(img.ptr, &mut result.into()),
            }
        }
    }
}

pub mod segmentation {
    use fastdeploy_bind::FD_C_SegmentationResult;
    use super::*;

    pub fn vis_segmentation(img: &Mat, result: SegmentationResult, weight: f32) -> Mat {
        unsafe {
            let mut a = result.clone().into();
            let s = &mut a as *mut FD_C_SegmentationResult;
            Mat {
                ptr: FD_C_VisSegmentation(img.ptr, s, weight),
            }
        }
    }
}