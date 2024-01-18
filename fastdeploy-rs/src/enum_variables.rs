#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use fastdeploy_bind::*;

pub enum ModelFormat {
    AUTOREC,
    PADDLE,
    ONNX,
    RKNN,
    TORCHSCRIPT,
    SOPHGO,
}

impl ModelFormat {
    pub fn to_raw(&self) -> i32 {
        match self {
            ModelFormat::AUTOREC => FD_C_ModelFormat_AUTOREC,
            ModelFormat::PADDLE => FD_C_ModelFormat_PADDLE,
            ModelFormat::ONNX => FD_C_ModelFormat_ONNX,
            ModelFormat::RKNN => FD_C_ModelFormat_RKNN,
            ModelFormat::TORCHSCRIPT => FD_C_ModelFormat_TORCHSCRIPT,
            ModelFormat::SOPHGO => FD_C_ModelFormat_SOPHGO,
        }
    }
}

pub enum RKNpu2CpuName {
    RK356X,
    RK3588,
    UNDEFINED,
}

impl RKNpu2CpuName {
    pub fn to_raw(&self) -> i32 {
        match self {
            RKNpu2CpuName::RK356X => FD_C_ModelFormat_RK356X,
            RKNpu2CpuName::RK3588 => FD_C_ModelFormat_RK3588,
            RKNpu2CpuName::UNDEFINED => FD_C_ModelFormat_UNDEFINED,
        }
    }
}

pub enum RKNpu2CoreMask {
    RKNN_NPU_CORE_AUTO,
    RKNN_NPU_CORE_0,
    RKNN_NPU_CORE_1,
    RKNN_NPU_CORE_2,
    RKNN_NPU_CORE_0_1,
    RKNN_NPU_CORE_0_1_2,
    RKNN_NPU_CORE_UNDEFINED,
}

impl RKNpu2CoreMask {
    pub fn to_raw(&self) -> i32 {
        match self {
            RKNpu2CoreMask::RKNN_NPU_CORE_AUTO => FD_C_ModelFormat_RKNN_NPU_CORE_AUTO,
            RKNpu2CoreMask::RKNN_NPU_CORE_0 => FD_C_ModelFormat_RKNN_NPU_CORE_0,
            RKNpu2CoreMask::RKNN_NPU_CORE_1 => FD_C_ModelFormat_RKNN_NPU_CORE_1,
            RKNpu2CoreMask::RKNN_NPU_CORE_2 => FD_C_ModelFormat_RKNN_NPU_CORE_2,
            RKNpu2CoreMask::RKNN_NPU_CORE_0_1 => FD_C_ModelFormat_RKNN_NPU_CORE_0_1,
            RKNpu2CoreMask::RKNN_NPU_CORE_0_1_2 => FD_C_ModelFormat_RKNN_NPU_CORE_0_1_2,
            RKNpu2CoreMask::RKNN_NPU_CORE_UNDEFINED => FD_C_ModelFormat_RKNN_NPU_CORE_UNDEFINED,
        }
    }
}

pub enum LitePowerMode {
    LITE_POWER_HIGH,
    LITE_POWER_LOW,
    LITE_POWER_FULL,
    LITE_POWER_NO_BIND,
    LITE_POWER_RAND_HIGH,
    LITE_POWER_RAND_LOW,
}

impl LitePowerMode {
    pub fn to_raw(&self) -> i32 {
        match self {
            LitePowerMode::LITE_POWER_HIGH => FD_C_ModelFormat_LITE_POWER_HIGH,
            LitePowerMode::LITE_POWER_LOW => FD_C_ModelFormat_LITE_POWER_LOW,
            LitePowerMode::LITE_POWER_FULL => FD_C_ModelFormat_LITE_POWER_FULL,
            LitePowerMode::LITE_POWER_NO_BIND => FD_C_ModelFormat_LITE_POWER_NO_BIND,
            LitePowerMode::LITE_POWER_RAND_HIGH => FD_C_ModelFormat_LITE_POWER_RAND_HIGH,
            LitePowerMode::LITE_POWER_RAND_LOW => FD_C_ModelFormat_LITE_POWER_RAND_LOW,
        }
    }
}

#[derive(Debug)]
pub enum ResultType {
    UNKNOWN_RESULT,
    CLASSIFY,
    DETECTION,
    SEGMENTATION,
    OCR,
    MOT,
    FACE_DETECTION,
    FACE_ALIGNMENT,
    FACE_RECOGNITION,
    MATTING,
    MASK,
    KEYPOINT_DETECTION,
    HEADPOSE,
}

impl ResultType {
    pub fn to_c_type(self) -> i32 {
        match self {
            ResultType::UNKNOWN_RESULT => FD_C_ModelFormat_UNKNOWN_RESULT,
            ResultType::CLASSIFY => FD_C_ModelFormat_CLASSIFY,
            ResultType::DETECTION => FD_C_ModelFormat_DETECTION,
            ResultType::SEGMENTATION => FD_C_ModelFormat_SEGMENTATION,
            ResultType::OCR => FD_C_ModelFormat_OCR,
            ResultType::MOT => FD_C_ModelFormat_MOT,
            ResultType::FACE_DETECTION => FD_C_ModelFormat_FACE_DETECTION,
            ResultType::FACE_ALIGNMENT => FD_C_ModelFormat_FACE_ALIGNMENT,
            ResultType::FACE_RECOGNITION => FD_C_ModelFormat_FACE_RECOGNITION,
            ResultType::MATTING => FD_C_ModelFormat_MATTING,
            ResultType::MASK => FD_C_ModelFormat_MASK,
            ResultType::KEYPOINT_DETECTION => FD_C_ModelFormat_KEYPOINT_DETECTION,
            ResultType::HEADPOSE => FD_C_ModelFormat_HEADPOSE,
        }
    }
    pub fn from_c_type(type_: i32) -> ResultType {
        match type_ {
            FD_C_ModelFormat_UNKNOWN_RESULT => ResultType::UNKNOWN_RESULT,
            FD_C_ModelFormat_CLASSIFY => ResultType::CLASSIFY,
            FD_C_ModelFormat_DETECTION => ResultType::DETECTION,
            FD_C_ModelFormat_SEGMENTATION => ResultType::SEGMENTATION,
            FD_C_ModelFormat_OCR => ResultType::OCR,
            FD_C_ModelFormat_MOT => ResultType::MOT,
            FD_C_ModelFormat_FACE_DETECTION => ResultType::FACE_DETECTION,
            FD_C_ModelFormat_FACE_ALIGNMENT => ResultType::FACE_ALIGNMENT,
            FD_C_ModelFormat_FACE_RECOGNITION => ResultType::FACE_RECOGNITION,
            FD_C_ModelFormat_MATTING => ResultType::MATTING,
            FD_C_ModelFormat_MASK => ResultType::MASK,
            FD_C_ModelFormat_KEYPOINT_DETECTION => ResultType::KEYPOINT_DETECTION,
            FD_C_ModelFormat_HEADPOSE => ResultType::HEADPOSE,
            _ => ResultType::UNKNOWN_RESULT,
        }
    }
}

