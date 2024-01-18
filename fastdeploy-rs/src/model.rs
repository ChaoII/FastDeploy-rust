#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

use fastdeploy_bind::*;

use crate::enum_variables::ModelFormat;
use crate::result::{ClassifyResult, DetectionResult, OcrResult, OneDimClassifyResult, OneDimOcrResult, OneDimSegmentationResult, SegmentationResult};
use crate::runtime_option::RuntimeOption;
use crate::type_bridge::{Cstr, Mat, OneDimArrayCstr, OneDimArrayFloat, OneDimArrayInt32, OneDimMat, ThreeDimArrayInt32, TwoDimArrayCstr, TwoDimArrayInt32};

pub struct PaddleClasModel {
    ptr: *mut FD_C_PaddleClasModelWrapper,
}

impl PaddleClasModel {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> PaddleClasModel {
        unsafe {
            return PaddleClasModel {
                ptr: FD_C_CreatePaddleClasModelWrapper(CString::new(model_file).unwrap().into_raw(),
                                                       CString::new(param_file).unwrap().into_raw(),
                                                       CString::new(config_file).unwrap().into_raw(),
                                                       runtime_option.ptr,
                                                       model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, classify_result: &ClassifyResult) {
        unsafe {
            FD_C_PaddleClasModelWrapperPredict(self.ptr, img.ptr, classify_result.ptr);
        }
    }
    pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<ClassifyResult>) {
        unsafe {
            FD_C_PaddleClasModelWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimClassifyResult::build(detect_result).ptr);
        }
    }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PaddleClasModelWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for PaddleClasModel {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyPaddleClasModelWrapper(self.ptr);
        }
    }
}


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
    pub fn batch_predict(&self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
        unsafe {
            let mut s: Vec<FD_C_Mat> = imgs.iter().map(|x| x.ptr).collect();
            let s = FD_C_OneDimMat { size: imgs.len(), data: s.as_mut_ptr() };
            let result = &mut FD_C_OneDimDetectionResult { size: 0, data: std::ptr::null_mut() } as *mut FD_C_OneDimDetectionResult;
            let ret = FD_C_PPYOLOEWrapperBatchPredict(self.ptr, s, result);
            let rr = DetectionResult::convert_c_array_to_rust_slice(result);
            // println!("{:#?}", rr);
        }
    }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PPYOLOEWrapperInitialized(self.ptr) != 0
        }
    }
}

// impl Drop for PPYOLOE {
//     fn drop(&mut self) {
//         unsafe { FD_C_DestroyPPYOLOEWrapper(self.ptr); }
//     }
// }


pub struct PicoDet {
    ptr: *mut FD_C_PicoDetWrapper,
}

