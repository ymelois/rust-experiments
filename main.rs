#![no_std]
#![no_main]

#[cfg(not(all(target_arch = "x86_64", any(target_os = "linux", target_os = "none"))))]
compile_error!("This program uses Linux kernel syscalls.");

const STDOUT: i64 = 1;

type Result<T, E = ()> = core::result::Result<T, E>;

fn write_all(
    fd: i64,
    mut buf: &[u8],
) -> Result<()> {
    while !buf.is_empty() {
        match syscall::write(fd, buf) {
            Ok(0) => return Err(()),
            Ok(n) => buf = &buf[n..],
            Err(e) if e == syscall::EINTR => {}
            Err(_) => return Err(()),
        }
    }
    Ok(())
}

fn print(buf: &[u8]) -> Result<()> {
    write_all(STDOUT, buf)?;
    Ok(())
}

fn main(args: &[*const u8]) -> Result<()> {
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

#[unsafe(no_mangle)]
extern "C" fn strlen(ptr: *const u8) -> usize {
    unsafe {
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        len
    }
}

#[unsafe(export_name = "main")]
extern "C" fn _main(
    argc: usize,
    argv: *const *const u8,
) -> ! {
    let args = unsafe { core::slice::from_raw_parts(argv, argc) };

    match main(args) {
        Ok(()) => syscall::exit(0),
        Err(()) => syscall::exit(1),
    }
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
#[rustfmt::skip]
extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "mov rdi, [rsp]",
        "lea rsi, [rsp + 8]",
        "jmp main",
    );
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    syscall::exit(1);
}
