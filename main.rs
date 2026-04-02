#![no_std]
#![no_main]

#[cfg(not(all(target_arch = "x86_64", any(target_os = "linux", target_os = "none"))))]
compile_error!("This program uses Linux kernel syscalls.");

use common::*;

fn main(args: &[*const u8]) -> Result<(), Error> {
    for arg in args {
        let arg = unsafe {
            let ptr = *arg;
            let len = strlen(ptr);
            core::slice::from_raw_parts(ptr, len)
        };
        print(arg)?;
        print(b"\n")?;
    }

    Ok(())
}

#[unsafe(export_name = "main")]
extern "C" fn _main(
    argc: usize,
    argv: *const *const u8,
) -> ! {
    let args = unsafe { core::slice::from_raw_parts(argv, argc) };

    match main(args) {
        Ok(()) => exit(0),
        Err(_) => exit(1),
    }
}
