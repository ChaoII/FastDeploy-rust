use std::ffi::CString;

use fastdeploy_bind::*;

use crate::enum_variables::ResultType;

pub mod common {
    use std::slice;

    use fastdeploy_bind::{FD_C_Bool, FD_C_OneDimArrayFloat, FD_C_OneDimArrayInt32, FD_C_OneDimArrayInt64, FD_C_OneDimArrayUint8, FD_C_TwoDimArrayFloat};

    pub fn fd_c_bool_to_bool(ret: FD_C_Bool) -> bool {
        ret >= 1
    }

    pub fn fd_c_one_dim_array_float_to_vec_float(array: FD_C_OneDimArrayFloat) -> Vec<f32> {
        if array.data.is_null() {
            return vec![];
        }
        unsafe {
            slice::from_raw_parts(array.data as *const f32, array.size).to_vec()
        }
    }

    pub fn fd_c_one_dim_array_int32_to_vec_i32(array: FD_C_OneDimArrayInt32) -> Vec<i32> {
        if array.data.is_null() {
            return vec![];
        }
        unsafe {
            slice::from_raw_parts(array.data as *const i32, array.size).to_vec()
        }
    }

    pub fn fd_c_one_dim_array_int64_to_vec_i64(array: FD_C_OneDimArrayInt64) -> Vec<i64> {
        if array.data.is_null() {
            return vec![];
        }
        unsafe {
            slice::from_raw_parts(array.data as *const i64, array.size).to_vec()
        }
    }

    pub fn fd_c_one_dim_array_uint8_to_vec_u8(array: FD_C_OneDimArrayUint8) -> Vec<u8> {
        if array.data.is_null() {
            return vec![];
        }
        unsafe {
            slice::from_raw_parts(array.data as *const u8, array.size).to_vec()
        }
    }


    pub fn fd_c_two_dim_array_float_to_vec_float(array: *mut FD_C_TwoDimArrayFloat) -> Vec<Vec<f32>> {
        unsafe {
            if array.is_null() || (*array).data.is_null() {
                return vec![];
            }
            let mut result = vec![];
            for i in 0..(*array).size {
                let ptr = (*array).data.wrapping_add(i);
                result.push(fd_c_one_dim_array_float_to_vec_float(*ptr))
            }
            return result;
        }
    }
}


pub struct OneDimArrayUint8 {
    ptr: *mut FD_C_OneDimArrayUint8,
}

impl OneDimArrayUint8 {
    pub fn build(array: &mut [u8]) -> OneDimArrayUint8 {
        OneDimArrayUint8 {
            ptr: &mut FD_C_OneDimArrayUint8 { size: array.len(), data: array.as_mut_ptr() }
        }
    }
}

impl Drop for OneDimArrayUint8 {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayUint8(self.ptr)
        }
    }
}

pub struct OneDimArrayInt8 {
    ptr: *mut FD_C_OneDimArrayInt8,
}

impl OneDimArrayInt8 {
    pub fn build(array: &mut Vec<i8>) -> OneDimArrayInt8 {
        OneDimArrayInt8 {
            ptr: &mut FD_C_OneDimArrayInt8 { size: array.len(), data: array.as_mut_ptr() }
        }
    }
}

impl Drop for OneDimArrayInt8 {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayInt8(self.ptr)
        }
    }
}

pub struct OneDimArrayInt32 {
    pub ptr: *mut FD_C_OneDimArrayInt32,
}

impl OneDimArrayInt32 {
    pub fn build(array: &mut [i32]) -> OneDimArrayInt32 {
        OneDimArrayInt32 {
            ptr: &mut FD_C_OneDimArrayInt32 { size: array.len(), data: array.as_mut_ptr() }
        }
    }
}

impl Drop for OneDimArrayInt32 {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayInt32(self.ptr)
        }
    }
}


pub struct OneDimArrayInt64 {
    pub ptr: *mut FD_C_OneDimArrayInt64,
}

impl OneDimArrayInt64 {
    pub fn build(array: &mut [i64]) -> OneDimArrayInt64 {
        OneDimArrayInt64 {
            ptr: &mut FD_C_OneDimArrayInt64 { size: array.len(), data: array.as_mut_ptr() }
        }
    }
}

impl Drop for OneDimArrayInt64 {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayInt64(self.ptr)
        }
    }
}

