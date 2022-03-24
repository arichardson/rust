// Checks that we don't crash when generating code for a usize->ptr cast
// when targetting a CHERI target
//
// needs-llvm-components: riscv
// compile-flags: -C no-prepopulate-passes --target=riscv64imac-unknown-none-purecap-elf

#![crate_type = "lib"]
#![no_core]
#![feature(no_core, lang_items, auto_traits)]

#[lang = "sized"]
trait Sized {}

#[lang = "copy"]
trait Copy {}

// CHECK: define i8 addrspace(200)* @inttoptr
#[no_mangle]
pub fn inttoptr_1() -> *mut char {
    let x = 0;
    let y = x as *mut char;
    y
}

#[no_mangle]
pub fn inttoptr_2() -> *mut char {
    let x = 1 as usize;
    let y = x as *mut char;
    y
}

#[no_mangle]
pub fn inttoptr_usize_arg(x: usize) -> *mut char {
    x as *mut char
}

#[no_mangle]
pub fn inttoptr_isize_arg(x: isize) -> *mut char {
    x as *mut char
}

#[no_mangle]
pub fn inttoptr_i16_arg(x: i16) -> *mut char {
    x as *mut char
}

#[no_mangle]
pub fn inttoptr_iptr_arg(x: iptr) -> *mut char {
    x as *mut char
}

#[no_mangle]
pub fn inttoptr_uptr_arg(x: uptr) -> *mut char {
    x as *mut char
}


#[lang = "sync"]
auto trait Sync {}

// 2 digit decimal look up table
static DEC_DIGITS_LUT: &[u8; 200] = b"0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

pub fn lookup() ->  u8 {
    let lut_ptr = DEC_DIGITS_LUT.as_ptr();
    lut_ptr[2]
}
