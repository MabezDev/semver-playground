#![no_std]

#[unsafe(no_mangle)]
#[used]
pub static mut __EXTERNAL_INTERRUTPTS: [usize; 1] = [0xFFusize];
