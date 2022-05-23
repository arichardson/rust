// Checks that we can use CHERI inline assembly
//
// needs-llvm-components: riscv
// compile-flags: --target=riscv64imac-unknown-none-cheri-hybrid-elf

#![no_core]
#![feature(no_core, lang_items, rustc_attrs)]
#![crate_type = "rlib"]

#[rustc_builtin_macro]
macro_rules! asm {
    () => {};
}

// #[lang = "sized"]
// trait Sized {}
// #[lang = "copy"]
// trait Copy {}

pub unsafe fn cincoffset() {
    // CHECK: call void asm sideeffect alignstack inteldialect "cincoffset cnull, cnull, 123"
    asm!(
        r"cincoffset cnull, cnull, 123",
    )
}
