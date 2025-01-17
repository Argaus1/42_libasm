use std::os::raw::c_int; // 32 bits
use std::os::raw::c_ulonglong; // for size_t
use std::str::from_utf8_unchecked; // convert bytes in string slice

extern "C" {

   fn ft_strlen(s: *const u8) -> c_ulonglong;
   fn strlen(s: *const u8) -> c_ulonglong;

    fn ft_strcmp(s1: *const u8, s2: *const u8) -> c_int;
    fn strcmp(s1: *const u8, s2: *const u8) -> c_int;
}

fn main() {

    {
        // STRLEN
        println!("STRLEN\n");

        let test_str = b"Hello, world!\0";

        unsafe {
            println!("Test n°1: \"{}\"", from_utf8_unchecked(test_str));

            let length: c_ulonglong = strlen(test_str.as_ptr());
            println!("With strlen: Length of the string: {}", length);
            let length: c_ulonglong = ft_strlen(test_str.as_ptr());
            println!("With ft_strlen.s: Length of the string: {}", length);
        }
    }

    {
        // STRCMP
        println!("\nSTRCMP\n");

        unsafe {

            let s1 = b"zouzou\0";
            let s2 = b"aouzou\0";

            println!("Test n°1: \"{}\" and \"{}\"", from_utf8_unchecked(s1), from_utf8_unchecked(s2));

            let res: c_int = strcmp(s1.as_ptr(), s2.as_ptr());
            println!("With strcmp: Res: {}", res);
            let res: c_int = ft_strcmp(s1.as_ptr(), s2.as_ptr());
            println!("With ft_strcmp: Res: {}", res);
        }

        unsafe {

            let s1 = b"\0";
            let s2 = b"\0";

            println!("Test n°2: \"{}\" and \"{}\"", from_utf8_unchecked(s1), from_utf8_unchecked(s2));

            let res: c_int = strcmp(s1.as_ptr(), s2.as_ptr());
            println!("With strcmp: Res: {}", res);
            let res: c_int = ft_strcmp(s1.as_ptr(), s2.as_ptr());
            println!("With ft_strcmp: Res: {}", res);
        }
    }

}
