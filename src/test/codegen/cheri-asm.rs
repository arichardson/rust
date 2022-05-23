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

#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}

pub unsafe fn cincoffset() {
    // CHECK: tail call void asm sideeffect alignstack "cincoffset cnull, cnull, 123", "~{vtype},~{vl},~{vxsat},~{vxrm},~{memory}"()
    asm!(
        r"cincoffset cnull, cnull, 123",
    )
}
