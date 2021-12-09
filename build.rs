fn main() {
    println!("cargo:rerun-if-changed=src/syscalls.S");
    nasm_rs::compile_library_args("libsyscalls.a", &["src/syscalls.S"], &["-f elf32"])
        .expect("failed to compile assembly");
}
