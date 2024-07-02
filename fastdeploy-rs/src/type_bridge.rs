use std::ffi::{c_char, CStr, CString};
use std::str::Utf8Error;

use fastdeploy_bind::*;

pub mod common {
    use std::ffi::{CStr, CString};
    use std::slice;

    use fastdeploy_bind::{FD_C_Bool, FD_C_Cstr, FD_C_OneDimArrayCstr, FD_C_OneDimArrayFloat,
                          FD_C_OneDimArrayInt32, FD_C_OneDimArrayInt64, FD_C_OneDimArrayUint8,
                          FD_C_TwoDimArrayFloat, FD_C_TwoDimArrayInt32};

    #[inline]
    pub fn fd_c_bool_to_bool(ret: FD_C_Bool) -> bool {
        ret >= 1
    }

    pub fn fd_c_cstr_to_string(str: FD_C_Cstr) -> String {
        unsafe {
            CStr::from_ptr(str.data).to_str().unwrap().to_string()
        }
    }

    pub fn fd_one_dim_array_c_str_to_vec_string(array: FD_C_OneDimArrayCstr) -> Vec<String> {
        unsafe {
            let mut result = Vec::with_capacity(array.size);
            for i in 0..array.size {
                result.push(CStr::from_ptr((*array.data.wrapping_add(i)).data).to_str().unwrap().to_string());
            };
            return result;
        }
    }

    pub fn fd_c_one_dim_array_float_to_vec_f32(array: FD_C_OneDimArrayFloat) -> Vec<f32> {
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
                result.push(fd_c_one_dim_array_float_to_vec_f32(*ptr))
            }
            return result;
        }
    }

    pub fn fd_c_two_dim_array_int32_to_vec_i32(array: FD_C_TwoDimArrayInt32) -> Vec<Vec<i32>> {
        unsafe {
            let mut result = vec![];
            for i in 0..array.size {
                let ptr = array.data.wrapping_add(i);
                result.push(fd_c_one_dim_array_int32_to_vec_i32(*ptr))
            }
            return result;
        }
    }


    //--------------------------------------------------------------------------------------------

    #[inline]
    pub fn bool_to_fd_c_bool(ret: bool) -> FD_C_Bool {
        return if ret { 1 } else { 0 };
    }

    pub fn string_to_fd_c_cstr(str: &str) -> FD_C_Cstr {
        unsafe {
            FD_C_Cstr {
                size: str.len(),
                data: CString::new(str).unwrap().into_raw(),
            }
        }
    }

    pub fn vec_string_to_fd_one_dim_array_c_str(array: Vec<&str>) -> FD_C_OneDimArrayCstr {
        unsafe {
            let mut result = Vec::with_capacity(array.len());
            for i in 0..array.len() {
                result.push(string_to_fd_c_cstr(array[i]));
            }
            return FD_C_OneDimArrayCstr {
                size: array.len(),
                data: result.as_mut_ptr(),
            };
        }
    }


    pub fn vec_f32_to_fd_c_one_dim_array_float(array: Vec<f32>) -> FD_C_OneDimArrayFloat {
        let len = array.len();
        let ptr = Box::into_raw(array.into_boxed_slice()) as *mut f32;
        FD_C_OneDimArrayFloat { size: len, data: ptr }
    }


    pub fn vec_i32_to_fd_c_one_dim_array_int32(array: Vec<i32>) -> FD_C_OneDimArrayInt32 {
        let mut s = array.clone();
        unsafe {
            FD_C_OneDimArrayInt32 {
                size: array.len(),
                data: s.as_mut_ptr(),
            }
        }
    }

    pub fn vec_i64_to_fd_c_one_dim_array_int64(array: Vec<i64>) -> FD_C_OneDimArrayInt64 {
        let mut s = array.clone();
        unsafe {
            FD_C_OneDimArrayInt64 {
                size: array.len(),
                data: s.as_mut_ptr(),
            }
        }
    }

    pub fn vec_u8_to_fd_c_one_dim_array_uint8(array: Vec<u8>) -> FD_C_OneDimArrayUint8 {
        let mut s = array.clone();
        unsafe {
            FD_C_OneDimArrayUint8 {
                size: array.len(),
                data: s.as_mut_ptr(),
            }
        }
    }

    pub fn vec_f32_to_fd_c_two_dim_array_float(array: Vec<Vec<f32>>) -> FD_C_TwoDimArrayFloat {
        unsafe {
            let len = array.len();
            let mut inner_arrays = Vec::with_capacity(len);
            for i in 0..array.len() {
                inner_arrays.push(vec_f32_to_fd_c_one_dim_array_float(array[i].clone()));
            }
            let mut inner_arrays_raw = inner_arrays.into_boxed_slice();
            let ptr = Box::into_raw(inner_arrays_raw) as *mut FD_C_OneDimArrayFloat;
            FD_C_TwoDimArrayFloat { size: len, data: ptr }
        }
    }

    pub fn vec_i32_to_fd_c_two_dim_array_int32(array: Vec<Vec<i32>>) -> FD_C_TwoDimArrayInt32 {
        unsafe {
            let mut inner_arrays = Vec::with_capacity(array.len());
            for i in 0..array.len() {
                inner_arrays.push(vec_i32_to_fd_c_one_dim_array_int32(array[i].clone()));
            }
            let mut inner_arrays_raw = inner_arrays.into_boxed_slice();
            let ptr = inner_arrays_raw.as_mut_ptr();
            let len = inner_arrays_raw.len();
            std::mem::forget(inner_arrays_raw);
            FD_C_TwoDimArrayInt32 { size: len, data: ptr }
        }
    }
}

