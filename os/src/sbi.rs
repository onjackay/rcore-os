#![allow(unused)]
use core::arch::asm;
// use rustsbi;

// legacy extensions: ignore fid
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;

// system reset extension
const SRST_EXTENSION: usize = 0x53525354;
const SBI_SHUTDOWN: usize = 0;

// base extension
const EXTENSION_BASE: usize = 0x10;
const FUNCTION_BASE_GET_SPEC_VERSION: usize = 0x00;

// TODO: remove it
#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x16") fid,
            in("x17") eid,
        );
    }
    ret
}

#[inline(always)]
fn sbi_call_(extension: usize, function: usize, arg0: usize, arg1: usize) -> usize {
    let value: usize;
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe { asm!(
            "ecall",
            in("a0") arg0, in("a1") arg1,
            in("a6") function, in("a7") extension,
            lateout("a0") _, lateout("a1") value,
        ) },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0, arg1));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    value
}

#[inline]
pub fn get_spec_version() -> usize {
    sbi_call_(EXTENSION_BASE, FUNCTION_BASE_GET_SPEC_VERSION, 0, 0)
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, c, 0, 0);
}

pub fn shutdown() -> ! {
    sbi_call(SRST_EXTENSION, SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!")
}