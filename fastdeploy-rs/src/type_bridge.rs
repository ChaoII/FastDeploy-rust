use std::ffi::{c_char, CStr, CString};
use std::slice;
use std::str::Utf8Error;

use fastdeploy_bind::*;

pub mod common {
    use std::slice;

    use fastdeploy_bind::{FD_C_Bool,
                          FD_C_OneDimArrayFloat,
                          FD_C_OneDimArrayInt32,
                          FD_C_OneDimArrayInt64,
                          FD_C_OneDimArrayUint8,
                          FD_C_TwoDimArrayFloat};

    #[inline]
    pub fn fd_c_bool_to_bool(ret: FD_C_Bool) -> bool {
        ret >= 1
    }


    pub fn fd_c_one_dim_array_float_to_vec_float(array: FD_C_OneDimArrayFloat) -> Vec<f32> {
        if array.data.is_null() {
            return vec![];
        }
        unsafe {
            let c = slice::from_raw_parts(array.data as *const f32, array.size).to_vec();
            return c;
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

    pub fn fd_c_two_dim_array_float_to_vec_float(array: FD_C_TwoDimArrayFloat) -> Vec<Vec<f32>> {
        unsafe {
            let mut result = vec![];
            for i in 0..array.size {
                let ptr = array.data.wrapping_add(i);
                result.push(fd_c_one_dim_array_float_to_vec_float(*ptr))
            }
            return result;
        }
    }
}


pub struct OneDimArrayUint8Wrapper {
    ptr: *mut FD_C_OneDimArrayUint8,
}

impl OneDimArrayUint8Wrapper {
    pub fn build(array: &mut [u8]) -> OneDimArrayUint8Wrapper {
        OneDimArrayUint8Wrapper {
            ptr: &mut FD_C_OneDimArrayUint8 { size: array.len(), data: array.as_mut_ptr() }
        }
    }

    pub unsafe fn to_vec(&self) -> Vec<u8> {
        return Vec::from_raw_parts((*self.ptr).data, (*self.ptr).size, (*self.ptr).size);
    }
}

impl Drop for OneDimArrayUint8Wrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayUint8(self.ptr)
        }
    }
}

pub struct OneDimArrayInt8Wrapper {
    ptr: *mut FD_C_OneDimArrayInt8,
}

impl OneDimArrayInt8Wrapper {
    pub fn build(array: &mut Vec<i8>) -> OneDimArrayInt8Wrapper {
        OneDimArrayInt8Wrapper {
            ptr: &mut FD_C_OneDimArrayInt8 { size: array.len(), data: array.as_mut_ptr() }
        }
    }
    pub unsafe fn to_vec(&self) -> Vec<i8> {
        return Vec::from_raw_parts((*self.ptr).data, (*self.ptr).size, (*self.ptr).size);
    }
}

impl Drop for OneDimArrayInt8Wrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayInt8(self.ptr)
        }
    }
}

pub struct OneDimArrayInt32Wrapper {
    pub ptr: Box<FD_C_OneDimArrayInt32>,
}

impl OneDimArrayInt32Wrapper {
    pub fn to_vec(&self) -> Vec<i32> {
        unsafe {
            return slice::from_raw_parts(self.ptr.data, self.ptr.size).to_vec();
        }
    }
}

impl Default for OneDimArrayInt32Wrapper {
    fn default() -> Self {
        Self { ptr: Box::new(FD_C_OneDimArrayInt32 { size: 0, data: std::ptr::null_mut() }) }
    }
}

impl Drop for OneDimArrayInt32Wrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayInt32(self.ptr.as_mut())
        }
    }
}


pub struct OneDimArrayInt64Wrapper {
    pub ptr: *mut FD_C_OneDimArrayInt64,
}

impl OneDimArrayInt64Wrapper {
    pub fn build(array: &mut [i64]) -> OneDimArrayInt64Wrapper {
        OneDimArrayInt64Wrapper {
            ptr: &mut FD_C_OneDimArrayInt64 { size: array.len(), data: array.as_mut_ptr() }
        }
    }
    pub unsafe fn to_vec(&self) -> Vec<i64> {
        return Vec::from_raw_parts((*self.ptr).data, (*self.ptr).size, (*self.ptr).size);
    }
}

