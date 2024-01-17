#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString};

use fastdeploy_bind::*;

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
    pub ptr: *mut FD_C_DetectionResult,
}

impl DetectionResult {
    pub fn new() -> DetectionResult {
        unsafe {
            DetectionResult {
                ptr: FD_C_CreateDetectionResult(),
            }
        }
    }
    pub fn from_c_ptr(data: *mut FD_C_DetectionResult) -> DetectionResult {
        unsafe {
            DetectionResult { ptr: data }
        }
    }
    pub fn str(&self) -> &str {
        unsafe {
            let s: &str = &String::from_utf8(vec![0x01; 10240]).unwrap();
            let c = CString::new(s).unwrap().into_raw();
            FD_C_DetectionResultStr(self.ptr, c);
            return CStr::from_ptr(c).to_str().unwrap();
        }
    }
}

pub struct OneDimDetectionResult {
    pub ptr: *mut FD_C_OneDimDetectionResult,
}

impl OneDimDetectionResult {
    pub fn new() -> OneDimDetectionResult {
        unsafe {
            OneDimDetectionResult {
                ptr: &mut FD_C_OneDimDetectionResult { size: 0, data: FD_C_CreateDetectionResult() },
            }
        }
    }
}

impl Drop for DetectionResult {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyDetectionResult(self.ptr);
        }
    }
}

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