pub struct OneDimArraySize {
    pub ptr: *mut FD_C_OneDimArraySize,
}

impl OneDimArraySize {
    pub fn build(array: &mut [usize]) -> OneDimArraySize {
        OneDimArraySize {
            ptr: &mut FD_C_OneDimArraySize { size: array.len(), data: array.as_mut_ptr() }
        }
    }
}

impl Drop for OneDimArraySize {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArraySize(self.ptr)
        }
    }
}

pub struct OneDimArrayFloat {
    pub ptr: *mut FD_C_OneDimArrayFloat,
}

impl OneDimArrayFloat {
    pub fn build(array: &mut [f32]) -> OneDimArrayFloat {
        OneDimArrayFloat {
            ptr: &mut FD_C_OneDimArrayFloat { size: array.len(), data: array.as_mut_ptr() }
        }
    }
}

impl Drop for OneDimArrayFloat {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayFloat(self.ptr)
        }
    }
}

pub struct TwoDimArrayInt8 {
    ptr: *mut FD_C_TwoDimArrayInt8,
}

impl TwoDimArrayInt8 {
    pub fn build(array: &mut Vec<Vec<i8>>) -> TwoDimArrayInt8 {
        unsafe {
            let one_dim_int8 = OneDimArrayInt8::build(&mut (*array.as_mut_ptr()));
            TwoDimArrayInt8 {
                ptr: &mut FD_C_TwoDimArrayInt8 { size: array.len(), data: one_dim_int8.ptr }
            }
        }
    }
}

impl Drop for TwoDimArrayInt8 {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyTwoDimArrayInt8(self.ptr)
        }
    }
}

pub struct TwoDimArrayInt32 {
    pub ptr: *mut FD_C_TwoDimArrayInt32,
}

impl TwoDimArrayInt32 {
    pub fn build(array: &mut Vec<Vec<i32>>) -> TwoDimArrayInt32 {
        unsafe {
            let one_dim_int8 = OneDimArrayInt32::build(&mut (*array.as_mut_ptr()));
            TwoDimArrayInt32 {
                ptr: &mut FD_C_TwoDimArrayInt32 { size: array.len(), data: one_dim_int8.ptr }
            }
        }
    }
}


impl Drop for TwoDimArrayInt32 {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyTwoDimArrayInt32(self.ptr)
        }
    }
}

pub struct TwoDimArrayFloat {
    pub ptr: *mut FD_C_TwoDimArrayFloat,
}

impl TwoDimArrayFloat {
    pub fn build(array: &mut Vec<Vec<f32>>) -> TwoDimArrayFloat {
        unsafe {
            let one_dim_int8 = OneDimArrayFloat::build(&mut (*array.as_mut_ptr()));
            TwoDimArrayFloat {
                ptr: &mut FD_C_TwoDimArrayFloat { size: array.len(), data: one_dim_int8.ptr }
            }
        }
    }
}

impl Drop for TwoDimArrayFloat {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyTwoDimArrayFloat(self.ptr)
        }
    }
}

pub struct TwoDimArraySize {
    ptr: *mut FD_C_TwoDimArraySize,
}

impl TwoDimArraySize {
    pub fn build(array: &mut Vec<Vec<usize>>) -> TwoDimArraySize {
        unsafe {
            let one_dim_int8 = OneDimArraySize::build(&mut (*array.as_mut_ptr()));
            TwoDimArraySize {
                ptr: &mut FD_C_TwoDimArraySize { size: array.len(), data: one_dim_int8.ptr }
            }
        }
    }
}

impl Drop for TwoDimArraySize {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyTwoDimArraySize(self.ptr)
        }
    }
}

pub struct ThreeDimArrayInt32 {
    pub ptr: *mut FD_C_ThreeDimArrayInt32,
}

impl ThreeDimArrayInt32 {
    pub fn build(array: &mut Vec<Vec<Vec<i32>>>) -> ThreeDimArrayInt32 {
        unsafe {
            let two_dim_int8 = TwoDimArrayInt32::build(&mut (*array.as_mut_ptr()));
            ThreeDimArrayInt32 {
                ptr: &mut FD_C_ThreeDimArrayInt32 { size: array.len(), data: two_dim_int8.ptr }
            }
        }
    }
}