impl Drop for OneDimArrayInt64Wrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayInt64(self.ptr)
        }
    }
}

pub struct OneDimArraySizeWrapper {
    pub ptr: *mut FD_C_OneDimArraySize,
}

impl OneDimArraySizeWrapper {
    pub fn build(array: &mut [usize]) -> OneDimArraySizeWrapper {
        OneDimArraySizeWrapper {
            ptr: &mut FD_C_OneDimArraySize { size: array.len(), data: array.as_mut_ptr() }
        }
    }
    pub unsafe fn to_vec(&self) -> Vec<usize> {
        return Vec::from_raw_parts((*self.ptr).data, (*self.ptr).size, (*self.ptr).size);
    }
}

impl Drop for OneDimArraySizeWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArraySize(self.ptr)
        }
    }
}

pub struct OneDimArrayFloatWrapper {
    pub ptr: Box<FD_C_OneDimArrayFloat>,
}

impl OneDimArrayFloatWrapper {
    pub fn to_vec(mut self) -> Vec<f32> {
        unsafe {
            // 此处进行数据拷贝
            return slice::from_raw_parts(self.ptr.data, self.ptr.size).to_vec();
        }
    }
}

impl Default for OneDimArrayFloatWrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_OneDimArrayFloat { size: 0, data: std::ptr::null_mut() }),
        }
    }
}

impl Drop for OneDimArrayFloatWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayFloat(self.ptr.as_mut());
        }
    }
}

pub struct TwoDimArrayInt8Wrapper {
    ptr: *mut FD_C_TwoDimArrayInt8,
}

impl TwoDimArrayInt8Wrapper {
    pub unsafe fn build(vec: &mut Vec<Vec<i8>>) -> TwoDimArrayInt8Wrapper {
        let mut data_raw_ptrs = vec![std::ptr::null_mut(); vec.len()];
        for i in 0..vec.len() {
            data_raw_ptrs[i] = OneDimArrayInt8Wrapper::build(&mut vec[i]).ptr
        }
        TwoDimArrayInt8Wrapper {
            ptr: &mut FD_C_TwoDimArrayInt8 { size: vec.len(), data: *data_raw_ptrs.as_mut_ptr() },
        }
    }
    pub unsafe fn to_vec(&self) -> Vec<Vec<i8>> {
        let mut vec = Vec::with_capacity((*self.ptr).size);
        for i in 0..(*self.ptr).size {
            let temp = (*(*self.ptr).data.wrapping_add(i));
            vec.push(Vec::from_raw_parts(temp.data, temp.size, temp.size));
        }
        return vec;
    }
}

impl Drop for TwoDimArrayInt8Wrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyTwoDimArrayInt8(self.ptr)
        }
    }
}

pub struct TwoDimArrayInt32Wrapper {
    pub ptr: Box<FD_C_TwoDimArrayInt32>,
}

impl TwoDimArrayInt32Wrapper {
    pub unsafe fn build(vec: &mut Vec<Vec<i32>>) -> TwoDimArrayInt32Wrapper {
        let mut data_raw_ptrs = vec![std::ptr::null_mut(); vec.len()];
        for i in 0..vec.len() {
            data_raw_ptrs[i] = OneDimArrayInt32Wrapper::build(&mut vec[i]).ptr
        }
        TwoDimArrayInt32Wrapper {
            ptr: Box::new(FD_C_TwoDimArrayInt32 { size: vec.len(), data: *data_raw_ptrs.as_mut_ptr() }),
        }
    }
    pub unsafe fn to_vec(&self) -> Vec<Vec<i32>> {
        let mut vec = Vec::with_capacity(self.ptr.size);
        for i in 0..(*self.ptr).size {
            let temp = (*(*self.ptr).data.wrapping_add(i));
            vec.push(slice::from_raw_parts(temp.data, temp.size).to_vec());
        }
        return vec;
    }
}

