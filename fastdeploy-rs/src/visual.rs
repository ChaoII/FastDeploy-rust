use fastdeploy_bind::*;

use crate::type_bridge::{Mat, OneDimArrayCstr};

pub mod detection {
    use crate::result::DetectionResult;

    use super::*;

    pub fn vis_detection(img: &Mat, result: &DetectionResult, score_threshold: f32, line_size: i32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisDetection(img.ptr, result.ptr, score_threshold, line_size, font_size),
            }
        }
    }

    pub fn vis_detection_with_label(img: &Mat, result: &DetectionResult, labels: &mut Vec<&str>, score_threshold: f32, line_size: i32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisDetectionWithLabel(img.ptr, result.ptr, OneDimArrayCstr::build(labels).ptr, score_threshold, line_size, font_size),
            }
        }
    }
}

pub mod classify {
    use crate::result::ClassifyResult;

    use super::*;

    pub fn vis_classify(img: &Mat, result: &ClassifyResult, top_k: i32, score_threshold: f32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisClassification(img.ptr, result.ptr, top_k, score_threshold, font_size),
            }
        }
    }

    pub fn vis_detection_with_label(img: &Mat, result: &ClassifyResult, labels: &mut Vec<&str>, top_k: i32, score_threshold: f32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisClassificationWithLabel(img.ptr, result.ptr, OneDimArrayCstr::build(labels).ptr, top_k, score_threshold, font_size),
            }
        }
    }
}

pub mod ocr {
    use crate::result::OcrResult;

    use super::*;

    pub fn vis_classify(img: &Mat, result: &OcrResult) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisOcr(img.ptr, result.ptr),
            }
        }
    }
}

pub mod segmentation {
    use crate::result::SegmentationResult;

    use super::*;

    pub fn vis_segmentation(img: &Mat, result: &SegmentationResult, weight: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisSegmentation(img.ptr, result.ptr, weight),
            }
        }
    }
}