impl Drop for ThreeDimArrayInt32 {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyThreeDimArrayInt32(self.ptr)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mat {
    pub ptr: FD_C_Mat,
}

impl Mat {
    pub fn imread(file_path: &str) -> Mat {
        unsafe {
            Mat {
                ptr: FD_C_Imread(CString::new(file_path).unwrap().into_raw()),
            }
        }
    }

    pub fn imwrite(&self, file_path: &str) -> bool {
        unsafe {
            return FD_C_Imwrite(CString::new(file_path).unwrap().into_raw(), self.ptr) != 0;
        }
    }
}

// impl Drop for Mat {
//     fn drop(&mut self) {
//         unsafe {
//             FD_C_DestroyMat(self.ptr)
//         }
//     }
// }

pub struct OneDimMat {
    pub ptr: FD_C_OneDimMat,
}

impl OneDimMat {
    pub fn build(data: &mut Vec<Mat>) -> OneDimMat {
        unsafe {
            OneDimMat {
                ptr: FD_C_OneDimMat { size: data.len(), data: &mut (*data.as_mut_ptr()).ptr }
            }
        }
    }
}

impl Drop for OneDimMat {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimMat(&mut self.ptr)
        }
    }
}

pub struct Mask {
    pub ptr: *mut FD_C_Mask,
}

impl Mask {
    pub fn build(data: &mut Vec<u8>, shape: &mut Vec<i64>, type_: ResultType) -> Mask {
        unsafe {
            let c_mask = FD_C_Mask {
                data: *OneDimArrayUint8::build(data).ptr,
                shape: *OneDimArrayInt64::build(shape).ptr,
                type_: type_.to_c_type(),
            };
            Mask {
                ptr: &mut c_mask.to_owned() as *mut FD_C_Mask,
            }
        }
    }
}


pub struct OneDimMask {
    pub ptr: *mut FD_C_OneDimMask,
}

impl OneDimMask {
    pub fn build(data: &mut Vec<Mask>) -> OneDimMask {
        unsafe {
            let c: *mut FD_C_Mask = (*data.as_mut_ptr()).ptr;
            let c_one_dim_mask = FD_C_OneDimMask {
                size: data.len(),
                data: c,
            };

            OneDimMask {
                ptr: &mut c_one_dim_mask.to_owned()
            }
        }
    }
}

pub struct Cstr {
    pub ptr: *mut FD_C_Cstr,
}

impl Cstr {
    pub fn build(data: &str) -> Cstr {
        unsafe {
            let c_cstr = FD_C_Cstr { size: data.len(), data: CString::new(data).unwrap().into_raw() };
            Cstr {
                ptr: &mut c_cstr.to_owned() as *mut FD_C_Cstr
            }
        }
    }
}

impl Drop for Cstr {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyCstr(self.ptr)
        }
    }
}

pub struct OneDimArrayCstr {
    pub ptr: *mut FD_C_OneDimArrayCstr,
}

impl OneDimArrayCstr {
    pub fn build(data: &mut Vec<&str>) -> OneDimArrayCstr {
        unsafe {
            let c: *mut FD_C_Cstr = Cstr::build(*data.as_mut_ptr()).ptr;
            let c_one_dim_mask = FD_C_OneDimArrayCstr {
                size: data.len(),
                data: c,
            };
            OneDimArrayCstr {
                ptr: &mut c_one_dim_mask.to_owned() as *mut FD_C_OneDimArrayCstr
            }
        }
    }
}

impl Drop for OneDimArrayCstr {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayCstr(self.ptr)
        }
    }
}

pub struct TwoDimArrayCstr {
    pub ptr: *mut FD_C_TwoDimArrayCstr,
}

impl TwoDimArrayCstr {
    pub fn build(data: &mut Vec<Vec<&str>>) -> TwoDimArrayCstr {
        unsafe {
            let one_dim_c_str = OneDimArrayCstr::build(&mut (*data.as_mut_ptr()));
            let c_two_dim_mask = FD_C_TwoDimArrayCstr {
                size: data.len(),
                data: one_dim_c_str.ptr,
            };
            TwoDimArrayCstr {
                ptr: &mut c_two_dim_mask.to_owned() as *mut FD_C_TwoDimArrayCstr
            }
        }
    }
}

impl Drop for TwoDimArrayCstr {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyTwoDimArrayCstr(self.ptr)
        }
    }
}