impl Default for TwoDimArrayInt32Wrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_TwoDimArrayInt32 { size: 0, data: OneDimArrayInt32Wrapper::default().ptr.as_mut() })
        }
    }
}

impl Drop for TwoDimArrayInt32Wrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyTwoDimArrayInt32(self.ptr.as_mut())
        }
    }
}

pub struct TwoDimArrayFloatWrapper {
    pub ptr: *mut FD_C_TwoDimArrayFloat,
}

// impl TwoDimArrayFloatWrapper {
//     pub unsafe fn build(vec: &mut Vec<Vec<f32>>) -> TwoDimArrayFloatWrapper {
//         let mut data_raw_ptrs = vec![std::ptr::null_mut(); vec.len()];
//         for i in 0..vec.len() {
//             data_raw_ptrs[i] = OneDimArrayFloatWrapper::build(&mut vec[i]).ptr
//         }
//         TwoDimArrayFloatWrapper {
//             ptr: &mut FD_C_TwoDimArrayFloat { size: vec.len(), data: *data_raw_ptrs.as_mut_ptr() },
//         }
//     }
//     pub unsafe fn to_vec(&self) -> Vec<Vec<f32>> {
//         let mut vec = Vec::with_capacity((*self.ptr).size);
//         for i in 0..(*self.ptr).size {
//             let temp = (*(*self.ptr).data.wrapping_add(i));
//             vec.push(Vec::from_raw_parts(temp.data, temp.size, temp.size));
//         }
//         return vec;
//     }
// }
//
// impl Drop for TwoDimArrayFloatWrapper {
//     fn drop(&mut self) {
//         unsafe {
//             FD_C_DestroyTwoDimArrayFloat(self.ptr)
//         }
//     }
// }

pub struct TwoDimArraySizeWrapper {
    ptr: *mut FD_C_TwoDimArraySize,
}

impl TwoDimArraySizeWrapper {
    pub unsafe fn build(vec: &mut Vec<Vec<usize>>) -> TwoDimArraySizeWrapper {
        let mut data_raw_ptrs = vec![std::ptr::null_mut(); vec.len()];
        for i in 0..vec.len() {
            data_raw_ptrs[i] = OneDimArraySizeWrapper::build(&mut vec[i]).ptr
        }
        TwoDimArraySizeWrapper {
            ptr: &mut FD_C_TwoDimArraySize { size: vec.len(), data: *data_raw_ptrs.as_mut_ptr() },
        }
    }
    pub unsafe fn to_vec(&self) -> Vec<Vec<usize>> {
        let mut vec = Vec::with_capacity((*self.ptr).size);
        for i in 0..(*self.ptr).size {
            let temp = (*(*self.ptr).data.wrapping_add(i));
            vec.push(Vec::from_raw_parts(temp.data, temp.size, temp.size));
        }
        return vec;
    }
}

impl Drop for TwoDimArraySizeWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyTwoDimArraySize(self.ptr)
        }
    }
}

pub struct ThreeDimArrayInt32Wrapper {
    pub ptr: Box<FD_C_ThreeDimArrayInt32>,
}

impl ThreeDimArrayInt32Wrapper {
    pub unsafe fn build(vec: &mut Vec<Vec<Vec<i32>>>) -> ThreeDimArrayInt32Wrapper {
        let mut data_raw_ptrs = vec![std::ptr::null_mut(); vec.len()];
        for i in 0..vec.len() {
            data_raw_ptrs[i] = TwoDimArrayInt32Wrapper::build(&mut vec[i]).ptr.as_mut()
        }
        ThreeDimArrayInt32Wrapper {
            ptr: Box::new(FD_C_ThreeDimArrayInt32 { size: vec.len(), data: *data_raw_ptrs.as_mut_ptr() }),
        }
    }
    pub fn to_vec(&self) -> Vec<Vec<Vec<i32>>> {
        let mut vec = Vec::with_capacity(self.ptr.size);
        for i in 0..self.ptr.size {
            let p = self.ptr.data.wrapping_add(i);
            unsafe {
                let mut vec_ = Vec::with_capacity((*p).size);
                for j in 0..(*p).size {
                    let temp = (*(*p).data.wrapping_add(i));

                    vec_.push(slice::from_raw_parts(temp.data, temp.size).to_vec());
                }
                vec.push(vec_);
            }
        }
        return vec;
    }
}

