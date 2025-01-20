use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_ulonglong};
//use std::ptr;

extern "C" {
    fn ft_strlen(src: *const c_char) -> c_ulonglong;
    fn strlen(src: *const c_char) -> c_ulonglong;

    fn ft_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    // fn ft_strcmp(s1: *const u8, s2: *const u8) -> c_int;
    // fn strcmp(s1: *const u8, s2: *const u8) -> c_int;
}

fn rust_strcpy(dest: &mut [u8], src: &str, f: unsafe extern "C" fn(*mut c_char, *const c_char) -> *mut c_char) -> String {
    // Ensure the source is null-terminated
    let src_c = CString::new(src).expect("Failed to create CString");
    let dest_ptr = dest.as_mut_ptr() as *mut c_char;

    unsafe {
        f(dest_ptr, src_c.as_ptr());
        let result = CStr::from_ptr(dest_ptr).to_string_lossy().to_string();
        result
    }
}

fn rust_strlen(src: &str, f: unsafe extern "C" fn(*const c_char) -> c_ulonglong) -> usize {
    let src_c = CString::new(src).expect("Failed to create CString");

    unsafe {
        let res_c = f(src_c.as_ptr());
        res_c as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// STRLEN

    #[test]
    fn test_strlen_basic() {
        let src = "Hello, world!";
        let result_ft_strlen = rust_strlen(src, ft_strlen);
        let result_strlen = rust_strlen(src, strlen);
        assert_eq!(result_ft_strlen, result_strlen);
    }

    #[test]
    fn test_strlen_empty() {
        let src = "";
        let result_ft_strlen = rust_strlen(src, ft_strlen);
        let result_strlen = rust_strlen(src, strlen);
        assert_eq!(result_ft_strlen, result_strlen);
    }

    #[test]
    fn test_strlen_special_chars() {
        let src = "\n\t\r";
        let result_ft_strlen = rust_strlen(src, ft_strlen);
        let result_strlen = rust_strlen(src, strlen);
        assert_eq!(result_ft_strlen, result_strlen);
    }

// STRCPY

    #[test]
    fn test_strcpy_basic() {
        let mut dest = [0u8; 100];
        let src = "ZOUZOU";
        let result_ft_strcpy = rust_strcpy(&mut dest, src, ft_strcpy);
        let result_strcpy = rust_strcpy(&mut dest, src, strcpy);
        assert_eq!(result_ft_strcpy, result_strcpy);
    }

    #[test]
    fn test_strcpy_empty() {
        let mut dest = [0u8; 100];
        let src = "";
        let result_ft_strcpy = rust_strcpy(&mut dest, src, ft_strcpy);
        let result_strcpy = rust_strcpy(&mut dest, src, strcpy);
        assert_eq!(result_ft_strcpy, result_strcpy);
    }

    #[test]
    fn test_strcpy_with_special_chars() {
        let mut dest = [0u8; 100];
        let src = "Hello\nWorld\t!";
        let result_ft_strcpy = rust_strcpy(&mut dest, src, ft_strcpy);
        let result_strcpy = rust_strcpy(&mut dest, src, strcpy);
        assert_eq!(result_ft_strcpy, result_strcpy);
    }

    #[test]
    fn test_strcpy_with_dest_shorter() {
        let mut dest = [6u8; 5];
        let src = "Hello\nWorld\t!";
        let result_ft_strcpy = rust_strcpy(&mut dest, src, ft_strcpy);
        let result_strcpy = rust_strcpy(&mut dest, src, strcpy);
        assert_eq!(result_ft_strcpy, result_strcpy);
    }

    #[test]
    fn test_strcpy_both_empty() {
        let mut dest = [0u8; 0];
        let src = "";
        let result_ft_strcpy = rust_strcpy(&mut dest, src, ft_strcpy);
        let result_strcpy = rust_strcpy(&mut dest, src, strcpy);
        assert_eq!(result_ft_strcpy, result_strcpy);
    }

    // STRCMP
    
}

