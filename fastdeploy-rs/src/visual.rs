use fastdeploy_bind::*;

pub mod detection {
    use crate::result::DetectionResult;
    use crate::type_bridge::Mat;

    use super::*;

    pub fn vis_detection(img: &Mat, result: &DetectionResult, score_threshold: f32, line_size: i32, font_size: f32) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_VisDetection(img.ptr, result.ptr, score_threshold, line_size, font_size),
            }
        }
    }
}