// build.rs

extern crate cc;

fn main() {
    cc::Build::new()
        .file("termiWin/termiWin.c")
        .include("termiWin/termiWin.h")
        .define("_CRT_SECURE_NO_WARNINGS", None)
        .compile("termiWin");
}