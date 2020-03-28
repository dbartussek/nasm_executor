use nasm_rs::*;

pub fn main() {
    Build::new()
        .file("src/assembly.asm")
        .compile("assembly");
}
