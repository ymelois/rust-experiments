#![no_std]
#![no_main]

const SYS_EXIT: i64 = 60;

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
extern "C" fn main(
    argc: usize,
    argv: *const *const u8,
) -> ! {
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
