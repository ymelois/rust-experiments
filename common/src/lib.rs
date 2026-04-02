#![no_std]

pub use syscall::exit;

const STDOUT: i64 = 1;
const STDERR: i64 = 2;

pub enum Error {
    EndOfFile,
    Syscall(syscall::Error),
}

pub fn write_all(
    fd: i64,
    mut buf: &[u8],
) -> Result<(), Error> {
    while !buf.is_empty() {
        match syscall::write(fd, buf) {
            Ok(0) => return Err(Error::EndOfFile),
            Ok(n) => buf = unsafe { buf.get_unchecked(n..) },
            Err(e) if e == syscall::EINTR => {}
            Err(e) => return Err(Error::Syscall(e)),
        }
    }
    Ok(())
}

pub fn print(buf: &[u8]) -> Result<(), Error> {
    write_all(STDOUT, buf)?;
    Ok(())
}

pub fn eprint(buf: &[u8]) -> Result<(), Error> {
    write_all(STDERR, buf)?;
    Ok(())
}

pub fn sleep(seconds: u64) -> Result<(), Error> {
    let ts = syscall::Timespec {
        tv_sec: seconds,
        tv_nsec: 0,
    };

    syscall::nanosleep(ts).map_err(Error::Syscall)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(ptr: *const u8) -> usize {
    unsafe {
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        len
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
