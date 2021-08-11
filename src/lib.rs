// unsafe functions and traits 

#![feature(abi_thiscall, asm)]
#![allow(dead_code, non_snake_case)]

pub mod classes;
pub mod traits;
use winapi as __getfn_winapi__;

#[macro_export]
macro_rules! all_to_cstring {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(CString::new($x).unwrap());
            )*
            temp_vec
        }
    };
}
