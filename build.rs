fn main() {
    println!("cargo:rerun-if-changed=src/syscalls.c");
    cc::Build::new()
        .file("src/syscalls.c")
        .flag("-fno-PIC") // for some reason `pic(false)` doesn't work
        .static_flag(true)
        .compile("syscalls");
}
