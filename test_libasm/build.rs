fn main() {
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-search=native=../");
    println!("cargo:rustc-link-lib=static=asm");
    println!("cargo:rustc-link-arg=-no-pie");
}