#[derive(Debug, Clone)]
pub struct OneDimArrayUint8Wrapper {
    pub ptr: Box<FD_C_OneDimArrayUint8>,
}

impl Default for OneDimArrayUint8Wrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_OneDimArrayUint8 { size: 0, data: std::ptr::null_mut() }),
        }
    }
}

impl From<Vec<u8>> for OneDimArrayUint8Wrapper {
    fn from(vec: Vec<u8>) -> Self {
        let size = vec.len();
        let mut vec = vec; // 确保vec可变
        let data = vec.as_mut_ptr();
        std::mem::forget(vec); // 忘记vec以避免它被释放
        OneDimArrayUint8Wrapper {
            ptr: Box::new(FD_C_OneDimArrayUint8 { size, data }),
        }
    }
}


impl Into<Vec<u8>> for OneDimArrayUint8Wrapper {
    fn into(self) -> Vec<u8> {
        let size = self.ptr.size;
        let data = self.ptr.data;
        // 将Box指针转换为raw指针以避免Box自动释放
        let data = unsafe { Vec::from_raw_parts(data, size, size) };
        data
    }
}


impl Drop for OneDimArrayUint8Wrapper {
    fn drop(&mut self) {
        let size = self.ptr.size;
        let data = self.ptr.data;
        unsafe {
            Vec::from_raw_parts(data, size, size);
        };
    }
}


#[derive(Debug, Clone)]
pub struct OneDimArrayInt32Wrapper {
    pub ptr: Box<FD_C_OneDimArrayInt32>,
}

impl Default for OneDimArrayInt32Wrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_OneDimArrayInt32 { size: 0, data: std::ptr::null_mut() }),
        }
    }
}

impl From<Vec<i32>> for OneDimArrayInt32Wrapper {
    fn from(vec: Vec<i32>) -> Self {
        let size = vec.len();
        let mut vec = vec; // 确保vec可变
        let data = vec.as_mut_ptr();
        std::mem::forget(vec); // 忘记vec以避免它被释放
        OneDimArrayInt32Wrapper {
            ptr: Box::new(FD_C_OneDimArrayInt32 { size, data }),
        }
    }
}


