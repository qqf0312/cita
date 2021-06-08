//ffi_wrapper.rs
extern crate libc;
use libc::{c_ulong, c_uchar, c_int, c_uint};

//should be write in ffi.rs

#[link(name = "myjerasure", kind = "static")]
extern {
//    fn add(i1: c_int, i2: c_int) -> c_int;
//    fn replace(s: &mut [u8;3], d: &mut [u8;3]);
//    fn add_v2(i1: c_int, i2:c_int, out: &mut c_int);

   fn encoder_clothes(k: c_ulong, m: c_ulong, w: c_ulong, raw_datas: *mut  c_uchar, raw_coding: *mut  c_uchar, blocksize: c_uint) -> c_int;
   fn decoder_clothes(k: c_ulong, m: c_ulong, w: c_ulong, raw_datas: *mut  c_uchar, raw_coding: *mut  c_uchar, blocksize: c_uint, erasures: *mut c_int) -> c_int;
}

//should be write in wrapper.rs
pub fn encoder_clothes_w(k: u64, m: u64, w: u64, raw_datas:* mut u8, raw_coding:* mut u8, blocksize: usize)-> i32 {
    unsafe {
        encoder_clothes(k as c_ulong, m as c_ulong, w as c_ulong, raw_datas, raw_coding, blocksize as c_uint)
    }
}

pub fn decoder_clothes_w(k: u64, m: u64, w: u64, raw_datas:* mut u8, raw_coding:* mut u8, blocksize: usize, erasures: *mut i32)-> i32 {
    unsafe {
        decoder_clothes(k as c_ulong, m as c_ulong, w as c_ulong, raw_datas, raw_coding, blocksize as c_uint, erasures)
    }
}

// pub fn add_w(i1: i32, i2: i32) -> i32 {
//     unsafe {add(i1 as c_int, i2 as c_int)}
// }

// pub fn replace_w(s: &mut [u8;3], d: &mut [u8;3]) {
//     unsafe {replace(s, d)}
// }

// pub fn add_v2_w(i1: i32, i2: i32, out: &mut i32) {
//     unsafe {add_v2(i1 as c_int, i2 as c_int, out as &mut c_int)}
// }