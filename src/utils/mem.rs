//! Memory management utilities for FFI
//!
//! Safe wrappers for working with raw pointers across the FFI boundary.

use std::slice;

/// Safely create a slice from a raw pointer and length
///
/// # Safety
/// - `ptr` must be valid for reads of `len` bytes
/// - `ptr` must be properly aligned
/// - The memory must not be mutated for the lifetime of the returned slice
/// - The total size `len` must be no larger than `isize::MAX`
#[inline]
pub unsafe fn slice_from_raw_parts<'a>(ptr: *const u8, len: usize) -> Option<&'a [u8]> {
    if ptr.is_null() || len == 0 {
        return None;
    }

    Some(slice::from_raw_parts(ptr, len))
}

/// Box a byte vector and return a raw pointer to it
///
/// The caller is responsible for calling `free_byte_buffer` to deallocate
#[inline]
pub fn box_byte_vec(vec: Vec<u8>) -> (*const u8, usize) {
    let len = vec.len();
    let ptr = Box::into_raw(vec.into_boxed_slice()) as *const u8;
    (ptr, len)
}

/// Free a byte buffer allocated by `box_byte_vec`
///
/// # Safety
/// - `ptr` must have been allocated by `box_byte_vec`
/// - `len` must be the same length returned by `box_byte_vec`
/// - Must only be called once per pointer
#[inline]
pub unsafe fn free_byte_buffer(ptr: *const u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        let _ = Box::from_raw(slice::from_raw_parts_mut(ptr as *mut u8, len));
    }
}

/// Allocate a string and return a raw pointer
/// The caller must free it with `free_string`
#[inline]
pub fn box_string(s: String) -> *const std::os::raw::c_char {
    let c_string = std::ffi::CString::new(s).unwrap();
    c_string.into_raw()
}

/// Free a string allocated by `box_string`
///
/// # Safety
/// - `ptr` must have been allocated by `box_string`
/// - Must only be called once per pointer
#[inline]
pub unsafe fn free_string(ptr: *const std::os::raw::c_char) {
    if !ptr.is_null() {
        let _ = std::ffi::CString::from_raw(ptr as *mut std::os::raw::c_char);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_from_raw_parts_valid() {
        let data = vec![1u8, 2, 3, 4, 5];
        let ptr = data.as_ptr();
        let len = data.len();

        let slice = unsafe { slice_from_raw_parts(ptr, len) };

        assert!(slice.is_some());
        assert_eq!(slice.unwrap(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_slice_from_raw_parts_null() {
        let slice = unsafe { slice_from_raw_parts(std::ptr::null(), 10) };
        assert!(slice.is_none());
    }

    #[test]
    fn test_box_and_free_byte_vec() {
        let data = vec![10u8, 20, 30, 40];
        let (ptr, len) = box_byte_vec(data.clone());

        // Verify we can read the data
        let slice = unsafe { slice_from_raw_parts(ptr, len) }.unwrap();
        assert_eq!(slice, &[10, 20, 30, 40]);

        // Free the memory
        unsafe { free_byte_buffer(ptr, len) };
    }

    #[test]
    fn test_box_and_free_string() {
        let test_str = "Hello, FFI!".to_string();
        let ptr = box_string(test_str.clone());

        // Verify the string
        let c_str = unsafe { std::ffi::CStr::from_ptr(ptr) };
        assert_eq!(c_str.to_str().unwrap(), "Hello, FFI!");

        // Free the memory
        unsafe { free_string(ptr) };
    }
}