impl Into<Vec<i32>> for OneDimArrayInt32Wrapper {
    fn into(self) -> Vec<i32> {
        let size = self.ptr.size;
        let data = self.ptr.data;
        // 将Box指针转换为raw指针以避免Box自动释放
        let data = unsafe { Vec::from_raw_parts(data, size, size) };
        data
    }
}


impl Drop for OneDimArrayInt32Wrapper {
    fn drop(&mut self) {
        let size = self.ptr.size;
        let data = self.ptr.data;
        unsafe {
            Vec::from_raw_parts(data, size, size);
        };
    }
}


#[derive(Debug, Clone)]
pub struct OneDimArraySizeWrapper {
    pub ptr: Box<FD_C_OneDimArraySize>,
}

impl Default for OneDimArraySizeWrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_OneDimArraySize { size: 0, data: std::ptr::null_mut() }),
        }
    }
}

impl From<Vec<usize>> for OneDimArraySizeWrapper {
    fn from(vec: Vec<usize>) -> Self {
        let size = vec.len();
        let mut vec = vec; // 确保vec可变
        let data = vec.as_mut_ptr();
        std::mem::forget(vec); // 忘记vec以避免它被释放
        OneDimArraySizeWrapper {
            ptr: Box::new(FD_C_OneDimArraySize { size, data }),
        }
    }
}


impl Into<Vec<usize>> for OneDimArraySizeWrapper {
    fn into(self) -> Vec<usize> {
        let size = self.ptr.size;
        let data = self.ptr.data;
        // 将Box指针转换为raw指针以避免Box自动释放
        let data = unsafe { Vec::from_raw_parts(data, size, size) };
        data
    }
}


impl Drop for OneDimArraySizeWrapper {
    fn drop(&mut self) {
        let size = self.ptr.size;
        let data = self.ptr.data;
        unsafe {
            Vec::from_raw_parts(data, size, size);
        };
    }
}

#[derive(Debug, Clone)]
pub struct OneDimArrayFloatWrapper {
    pub ptr: Box<FD_C_OneDimArrayFloat>,
}

impl Default for OneDimArrayFloatWrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_OneDimArrayFloat { size: 0, data: std::ptr::null_mut() }),
        }
    }
}

impl From<Vec<f32>> for OneDimArrayFloatWrapper {
    fn from(vec: Vec<f32>) -> Self {
        let size = vec.len();
        let mut vec = vec; // 确保vec可变
        let data = vec.as_mut_ptr();
        std::mem::forget(vec); // 忘记vec以避免它被释放
        OneDimArrayFloatWrapper {
            ptr: Box::new(FD_C_OneDimArrayFloat { size, data }),
        }
    }
}


impl Into<Vec<f32>> for OneDimArrayFloatWrapper {
    fn into(self) -> Vec<f32> {
        let size = self.ptr.size;
        let data = self.ptr.data;
        // 将Box指针转换为raw指针以避免Box自动释放
        let data = unsafe { Vec::from_raw_parts(data, size, size) };
        data
    }
}


impl Drop for OneDimArrayFloatWrapper {
    fn drop(&mut self) {
        let size = self.ptr.size;
        let data = self.ptr.data;
        // 将raw指针转换为Box<[f32]>，以便在超出作用域时自动释放
        unsafe {
            Vec::from_raw_parts(data, size, size);
        };
    }
}

#[derive(Debug, Clone)]
pub struct OneDimArrayInt8Wrapper {
    pub ptr: Box<FD_C_OneDimArrayInt8>,
}

impl Default for OneDimArrayInt8Wrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_OneDimArrayInt8 { size: 0, data: std::ptr::null_mut() }),
        }
    }
}

