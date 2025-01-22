use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_ulonglong, c_longlong, c_int, c_void};
//use std::ptr;
use std::fs;
use std::os::fd::AsRawFd;
use std::io;

extern "C" {
    fn ft_strlen(src: *const c_char) -> c_ulonglong;
    fn strlen(src: *const c_char) -> c_ulonglong;

    fn ft_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    fn ft_strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn ft_write(fd: c_int, buf: *const c_void, count: c_ulonglong) -> c_longlong;
    fn write(fd: c_int, buf: *const c_void, count: c_ulonglong) -> c_longlong;

    fn ft_read(fd: c_int, buf: *const c_void, count: c_ulonglong) -> c_longlong;
    fn read(fd: c_int, buf: *const c_void, count: c_ulonglong) -> c_longlong;
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

fn rust_write(fd: i32, buf: &str, len: usize, f: unsafe extern "C" fn(c_int, *const c_void, c_ulonglong) -> c_longlong) -> i64 {
    let len_c = len as u64;

    unsafe {
        let result = f(fd, buf.as_bytes().as_ptr() as *const c_void, len_c);
        result as i64
    }
}

fn rust_read(fd: i32, buf: &mut [u8], len: usize, f: unsafe extern "C" fn(c_int, *const c_void, c_ulonglong) -> c_longlong) -> i64 {
    let len_c = len as u64;
    let buf_c = buf.as_mut_ptr() as *const c_void;

    unsafe {
        let result = f(fd, buf_c, len_c);
        result as i64
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

    #[test]
    fn test_write_basic_stdout() {
        let s1 = "write_basic_stdout";
        let result_ft_write = rust_write(1, s1, s1.len(), ft_write);
        let result_write = rust_write(1, s1, s1.len(), write);
        assert_eq!(result_ft_write, result_write);
    }

    #[test]
    fn test_write_empty() {
        let s1 = "";
        let result_ft_write = rust_write(1, s1, s1.len(), ft_write);
        let result_write = rust_write(1, s1, s1.len(), write);
        assert_eq!(result_ft_write, result_write);
    }

    #[test]
    fn test_write_neg_fd() {
        let s1 = "neg_fd";
        let result_ft_write = rust_write(-1, s1, s1.len(), ft_write);
        let result_write = rust_write(-1, s1, s1.len(), write);
        assert_eq!(result_ft_write, result_write);
    }

    #[test]
    fn test_write_other_fd() -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::File::create("write_other_fd.txt")?;
        let file_2 = fs::File::create("ft_write_other_fd.txt")?;
        let s1 = "write_other_fd";
        let result_ft_write = rust_write(file_2.as_raw_fd(), s1, s1.len(), ft_write);
        let result_write = rust_write(file.as_raw_fd(), s1, s1.len(), write);
        assert_eq!(result_ft_write, result_write);
        Ok(())
    }

    #[test]
    fn test_write_smaller_nb_char() -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::File::create("write_smaller_nb_char.txt")?;
        let file_2 = fs::File::create("ft_write_smaller_nb_char.txt")?;
        let s1 = "write_smaller_nb_char";
        let result_ft_write = rust_write(file_2.as_raw_fd(), s1, 1, ft_write);
        let result_write = rust_write(file.as_raw_fd(), s1, 1, write);
        assert_eq!(result_ft_write, result_write);
        Ok(())
    }

    #[test]
    fn test_write_bigger_nb_char() -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::File::create("ft_write_bigger_nb_char.txt")?;
        let file_2 = fs::File::create("write_bigger_nb_char.txt")?;
        let s1 = "write_bigger_nb_char";
        let result_ft_write = rust_write(file.as_raw_fd(), s1, 100, ft_write);
        let result_write = rust_write(file_2.as_raw_fd(), s1, 100, write);
        assert_eq!(result_ft_write, result_write);
        Ok(())
    }

    // READ

    // #[test]
    // fn test_read_basic_stdin() {
    //     let s1 = "read_basic_stdout";
    //     let result_ft_read = rust_read(0, s1, s1.len(), ft_read);
    //     let result_read = rust_read(0, s1, s1.len(), read);
    //     assert_eq!(result_ft_read, result_read);
    // }
    #[test]
    fn test_read_basic_stdin() -> Result<(), Box<dyn std::error::Error>> {
        let mut dest = [0u8; 100];
        let result_ft_read = rust_read(0, &mut dest, 100, ft_read);
        let result_read = rust_read(0, &mut dest, 100, read);
        assert_eq!(result_ft_read, result_read);
        Ok(())
    }

    #[test]
    fn test_read_basic() -> Result<(), Box<dyn std::error::Error>> {
        let mut dest = [0u8; 100];
        let file = fs::File::open("read_other_fd.txt")?;
        let result_ft_read = rust_read(file.as_raw_fd(), &mut dest, 100, ft_read);
        let result_read = rust_read(file.as_raw_fd(), &mut dest, 100, read);
        assert_eq!(result_ft_read, result_read);
        Ok(())
    }

    // #[test]
    // fn test_read_neg_fd() {
    //     let s1 = "neg_fd";
    //     let result_ft_read = rust_read(-1, s1, s1.len(), ft_read);
    //     let result_read = rust_read(-1, s1, s1.len(), read);
    //     assert_eq!(result_ft_read, result_read);
    // }

    // #[test]
    // fn test_read_other_fd() -> Result<(), Box<dyn std::error::Error>> {
    //     let file = fs::File::create("read_other_fd.txt")?;
    //     let file_2 = fs::File::create("ft_read_other_fd.txt")?;
    //     let s1 = "read_other_fd";
    //     let result_ft_read = rust_read(file_2.as_raw_fd(), s1, s1.len(), ft_read);
    //     let result_read = rust_read(file.as_raw_fd(), s1, s1.len(), read);
    //     assert_eq!(result_ft_read, result_read);
    //     Ok(())
    // }

    // #[test]
    // fn test_read_smaller_nb_char() -> Result<(), Box<dyn std::error::Error>> {
    //     let file = fs::File::create("read_smaller_nb_char.txt")?;
    //     let file_2 = fs::File::create("ft_read_smaller_nb_char.txt")?;
    //     let s1 = "read_smaller_nb_char";
    //     let result_ft_read = rust_read(file_2.as_raw_fd(), s1, 1, ft_read);
    //     let result_read = rust_read(file.as_raw_fd(), s1, 1, read);
    //     assert_eq!(result_ft_read, result_read);
    //     Ok(())
    // }

    // #[test]
    // fn test_read_bigger_nb_char() -> Result<(), Box<dyn std::error::Error>> {
    //     let file = fs::File::create("ft_read_bigger_nb_char.txt")?;
    //     let file_2 = fs::File::create("read_bigger_nb_char.txt")?;
    //     let s1 = "read_bigger_nb_char";
    //     let result_ft_read = rust_read(file.as_raw_fd(), s1, 100, ft_read);
    //     let result_read = rust_read(file_2.as_raw_fd(), s1, 100, read);
    //     assert_eq!(result_ft_read, result_read);
    //     Ok(())
    // }
}

