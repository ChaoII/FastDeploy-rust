use fastdeploy_bind::*;
use fastdeploy_bind::{FD_C_VisDetection, FD_C_VisDetectionWithLabel};
use fastdeploy_bind::{FD_C_VisClassification, FD_C_VisClassificationWithLabel};

use crate::result::ClassifyResult;
use crate::result::DetectionResult;
use crate::type_bridge::common::vec_string_to_fd_one_dim_array_c_str;
use crate::type_bridge::Mat;

pub mod detection {
    use super::*;

    pub fn vis_detection(img: &Mat, result: &DetectionResult, score_threshold: f32, line_size: i32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisDetection(img.ptr, &mut result.to_raw_ptr(), score_threshold, line_size, font_size),
            }
        }
    }

    pub fn vis_detection_with_label(img: &Mat, result: &DetectionResult, labels: Vec<String>, score_threshold: f32, line_size: i32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisDetectionWithLabel(img.ptr, &mut result.to_raw_ptr(), &mut vec_string_to_fd_one_dim_array_c_str(labels), score_threshold, line_size, font_size),
            }
        }
    }
}

pub mod classify {
    use super::*;

    pub fn vis_classify(img: &Mat, result: &ClassifyResult, top_k: i32, score_threshold: f32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisClassification(img.ptr, &mut result.to_raw_ptr(), top_k, score_threshold, font_size),
            }
        }
    }

    pub fn vis_detection_with_label(img: &Mat, result: &ClassifyResult, labels: Vec<String>, top_k: i32, score_threshold: f32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisClassificationWithLabel(img.ptr, &mut result.to_raw_ptr(), &mut vec_string_to_fd_one_dim_array_c_str(labels), top_k, score_threshold, font_size),
            }
        }
    }
}

// pub mod ocr {
//     use crate::result::OcrResult;
//
//     use super::*;
//
//     pub fn vis_classify(img: &Mat, result: &OcrResult) -> Mat {
//         unsafe {
//             Mat {
//                 ptr: FD_C_VisOcr(img.ptr, result.ptr),
//             }
//         }
//     }
// }

// pub mod segmentation {
//     use crate::result::SegmentationResult;
//
//     use super::*;
//
//     pub fn vis_segmentation(img: &Mat, result: &SegmentationResult, weight: f32) -> Mat {
//         unsafe {
//             Mat {
//                 ptr: FD_C_VisSegmentation(img.ptr, result.ptr, weight),
//             }
//         }
//     }
// }