impl From<Vec<i8>> for OneDimArrayInt8Wrapper {
    fn from(vec: Vec<i8>) -> Self {
        let size = vec.len();
        let mut vec = vec; // 确保vec可变
        let data = vec.as_mut_ptr();
        std::mem::forget(vec); // 忘记vec以避免它被释放
        OneDimArrayInt8Wrapper {
            ptr: Box::new(FD_C_OneDimArrayInt8 { size, data }),
        }
    }
}


impl Into<Vec<i8>> for OneDimArrayInt8Wrapper {
    fn into(self) -> Vec<i8> {
        let size = self.ptr.size;
        let data = self.ptr.data;
        // 将Box指针转换为raw指针以避免Box自动释放
        let data = unsafe { Vec::from_raw_parts(data, size, size) };
        data
    }
}


impl Drop for OneDimArrayInt8Wrapper {
    fn drop(&mut self) {
        let size = self.ptr.size;
        let data = self.ptr.data;
        unsafe {
            Vec::from_raw_parts(data, size, size);
        };
    }
}

#[derive(Debug, Clone)]
pub struct TwoDimArrayInt32Wrapper {
    pub ptr: Box<FD_C_TwoDimArrayInt32>,
}


impl Default for TwoDimArrayInt32Wrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_TwoDimArrayInt32 { data: std::ptr::null_mut(), size: 0 })
        }
    }
}

// 实现From<Vec<Vec<i32>>>，将Vec<Vec<i32>>转换为TwoDimArrayInt32Wrapper
impl From<Vec<Vec<i32>>> for TwoDimArrayInt32Wrapper {
    fn from(vec: Vec<Vec<i32>>) -> Self {
        let size = vec.len();

        let mut raw_pointers: Vec<FD_C_OneDimArrayInt32> = vec.into_iter()
            .map(|v| unsafe { std::ptr::read(OneDimArrayInt32Wrapper::from(v).ptr.as_mut()) })
            .collect();
        let data_ptr = raw_pointers.as_mut_ptr();
        std::mem::forget(raw_pointers); // 忘记raw_pointers以避免它们被释放
        TwoDimArrayInt32Wrapper {
            ptr: Box::new(FD_C_TwoDimArrayInt32 { size, data: data_ptr }),
        }
    }
}

// 实现Into<Vec<Vec<i32>>>，将TwoDimArrayInt32Wrapper转换为Vec<Vec<i32>>
impl Into<Vec<Vec<i32>>> for TwoDimArrayInt32Wrapper {
    fn into(self) -> Vec<Vec<i32>> {
        let size = self.ptr.size;
        let data = self.ptr.data;
        let wrappers = unsafe { Vec::from_raw_parts(data, size, size) };
        wrappers.into_iter().map(|w| {
            let wrapper = OneDimArrayInt32Wrapper { ptr: Box::new(w) };
            wrapper.into()
        }).collect()
    }
}

// 实现Drop trait以便释放FD_C_TwoDimArrayInt32内部的动态内存
impl Drop for TwoDimArrayInt32Wrapper {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.ptr.data, self.ptr.size, self.ptr.size) };
    }
}


#[derive(Debug, Clone)]
pub struct TwoDimArrayFloatWrapper {
    pub ptr: Box<FD_C_TwoDimArrayFloat>,
}

// 实现From<Vec<Vec<f32>>>，将Vec<Vec<f32>>转换为TwoDimArrayFloatWrapper
impl From<Vec<Vec<f32>>> for TwoDimArrayFloatWrapper {
    fn from(vec: Vec<Vec<f32>>) -> Self {
        let size = vec.len();

        let mut raw_pointers: Vec<FD_C_OneDimArrayFloat> = vec.into_iter()
            .map(|v| unsafe { std::ptr::read(OneDimArrayFloatWrapper::from(v).ptr.as_mut()) })
            .collect();
        let data_ptr = raw_pointers.as_mut_ptr();
        std::mem::forget(raw_pointers); // 忘记raw_pointers以避免它们被释放
        TwoDimArrayFloatWrapper {
            ptr: Box::new(FD_C_TwoDimArrayFloat { size, data: data_ptr }),
        }
    }
}

