#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;

use linux_kernel_module::println;


struct nRFBLE {
    message: String,
}

impl linux_kernel_module::KernelModule for nRFBLE {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello kernel module!");
        Ok(nRFBLE {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for nRFBLE {
    fn drop(&mut self) {
        //
    }
}

linux_kernel_module::kernel_module!(
   nRFBLE,
    author: b"Alexandra Clifford",
    description: b"A kernel module for nRF* BLE device",
    license: b"GPL"
);
