extern "C" {
    fn ft_strlen(s: *const u8) -> usize;
}

fn main() {
    let test_str = b"Hello, world!\0";
    unsafe {
        let length = ft_strlen(test_str.as_ptr());
        println!("Length of the string: {}", length);
    }
}