// 实现Into<Vec<Vec<f32>>>，将TwoDimArrayFloatWrapper转换为Vec<Vec<f32>>
impl Into<Vec<Vec<f32>>> for TwoDimArrayFloatWrapper {
    fn into(self) -> Vec<Vec<f32>> {
        let size = self.ptr.size;
        let data = self.ptr.data;
        let wrappers = unsafe { Vec::from_raw_parts(data, size, size) };
        wrappers.into_iter().map(|w| {
            let wrapper = OneDimArrayFloatWrapper { ptr: Box::new(w) };
            wrapper.into()
        }).collect()
    }
}

// 实现Drop trait以便释放FD_C_TwoDimArrayFloat内部的动态内存
impl Drop for TwoDimArrayFloatWrapper {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.ptr.data, self.ptr.size, self.ptr.size) };
    }
}


#[derive(Debug, Clone)]
pub struct TwoDimArraySizeWrapper {
    pub ptr: Box<FD_C_TwoDimArraySize>,
}

// 实现From<Vec<Vec<usize>>>，将Vec<Vec<usize>>转换为TwoDimArraySizeWrapper
impl From<Vec<Vec<usize>>> for TwoDimArraySizeWrapper {
    fn from(vec: Vec<Vec<usize>>) -> Self {
        let size = vec.len();

        let mut raw_pointers: Vec<FD_C_OneDimArraySize> = vec.into_iter()
            .map(|v| unsafe { std::ptr::read(OneDimArraySizeWrapper::from(v).ptr.as_mut()) })
            .collect();
        let data_ptr = raw_pointers.as_mut_ptr();
        std::mem::forget(raw_pointers); // 忘记raw_pointers以避免它们被释放
        TwoDimArraySizeWrapper {
            ptr: Box::new(FD_C_TwoDimArraySize { size, data: data_ptr }),
        }
    }
}

// 实现Into<Vec<Vec<usize>>>，将TwoDimArraySizeWrapper转换为Vec<Vec<usize>>
impl Into<Vec<Vec<usize>>> for TwoDimArraySizeWrapper {
    fn into(self) -> Vec<Vec<usize>> {
        let size = self.ptr.size;
        let data = self.ptr.data;
        let wrappers = unsafe { Vec::from_raw_parts(data, size, size) };
        wrappers.into_iter().map(|w| {
            let wrapper = OneDimArraySizeWrapper { ptr: Box::new(w) };
            wrapper.into()
        }).collect()
    }
}

// 实现Drop trait以便释放FD_C_TwoDimArraySize内部的动态内存
impl Drop for TwoDimArraySizeWrapper {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.ptr.data, self.ptr.size, self.ptr.size) };
    }
}


pub struct ThreeDimArrayInt32Wrapper {
    pub ptr: Box<FD_C_ThreeDimArrayInt32>,
}

impl Default for ThreeDimArrayInt32Wrapper {
    fn default() -> Self {
        Self {
            ptr: Box::new(FD_C_ThreeDimArrayInt32 { size: 0, data: std::ptr::null_mut() })
        }
    }
}

impl From<Vec<Vec<Vec<i32>>>> for ThreeDimArrayInt32Wrapper {
    fn from(vec: Vec<Vec<Vec<i32>>>) -> Self {
        let size = vec.len();

        let mut raw_pointers: Vec<FD_C_TwoDimArrayInt32> = vec.into_iter()
            .map(|v| unsafe { std::ptr::read(TwoDimArrayInt32Wrapper::from(v).ptr.as_mut()) })
            .collect();
        let data_ptr = raw_pointers.as_mut_ptr();
        std::mem::forget(raw_pointers); // 忘记raw_pointers以避免它们被释放
        ThreeDimArrayInt32Wrapper {
            ptr: Box::new(FD_C_ThreeDimArrayInt32 { size, data: data_ptr }),
        }
    }
}

