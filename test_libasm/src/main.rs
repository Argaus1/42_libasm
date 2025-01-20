use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_ulonglong, c_int};
//use std::ptr;

extern "C" {
    fn ft_strlen(src: *const c_char) -> c_ulonglong;
    fn strlen(src: *const c_char) -> c_ulonglong;

    fn ft_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    fn ft_strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

fn rust_strcpy(dest: &mut [u8], src: &str, f: unsafe extern "C" fn(*mut c_char, *const c_char) -> *mut c_char) -> String {
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

fn rust_strcmp(str1: &str, str2: &str, f: unsafe extern "C" fn(*const c_char, *const c_char) -> c_int) -> i32 {
    let str1_c = CString::new(str1).expect("Failed to create CString");
    let str2_c = CString::new(str2).expect("Failed to create CString");

    unsafe {
        let result = f(str1_c.as_ptr(), str2_c.as_ptr());
        result as i32
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

    #[test]
    fn test_strcmp_equal() {
        let s1 = "zouzou";
        let s2 = "zouzou";
        let result_ft_strcmp = rust_strcmp(s1, s2, ft_strcmp);
        let result_strcmp = rust_strcmp(s1, s2, strcmp);
        assert_eq!(result_ft_strcmp, result_strcmp);
    }

    #[test]
    fn test_strcmp_not_equal() {
        let s1 = "zouzou";
        let s2 = "zouzov";
        let result_ft_strcmp = rust_strcmp(s1, s2, ft_strcmp);
        let result_strcmp = rust_strcmp(s1, s2, strcmp);
        assert_eq!(result_ft_strcmp, result_strcmp);
    }

    #[test]
    fn test_strcmp_first_one_empty() {
        let s1 = "";
        let s2 = "zouzou";
        let result_ft_strcmp = rust_strcmp(s1, s2, ft_strcmp);
        let result_strcmp = rust_strcmp(s1, s2, strcmp);
        assert_eq!(result_ft_strcmp, result_strcmp);
    }

    #[test]
    fn test_strcmp_scnd_one_empty() {
        let s1 = "";
        let s2 = "zouzou";
        let result_ft_strcmp = rust_strcmp(s1, s2, ft_strcmp);
        let result_strcmp = rust_strcmp(s1, s2, strcmp);
        assert_eq!(result_ft_strcmp, result_strcmp);
    }
    
    #[test]
    fn test_strcmp_two_empty() {
        let s1 = "";
        let s2 = "";
        let result_ft_strcmp = rust_strcmp(s1, s2, ft_strcmp);
        let result_strcmp = rust_strcmp(s1, s2, strcmp);
        assert_eq!(result_ft_strcmp, result_strcmp);
    }

    #[test]
    fn test_strcmp_special_chars() {
        let s1 = "Hello\nWorld\t!";
        let s2 = "Hello\nWorld\t!";
        let result_ft_strcmp = rust_strcmp(s1, s2, ft_strcmp);
        let result_strcmp = rust_strcmp(s1, s2, strcmp);
        assert_eq!(result_ft_strcmp, result_strcmp);
    }

    // WRITE

    // basic
    // neg fd
    // neg nbr of char
    // empty string
}

