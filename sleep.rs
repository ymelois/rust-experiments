#![no_std]
#![no_main]

#[cfg(not(all(target_arch = "x86_64", any(target_os = "linux", target_os = "none"))))]
compile_error!("This program uses Linux kernel syscalls.");

use common::*;

enum ParseIntError {
    InvalidChar(u8),
    Overflow,
}

fn parse_u64(input: &[u8]) -> Result<u64, ParseIntError> {
    let len = input.len();

    if len > 20 {
        return Err(ParseIntError::Overflow);
    }

    let mut sum = 0_u64;

    for i in 0..len {
        let value = *unsafe { input.get_unchecked(len - i - 1) };

        if !value.is_ascii_digit() {
            return Err(ParseIntError::InvalidChar(value));
        }

        let value = value - b'0';

        let Some(value) = (value as u64).checked_mul(10_u64.pow(i as u32)) else {
            return Err(ParseIntError::Overflow);
        };

        let Some(new_sum) = sum.checked_add(value) else {
            return Err(ParseIntError::Overflow);
        };

        sum = new_sum;
    }

    Ok(sum)
}

#[unsafe(no_mangle)]
extern "C" fn main(
    argc: usize,
    argv: *const *const u8,
) -> ! {
    if argc != 2 {
        _ = eprint(b"Usage: sleep <seconds>");
        exit(1);
    }

    let input = unsafe {
        let ptr = *argv.add(1);
        let len = strlen(ptr);
        core::slice::from_raw_parts(ptr, len)
    };

    let seconds = match parse_u64(input) {
        Ok(seconds) => seconds,
        Err(ParseIntError::InvalidChar(value)) => {
            _ = eprint(b"Invalid character: ");
            _ = eprint(&[value]);
            _ = eprint(b".");
            exit(1);
        }
        Err(ParseIntError::Overflow) => {
            _ = eprint(b"Value is too big.");
            exit(1);
        }
    };

    match sleep(seconds) {
        Ok(()) => exit(0),
        Err(_) => exit(1),
    }
}
