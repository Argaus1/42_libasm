use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

extern "C" {
    fn ft_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
}

fn rust_strcpy(dest: &mut [u8], src: &str) -> String {
    // Ensure the source is null-terminated
    let src_c = std::ffi::CString::new(src).expect("Failed to create CString");
    let dest_ptr = dest.as_mut_ptr() as *mut c_char;

    unsafe {
        ft_strcpy(dest_ptr, src_c.as_ptr());
        let result = CStr::from_ptr(dest_ptr).to_string_lossy().to_string();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strcpy_basic() {
        let mut dest = [0u8; 100];
        let src = "Hello, world!";
        let result = rust_strcpy(&mut dest, src);

        assert_eq!(result, src);
    }

    #[test]
    fn test_strcpy_empty() {
        let mut dest = [0u8; 100];
        let src = "";
        let result = rust_strcpy(&mut dest, src);

        assert_eq!(result, src);
    }

    #[test]
    fn test_strcpy_with_special_chars() {
        let mut dest = [0u8; 100];
        let src = "Hello\nWorld\t!";
        let result = rust_strcpy(&mut dest, src);

        assert_eq!(result, src);
    }
}