impl Into<Vec<Vec<Vec<i32>>>> for ThreeDimArrayInt32Wrapper {
    fn into(self) -> Vec<Vec<Vec<i32>>> {
        let size = self.ptr.size;
        let data = self.ptr.data;
        let wrappers = unsafe { Vec::from_raw_parts(data, size, size) };
        wrappers.into_iter().map(|w| {
            let wrapper = TwoDimArrayInt32Wrapper { ptr: Box::new(w) };
            wrapper.into()
        }).collect()
    }
}

impl Drop for ThreeDimArrayInt32Wrapper {
    fn drop(&mut self) {
        unsafe {
            Vec::from_raw_parts(self.ptr.data, self.ptr.size, self.ptr.size);
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
        println!("drop single");
        unsafe {
            FD_C_DestroyMat(self.ptr)
        }
    }
}

pub struct OneDimMatWrapper {
    pub ptr: Box<FD_C_OneDimMat>,
}

impl OneDimMatWrapper {
    pub fn new() -> OneDimMatWrapper {
        OneDimMatWrapper {
            ptr: Box::new(FD_C_OneDimMat { size: 0, data: std::ptr::null_mut() }),
        }
    }
    pub fn to_vec(self) -> Vec<Mat> {
        let mut ret = Vec::with_capacity(self.ptr.size);

        for i in 0..self.ptr.size {
            unsafe {
                ret.push(Mat { ptr: *self.ptr.data.wrapping_add(i) })
            }
        };
        return ret;
    }
}

impl From<&mut Vec<Mat>> for OneDimMatWrapper {
    fn from(mut value: &mut Vec<Mat>) -> Self {
        unsafe {
            let s = FD_C_OneDimMat { size: value.len(), data: &mut (*value.as_mut_ptr()).ptr };
            Self {
                ptr: Box::new(s)
            }
        }
    }
}


// impl Drop for OneDimMatWrapper {
//     fn drop(&mut self) {
//         println!("start drop onedimmat");
//         unsafe {
//             FD_C_DestroyOneDimMat(self.ptr.as_mut())
//         }
//         println!("end drop onedimmat");
//     }
// }

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
    pub fn new(value: String) -> Self {
        Self {
            ptr: Box::new(FD_C_Cstr {
                size: value.len(),
                data: CString::new(value).unwrap().into_raw(),
            }),
        }
    }
    pub fn to_str(mut self) -> Result<&'static str, Utf8Error> {
        unsafe {
            CStr::from_ptr(self.ptr.data as *const c_char).to_str()
        }
    }
}


impl Default for CstrWrapper {
    fn default() -> Self {
        let s = Self {
            ptr: Box::new(FD_C_Cstr { size: 0, data: std::ptr::null_mut() }),
        };
        return s;
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

impl From<FD_C_Cstr> for CstrWrapper {
    fn from(data: FD_C_Cstr) -> Self {
        unsafe {
            Self {
                ptr: Box::new(data)
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
    pub fn new(value: Vec<String>) -> Self {
        let mut result = Vec::with_capacity(value.len());
        for vec in &value {
            result.push(*CstrWrapper::new(vec.to_string()).ptr);
        }

        Self {
            ptr: Box::new(FD_C_OneDimArrayCstr {
                size: value.len(),
                data: result.as_mut_ptr(),
            }),
        }
    }
    pub fn to_vec(mut self) -> Vec<String> {
        unsafe {
            let mut result = Vec::with_capacity(self.ptr.size);
            for i in 0..self.ptr.size {
                result.push(CStr::from_ptr((*self.ptr.data.wrapping_add(i)).data).to_str().unwrap().to_string())
            };
            return result;
        }
    }
}

impl From<FD_C_OneDimArrayCstr> for OneDimArrayCstrWrapper {
    fn from(value: FD_C_OneDimArrayCstr) -> Self {
        Self {
            ptr: Box::new(value)
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