impl PicoDet {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return PicoDet {
                ptr: FD_C_CreatePicoDetWrapper(CString::new(model_file).unwrap().into_raw(),
                                               CString::new(param_file).unwrap().into_raw(),
                                               CString::new(config_file).unwrap().into_raw(),
                                               runtime_option.ptr,
                                               model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_PicoDetWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_PicoDetWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PicoDetWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for PicoDet {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyPicoDetWrapper(self.ptr); }
    }
}

pub struct PPYOLO {
    ptr: *mut FD_C_PPYOLOWrapper,
}

impl PPYOLO {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return PPYOLO {
                ptr: FD_C_CreatePPYOLOWrapper(CString::new(model_file).unwrap().into_raw(),
                                              CString::new(param_file).unwrap().into_raw(),
                                              CString::new(config_file).unwrap().into_raw(),
                                              runtime_option.ptr,
                                              model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_PPYOLOWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_PPYOLOWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PPYOLOWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for PPYOLO {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyPPYOLOWrapper(self.ptr); }
    }
}

pub struct YOLOv3 {
    ptr: *mut FD_C_YOLOv3Wrapper,
}

impl YOLOv3 {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return YOLOv3 {
                ptr: FD_C_CreateYOLOv3Wrapper(CString::new(model_file).unwrap().into_raw(),
                                              CString::new(param_file).unwrap().into_raw(),
                                              CString::new(config_file).unwrap().into_raw(),
                                              runtime_option.ptr,
                                              model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_YOLOv3WrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_YOLOv3WrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_YOLOv3WrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for YOLOv3 {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyYOLOv3Wrapper(self.ptr); }
    }
}

pub struct PaddleYOLOX {
    ptr: *mut FD_C_PaddleYOLOXWrapper,
}

impl PaddleYOLOX {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return PaddleYOLOX {
                ptr: FD_C_CreatePaddleYOLOXWrapper(CString::new(model_file).unwrap().into_raw(),
                                                   CString::new(param_file).unwrap().into_raw(),
                                                   CString::new(config_file).unwrap().into_raw(),
                                                   runtime_option.ptr,
                                                   model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_PaddleYOLOXWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_PaddleYOLOXWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PaddleYOLOXWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for PaddleYOLOX {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyPaddleYOLOXWrapper(self.ptr); }
    }
}

pub struct FasterRCNN {
    ptr: *mut FD_C_FasterRCNNWrapper,
}

impl FasterRCNN {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return FasterRCNN {
                ptr: FD_C_CreateFasterRCNNWrapper(CString::new(model_file).unwrap().into_raw(),
                                                  CString::new(param_file).unwrap().into_raw(),
                                                  CString::new(config_file).unwrap().into_raw(),
                                                  runtime_option.ptr,
                                                  model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_FasterRCNNWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_FasterRCNNWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_FasterRCNNWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for FasterRCNN {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyFasterRCNNWrapper(self.ptr); }
    }
}

pub struct MaskRCNN {
    ptr: *mut FD_C_MaskRCNNWrapper,
}

impl MaskRCNN {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return MaskRCNN {
                ptr: FD_C_CreateMaskRCNNWrapper(CString::new(model_file).unwrap().into_raw(),
                                                CString::new(param_file).unwrap().into_raw(),
                                                CString::new(config_file).unwrap().into_raw(),
                                                runtime_option.ptr,
                                                model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_MaskRCNNWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_MaskRCNNWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_MaskRCNNWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for MaskRCNN {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyMaskRCNNWrapper(self.ptr); }
    }
}

pub struct SSD {
    ptr: *mut FD_C_SSDWrapper,
}

impl SSD {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return SSD {
                ptr: FD_C_CreateSSDWrapper(CString::new(model_file).unwrap().into_raw(),
                                           CString::new(param_file).unwrap().into_raw(),
                                           CString::new(config_file).unwrap().into_raw(),
                                           runtime_option.ptr,
                                           model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_SSDWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_SSDWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_SSDWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for SSD {
    fn drop(&mut self) {
        unsafe { FD_C_DestroySSDWrapper(self.ptr); }
    }
}

pub struct PaddleYOLOv5 {
    ptr: *mut FD_C_PaddleYOLOv5Wrapper,
}

impl PaddleYOLOv5 {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return PaddleYOLOv5 {
                ptr: FD_C_CreatePaddleYOLOv5Wrapper(CString::new(model_file).unwrap().into_raw(),
                                                    CString::new(param_file).unwrap().into_raw(),
                                                    CString::new(config_file).unwrap().into_raw(),
                                                    runtime_option.ptr,
                                                    model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_PaddleYOLOv5WrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_PaddleYOLOv5WrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PaddleYOLOv5WrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for PaddleYOLOv5 {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyPaddleYOLOv5Wrapper(self.ptr); }
    }
}

pub struct PaddleYOLOv6 {
    ptr: *mut FD_C_PaddleYOLOv6Wrapper,
}

impl PaddleYOLOv6 {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return PaddleYOLOv6 {
                ptr: FD_C_CreatePaddleYOLOv6Wrapper(CString::new(model_file).unwrap().into_raw(),
                                                    CString::new(param_file).unwrap().into_raw(),
                                                    CString::new(config_file).unwrap().into_raw(),
                                                    runtime_option.ptr,
                                                    model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_PaddleYOLOv6WrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_PaddleYOLOv6WrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PaddleYOLOv6WrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for PaddleYOLOv6 {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyPaddleYOLOv6Wrapper(self.ptr); }
    }
}

pub struct PaddleYOLOv7 {
    ptr: *mut FD_C_PaddleYOLOv7Wrapper,
}

impl PaddleYOLOv7 {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return PaddleYOLOv7 {
                ptr: FD_C_CreatePaddleYOLOv7Wrapper(CString::new(model_file).unwrap().into_raw(),
                                                    CString::new(param_file).unwrap().into_raw(),
                                                    CString::new(config_file).unwrap().into_raw(),
                                                    runtime_option.ptr,
                                                    model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_PaddleYOLOv7WrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_PaddleYOLOv7WrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PaddleYOLOv7WrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for PaddleYOLOv7 {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyPaddleYOLOv7Wrapper(self.ptr); }
    }
}

pub struct PaddleYOLOv8 {
    ptr: *mut FD_C_PaddleYOLOv8Wrapper,
}

impl PaddleYOLOv8 {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return PaddleYOLOv8 {
                ptr: FD_C_CreatePaddleYOLOv8Wrapper(CString::new(model_file).unwrap().into_raw(),
                                                    CString::new(param_file).unwrap().into_raw(),
                                                    CString::new(config_file).unwrap().into_raw(),
                                                    runtime_option.ptr,
                                                    model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_PaddleYOLOv8WrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_PaddleYOLOv8WrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PaddleYOLOv8WrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for PaddleYOLOv8 {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyPaddleYOLOv8Wrapper(self.ptr); }
    }
}

pub struct RTMDet {
    ptr: *mut FD_C_RTMDetWrapper,
}

impl RTMDet {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return RTMDet {
                ptr: FD_C_CreateRTMDetWrapper(CString::new(model_file).unwrap().into_raw(),
                                              CString::new(param_file).unwrap().into_raw(),
                                              CString::new(config_file).unwrap().into_raw(),
                                              runtime_option.ptr,
                                              model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_RTMDetWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_RTMDetWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_RTMDetWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for RTMDet {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyRTMDetWrapper(self.ptr); }
    }
}

pub struct CascadeRCNN {
    ptr: *mut FD_C_CascadeRCNNWrapper,
}

impl CascadeRCNN {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return CascadeRCNN {
                ptr: FD_C_CreateCascadeRCNNWrapper(CString::new(model_file).unwrap().into_raw(),
                                                   CString::new(param_file).unwrap().into_raw(),
                                                   CString::new(config_file).unwrap().into_raw(),
                                                   runtime_option.ptr,
                                                   model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_CascadeRCNNWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_CascadeRCNNWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_CascadeRCNNWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for CascadeRCNN {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyCascadeRCNNWrapper(self.ptr); }
    }
}

pub struct PSSDet {
    ptr: *mut FD_C_PSSDetWrapper,
}

impl PSSDet {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return PSSDet {
                ptr: FD_C_CreatePSSDetWrapper(CString::new(model_file).unwrap().into_raw(),
                                              CString::new(param_file).unwrap().into_raw(),
                                              CString::new(config_file).unwrap().into_raw(),
                                              runtime_option.ptr,
                                              model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_PSSDetWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_PSSDetWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_PSSDetWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for PSSDet {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyPSSDetWrapper(self.ptr); }
    }
}

pub struct RetinaNet {
    ptr: *mut FD_C_RetinaNetWrapper,
}

impl RetinaNet {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return RetinaNet {
                ptr: FD_C_CreateRetinaNetWrapper(CString::new(model_file).unwrap().into_raw(),
                                                 CString::new(param_file).unwrap().into_raw(),
                                                 CString::new(config_file).unwrap().into_raw(),
                                                 runtime_option.ptr,
                                                 model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_RetinaNetWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_RetinaNetWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_RetinaNetWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for RetinaNet {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyRetinaNetWrapper(self.ptr); }
    }
}

pub struct FCOS {
    ptr: *mut FD_C_FCOSWrapper,
}

impl FCOS {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return FCOS {
                ptr: FD_C_CreateFCOSWrapper(CString::new(model_file).unwrap().into_raw(),
                                            CString::new(param_file).unwrap().into_raw(),
                                            CString::new(config_file).unwrap().into_raw(),
                                            runtime_option.ptr,
                                            model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_FCOSWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_FCOSWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_FCOSWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for FCOS {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyFCOSWrapper(self.ptr); }
    }
}

pub struct TTFNet {
    ptr: *mut FD_C_TTFNetWrapper,
}

impl TTFNet {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return TTFNet {
                ptr: FD_C_CreateTTFNetWrapper(CString::new(model_file).unwrap().into_raw(),
                                              CString::new(param_file).unwrap().into_raw(),
                                              CString::new(config_file).unwrap().into_raw(),
                                              runtime_option.ptr,
                                              model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_TTFNetWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_TTFNetWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_TTFNetWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for TTFNet {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyTTFNetWrapper(self.ptr); }
    }
}

pub struct TOOD {
    ptr: *mut FD_C_TOODWrapper,
}

impl TOOD {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return TOOD {
                ptr: FD_C_CreateTOODWrapper(CString::new(model_file).unwrap().into_raw(),
                                            CString::new(param_file).unwrap().into_raw(),
                                            CString::new(config_file).unwrap().into_raw(),
                                            runtime_option.ptr,
                                            model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_TOODWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_TOODWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_TOODWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for TOOD {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyTOODWrapper(self.ptr); }
    }
}

pub struct GFL {
    ptr: *mut FD_C_GFLWrapper,
}

impl GFL {
    pub fn new(model_file: &str, param_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return GFL {
                ptr: FD_C_CreateGFLWrapper(CString::new(model_file).unwrap().into_raw(),
                                           CString::new(param_file).unwrap().into_raw(),
                                           CString::new(config_file).unwrap().into_raw(),
                                           runtime_option.ptr,
                                           model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_GFLWrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_GFLWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_GFLWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for GFL {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyGFLWrapper(self.ptr); }
    }
}

pub struct YOLOv5 {
    ptr: *mut FD_C_YOLOv5Wrapper,
}

impl YOLOv5 {
    pub fn new(model_file: &str, param_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return YOLOv5 {
                ptr: FD_C_CreateYOLOv5Wrapper(CString::new(model_file).unwrap().into_raw(),
                                              CString::new(param_file).unwrap().into_raw(),
                                              runtime_option.ptr,
                                              model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_YOLOv5WrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_YOLOv5WrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_YOLOv5WrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for YOLOv5 {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyYOLOv5Wrapper(self.ptr); }
    }
}

pub struct YOLOv6 {
    ptr: *mut FD_C_YOLOv6Wrapper,
}

impl YOLOv6 {
    pub fn new(model_file: &str, param_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return YOLOv6 {
                ptr: FD_C_CreateYOLOv6Wrapper(CString::new(model_file).unwrap().into_raw(),
                                              CString::new(param_file).unwrap().into_raw(),
                                              runtime_option.ptr,
                                              model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult, conf_threshold: f32,
                   nms_threshold: f32, ) {
        unsafe {
            FD_C_YOLOv6WrapperPredict(self.ptr, img.ptr, detect_result.ptr, conf_threshold, nms_threshold);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_YOLOv6WrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_YOLOv6WrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for YOLOv6 {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyYOLOv6Wrapper(self.ptr); }
    }
}

pub struct YOLOv7 {
    ptr: *mut FD_C_YOLOv7Wrapper,
}

impl YOLOv7 {
    pub fn new(model_file: &str, param_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return YOLOv7 {
                ptr: FD_C_CreateYOLOv7Wrapper(CString::new(model_file).unwrap().into_raw(),
                                              CString::new(param_file).unwrap().into_raw(),
                                              runtime_option.ptr,
                                              model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_YOLOv7WrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_YOLOv7WrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_YOLOv7WrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for YOLOv7 {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyYOLOv7Wrapper(self.ptr); }
    }
}

pub struct YOLOv8 {
    ptr: *mut FD_C_YOLOv8Wrapper,
}

impl YOLOv8 {
    pub fn new(model_file: &str, param_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return YOLOv8 {
                ptr: FD_C_CreateYOLOv8Wrapper(CString::new(model_file).unwrap().into_raw(),
                                              CString::new(param_file).unwrap().into_raw(),
                                              runtime_option.ptr,
                                              model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult) {
        unsafe {
            FD_C_YOLOv8WrapperPredict(self.ptr, img.ptr, detect_result.ptr);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_YOLOv8WrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_YOLOv8WrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for YOLOv8 {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyYOLOv8Wrapper(self.ptr); }
    }
}

pub struct YOLOR {
    ptr: *mut FD_C_YOLORWrapper,
}

impl YOLOR {
    pub fn new(model_file: &str, param_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return YOLOR {
                ptr: FD_C_CreateYOLORWrapper(CString::new(model_file).unwrap().into_raw(),
                                             CString::new(param_file).unwrap().into_raw(),
                                             runtime_option.ptr,
                                             model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult, conf_threshold: f32, nms_threshold: f32) {
        unsafe {
            FD_C_YOLORWrapperPredict(self.ptr, img.ptr, detect_result.ptr, conf_threshold, nms_threshold);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_YOLORWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_YOLORWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for YOLOR {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyYOLORWrapper(self.ptr); }
    }
}

pub struct YOLOX {
    ptr: *mut FD_C_YOLOXWrapper,
}

impl YOLOX {
    pub fn new(model_file: &str, param_file: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return YOLOX {
                ptr: FD_C_CreateYOLOXWrapper(CString::new(model_file).unwrap().into_raw(),
                                             CString::new(param_file).unwrap().into_raw(),
                                             runtime_option.ptr,
                                             model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, detect_result: &DetectionResult, conf_threshold: f32, nms_threshold: f32) {
        unsafe {
            FD_C_YOLOXWrapperPredict(self.ptr, img.ptr, detect_result.ptr, conf_threshold, nms_threshold);
        }
    }
    // pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, detect_result: &mut Vec<DetectionResult>) {
    //     unsafe {
    //         FD_C_YOLOXWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr, OneDimDetectionResult::new().ptr);
    //     }
    // }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_YOLOXWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for YOLOX {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyYOLOXWrapper(self.ptr); }
    }
}

pub struct Recognizer {
    ptr: *mut FD_C_RecognizerWrapper,
}

impl Recognizer {
    pub fn new(model_file: &str, param_file: &str, label_path: &str, runtime_option: &RuntimeOption, model_format: ModelFormat) -> Self {
        unsafe {
            return Recognizer {
                ptr: FD_C_CreateRecognizerWrapper(CString::new(model_file).unwrap().into_raw(),
                                                  CString::new(param_file).unwrap().into_raw(),
                                                  CString::new(label_path).unwrap().into_raw(),
                                                  runtime_option.ptr,
                                                  model_format.to_raw())
            };
        }
    }
    pub fn predict(&self, img: &Mat, text: &str, score: &mut f32) {
        unsafe {
            FD_C_RecognizerWrapperPredict(self.ptr, img.ptr, Cstr::build(text).ptr, score);
        }
    }
    pub fn batch_predict(&mut self, imgs: &mut Vec<Mat>, texts: &mut Vec<&str>, rec_scores: &mut Vec<f32>) {
        unsafe {
            FD_C_RecognizerWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr,
                                               OneDimArrayCstr::build(texts).ptr,
                                               OneDimArrayFloat::build(rec_scores.as_mut_slice()).ptr);
        }
    }
    pub fn initialized(&mut self) -> bool {
        unsafe {
            FD_C_RecognizerWrapperInitialized(self.ptr) != 0
        }
    }
}

impl Drop for Recognizer {
    fn drop(&mut self) {
        unsafe { FD_C_DestroyRecognizerWrapper(self.ptr); }
    }
}

pub struct Classifier {
    pub ptr: *mut FD_C_ClassifierWrapper,
}

impl Classifier {
    pub fn new(model_file: &str,
               params_file: &str,
               runtime_option: &RuntimeOption,
               model_format: ModelFormat, ) -> Classifier {
        Classifier {
            ptr: unsafe {
                FD_C_CreateClassifierWrapper(CString::new(model_file).unwrap().into_raw(),
                                             CString::new(params_file).unwrap().into_raw(),
                                             runtime_option.ptr,
                                             model_format.to_raw())
            }
        }
    }
    pub fn initialized(&self) -> bool {
        unsafe {
            FD_C_ClassifierWrapperInitialized(self.ptr) != 0
        }
    }
    pub fn predict(&self, img: Mat, cls_label: &mut i32, cls_score: &mut f32) -> bool {
        unsafe {
            FD_C_ClassifierWrapperPredict(self.ptr, img.ptr, cls_label, cls_score) != 0
        }
    }

    pub fn batch_predict_with_index(&self, imgs: &mut Vec<Mat>,
                                    cls_labels: &mut Vec<i32>,
                                    cls_scores: &mut Vec<f32>,
                                    start_index: usize,
                                    end_index: usize,
    ) -> bool {
        unsafe {
            FD_C_ClassifierWrapperBatchPredictWithIndex(self.ptr,
                                                        OneDimMat::build(imgs).ptr,
                                                        OneDimArrayInt32::build(cls_labels.as_mut_slice()).ptr,
                                                        OneDimArrayFloat::build(cls_scores.as_mut_slice()).ptr,
                                                        start_index, end_index) != 0
        }
    }
}

impl Drop for Classifier {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyClassifierWrapper(self.ptr);
        }
    }
}


pub struct DBDetector {
    pub ptr: *mut FD_C_DBDetectorWrapper,
}

impl DBDetector {
    pub fn new(model_file: &str, params_file: &str, runtime_option: &RuntimeOption, model_format: &ModelFormat) -> DBDetector {
        DBDetector {
            ptr: unsafe {
                FD_C_CreateDBDetectorWrapper(
                    CString::new(model_file).unwrap().into_raw(),
                    CString::new(params_file).unwrap().into_raw(),
                    runtime_option.ptr,
                    model_format.to_raw(),
                )
            }
        }
    }

    pub fn initialized(&self) -> bool {
        unsafe {
            FD_C_DBDetectorWrapperInitialized(self.ptr) != 0
        }
    }
    pub fn predict(&self, img: Mat, boxes_result: &mut Vec<Vec<i32>>) -> bool {
        unsafe {
            FD_C_DBDetectorWrapperPredict(self.ptr, img.ptr, TwoDimArrayInt32::build(boxes_result).ptr) != 0
        }
    }

    pub fn batch_predict(&self, img: &mut Vec<Mat>, boxes_result: &mut Vec<Vec<Vec<i32>>>) -> bool {
        unsafe {
            FD_C_DBDetectorWrapperBatchPredict(self.ptr, OneDimMat::build(img).ptr,
                                               ThreeDimArrayInt32::build(boxes_result).ptr) != 0
        }
    }
}

impl Drop for DBDetector {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyDBDetectorWrapper(self.ptr);
        }
    }
}

pub struct StructureV2Table {
    pub ptr: *mut FD_C_StructureV2TableWrapper,
}

impl StructureV2Table {
    pub fn new(model_file: &str, params_file: &str, table_char_dict_path: &str, runtime_option: &RuntimeOption, model_format: &ModelFormat) -> StructureV2Table {
        StructureV2Table {
            ptr: unsafe {
                FD_C_CreateStructureV2TableWrapper(
                    CString::new(model_file).unwrap().into_raw(),
                    CString::new(params_file).unwrap().into_raw(),
                    CString::new(table_char_dict_path).unwrap().into_raw(),
                    runtime_option.ptr,
                    model_format.to_raw(),
                )
            }
        }
    }

    pub fn initialized(&self) -> bool {
        unsafe {
            FD_C_StructureV2TableWrapperInitialized(self.ptr) != 0
        }
    }
    pub fn predict(&self, img: Mat, boxes_result: &mut Vec<Vec<i32>>, structure_result: &mut Vec<&str>) -> bool {
        unsafe {
            FD_C_StructureV2TableWrapperPredict(self.ptr, img.ptr,
                                                TwoDimArrayInt32::build(boxes_result).ptr,
                                                OneDimArrayCstr::build(structure_result).ptr) != 0
        }
    }

    pub fn batch_predict(&self, img: &mut Vec<Mat>, boxes_results: &mut Vec<Vec<Vec<i32>>>, structure_results: &mut Vec<Vec<&str>>) -> bool {
        unsafe {
            FD_C_StructureV2TableWrapperBatchPredict(self.ptr, OneDimMat::build(img).ptr,
                                                     ThreeDimArrayInt32::build(boxes_results).ptr,
                                                     TwoDimArrayCstr::build(structure_results).ptr) != 0
        }
    }
}

impl Drop for StructureV2Table {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyStructureV2TableWrapper(self.ptr);
        }
    }
}

pub struct PPOCRv2 {
    pub ptr: *mut FD_C_PPOCRv2Wrapper,
}

impl PPOCRv2 {
    pub fn new(det_model: &DBDetector, cls_model: &Classifier, rec_model: &Recognizer) -> PPOCRv2 {
        PPOCRv2 {
            ptr: unsafe {
                FD_C_CreatePPOCRv2Wrapper(det_model.ptr, cls_model.ptr, rec_model.ptr)
            }
        }
    }

    pub fn initialized(&self) -> bool {
        unsafe {
            FD_C_PPOCRv2WrapperInitialized(self.ptr) != 0
        }
    }
    pub fn predict(&self, img: Mat, result: &OcrResult) -> bool {
        unsafe {
            FD_C_PPOCRv2WrapperPredict(self.ptr, img.ptr, result.ptr) != 0
        }
    }

    pub fn batch_predict(&self, img: &mut Vec<Mat>, results: &mut Vec<OcrResult>) -> bool {
        unsafe {
            FD_C_PPOCRv2WrapperBatchPredict(self.ptr, OneDimMat::build(img).ptr,
                                            OneDimOcrResult::new(results).ptr) != 0
        }
    }
}

impl Drop for PPOCRv2 {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyPPOCRv2Wrapper(self.ptr);
        }
    }
}

pub struct PPOCRv3 {
    pub ptr: *mut FD_C_PPOCRv3Wrapper,
}

impl PPOCRv3 {
    pub fn new(det_model: &DBDetector, cls_model: &Classifier, rec_model: &Recognizer) -> PPOCRv3 {
        PPOCRv3 {
            ptr: unsafe {
                FD_C_CreatePPOCRv3Wrapper(det_model.ptr, cls_model.ptr, rec_model.ptr)
            }
        }
    }

    pub fn initialized(&self) -> bool {
        unsafe {
            FD_C_PPOCRv3WrapperInitialized(self.ptr) != 0
        }
    }
    pub fn predict(&self, img: Mat, result: &OcrResult) -> bool {
        unsafe {
            FD_C_PPOCRv3WrapperPredict(self.ptr, img.ptr, result.ptr) != 0
        }
    }

    pub fn batch_predict(&self, img: &mut Vec<Mat>, results: &mut Vec<OcrResult>) -> bool {
        unsafe {
            FD_C_PPOCRv3WrapperBatchPredict(self.ptr, OneDimMat::build(img).ptr,
                                            OneDimOcrResult::new(results).ptr) != 0
        }
    }
}

impl Drop for PPOCRv3 {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyPPOCRv3Wrapper(self.ptr);
        }
    }
}

pub struct PPStructureV2Table {
    pub ptr: *mut FD_C_PPStructureV2TableWrapper,
}

impl PPStructureV2Table {
    pub fn new(det_model: &DBDetector,
               rec_model: &Recognizer,
               table_model: StructureV2Table) -> PPStructureV2Table {
        PPStructureV2Table {
            ptr: unsafe {
                FD_C_CreatePPStructureV2TableWrapper(
                    det_model.ptr,
                    rec_model.ptr,
                    table_model.ptr,
                )
            }
        }
    }

    pub fn initialized(&self) -> bool {
        unsafe {
            FD_C_PPStructureV2TableWrapperInitialized(self.ptr) != 0
        }
    }
    pub fn predict(&self, img: Mat, result: &mut OcrResult) -> bool {
        unsafe {
            FD_C_PPStructureV2TableWrapperPredict(self.ptr, img.ptr, result.ptr) != 0
        }
    }

    pub fn batch_predict(&self, img: &mut Vec<Mat>, results: &mut Vec<OcrResult>) -> bool {
        unsafe {
            FD_C_PPStructureV2TableWrapperBatchPredict(self.ptr, OneDimMat::build(img).ptr,
                                                       OneDimOcrResult::new(results).ptr) != 0
        }
    }
}

impl Drop for PPStructureV2Table {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyPPStructureV2TableWrapper(self.ptr);
        }
    }
}

pub struct PaddleSegModel {
    ptr: *mut FD_C_PaddleSegModelWrapper,
}

impl PaddleSegModel {
    pub fn new(model_file: &str, params_file: &str, config_file: &str, runtime_option: &RuntimeOption, model_format: &ModelFormat) -> PaddleSegModel {
        PaddleSegModel {
            ptr: unsafe {
                FD_C_CreatePaddleSegModelWrapper(
                    CString::new(model_file).unwrap().into_raw(),
                    CString::new(params_file).unwrap().into_raw(),
                    CString::new(config_file).unwrap().into_raw(),
                    runtime_option.ptr,
                    model_format.to_raw(),
                )
            }
        }
    }

    pub fn initialized(&self) -> bool {
        unsafe {
            FD_C_PaddleSegModelWrapperInitialized(self.ptr) != 0
        }
    }
    pub fn predict(&self, img: Mat, result: &mut SegmentationResult) -> bool {
        unsafe {
            FD_C_PaddleSegModelWrapperPredict(self.ptr, img.ptr, result.ptr) != 0
        }
    }

    pub fn batch_predict(&self, imgs: &mut Vec<Mat>, results: &mut Vec<SegmentationResult>) -> bool {
        unsafe {
            FD_C_PaddleSegModelWrapperBatchPredict(self.ptr, OneDimMat::build(imgs).ptr,
                                                   OneDimSegmentationResult::new(results).ptr) != 0
        }
    }
}

impl Drop for PaddleSegModel {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyPaddleSegModelWrapper(self.ptr);
        }
    }
}







