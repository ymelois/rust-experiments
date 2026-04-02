#![no_std]
#![no_main]

const SYS_WRITE: i64 = 1;
const SYS_EXIT: i64 = 60;

const STDOUT: i64 = 1;

struct SyscallError(usize);

fn write(
    fd: i64,
    buf: &[u8],
) -> Result<usize, SyscallError> {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_WRITE,
            in("rdi") fd,
            in("rsi") buf.as_ptr() as u64,
            in("rdx") buf.len() as u64,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack),
        );
    };
    if ret < 0 {
        Err(SyscallError(-ret as usize))
    } else {
        Ok(ret as usize)
    }
}

fn write_all(
    fd: i64,
    mut buf: &[u8],
) -> Result<(), SyscallError> {
    while !buf.is_empty() {
        match write(fd, buf) {
            Ok(0) => todo!("handle error"),
            Ok(n) => buf = &buf[n..],
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn print(buf: &[u8]) -> Result<(), SyscallError> {
    write_all(STDOUT, buf)?;
    Ok(())
}

fn exit(code: u64) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") code,
            options(noreturn),
        )
    }
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

#[unsafe(no_mangle)]
extern "C" fn main(
    argc: usize,
    argv: *const *const u8,
) -> ! {
    for i in 0..argc {
        let arg = unsafe {
            let ptr = *argv.add(i);
            let len = strlen(ptr);
            core::slice::from_raw_parts(ptr, len)
        };
        print(arg);
        print(b"\n");
    }

    exit(0);
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
    exit(1)
}
