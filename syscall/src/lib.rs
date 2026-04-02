#![no_std]

pub const SYS_WRITE: i64 = 1;
pub const SYS_EXIT: i64 = 60;

pub const EINTR: Error = Error(4);

#[derive(PartialEq, Eq)]
pub struct Error(usize);

pub fn write(
    fd: i64,
    buf: &[u8],
) -> Result<usize, Error> {
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
        Err(Error(-ret as usize))
    } else {
        Ok(ret as usize)
    }
}

pub fn exit(code: u64) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") code,
            options(noreturn),
        )
    }
}