impl Default for ThreeDimArrayInt32Wrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_ThreeDimArrayInt32 { size: 0, data: TwoDimArrayInt32Wrapper::default().ptr.as_mut() })
        }
    }
}

impl Drop for ThreeDimArrayInt32Wrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyThreeDimArrayInt32(self.ptr.as_mut())
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

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyMat(self.ptr)
        }
    }
}

pub struct OneDimMat {
    pub ptr: *mut FD_C_OneDimMat,
}

impl OneDimMat {
    pub fn new() -> OneDimMat {
        OneDimMat {
            ptr: &mut FD_C_OneDimMat { size: 0, data: std::ptr::null_mut() },
        }
    }
}

impl Drop for OneDimMat {
    fn drop(&mut self) {
        println!("start drop onedimmat");
        unsafe {
            FD_C_DestroyOneDimMat(self.ptr)
        }
        println!("end drop onedimmat");
    }
}

pub struct Mask {
    pub ptr: *mut FD_C_Mask,
}

impl Mask {}


pub struct OneDimMask {
    pub ptr: *mut FD_C_OneDimMask,
}

impl OneDimMask {}

pub struct CstrWrapper {
    pub ptr: Box<FD_C_Cstr>,
}

impl CstrWrapper {
    pub fn to_str(mut self) -> Result<&'static str, Utf8Error> {
        unsafe {
            CStr::from_ptr(self.ptr.data as *const c_char).to_str()
        }
    }
}


impl Default for CstrWrapper {
    fn default() -> Self {
        unsafe {
            let s = Self {
                ptr: Box::new(FD_C_Cstr { size: 0, data: std::ptr::null_mut() }),
            };
            return s;
        }
    }
}

impl From<&str> for CstrWrapper {
    fn from(data: &str) -> Self {
        unsafe {
            Self {
                ptr: Box::new(FD_C_Cstr { size: data.len(), data: CString::new(data).unwrap().into_raw() })
            }
        }
    }
}

impl Drop for CstrWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyCstr(self.ptr.as_mut())
        }
    }
}

pub struct OneDimArrayCstrWrapper {
    pub ptr: Box<FD_C_OneDimArrayCstr>,
}

impl OneDimArrayCstrWrapper {
    pub fn to_vec(mut self) -> Vec<&'static str> {
        unsafe {
            let mut result = Vec::with_capacity(self.ptr.size);
            for i in 0..self.ptr.size {
                result.push(CStr::from_ptr((*self.ptr.data.wrapping_add(i)).data).to_str().unwrap())
            };
            return result;
        }
    }
}

impl Default for OneDimArrayCstrWrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_OneDimArrayCstr {
                size: 0,
                data: CstrWrapper::default().ptr.as_mut(),
            })
        }
    }
}

impl Drop for OneDimArrayCstrWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyOneDimArrayCstr(self.ptr.as_mut())
        }
    }
}

pub struct TwoDimArrayCstrWrapper {
    pub ptr: Box<FD_C_TwoDimArrayCstr>,
}

impl TwoDimArrayCstrWrapper {
    pub fn to_vec(mut self) -> Vec<Vec<&'static str>> {
        unsafe {
            let mut result = Vec::with_capacity(self.ptr.size);
            for i in 0..self.ptr.size {
                let p = self.ptr.data.wrapping_add(i);
                let mut vec = Vec::with_capacity((*p).size);
                for j in 0..(*p).size {
                    vec.push(CStr::from_ptr((*(*p).data.wrapping_add(j)).data).to_str().unwrap())
                }
                result.push(vec);
            };
            return result;
        }
    }
}

impl Default for TwoDimArrayCstrWrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_TwoDimArrayCstr { size: 0, data: OneDimArrayCstrWrapper::default().ptr.as_mut() })
        }
    }
}

impl Drop for TwoDimArrayCstrWrapper {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyTwoDimArrayCstr(self.ptr.as_mut())
        }
    }
}



