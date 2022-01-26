const FD_STDOUT: usize = 1;
use core::arch::asm;

use log::warn;

use crate::batch::USER_STACK;

fn check_memory_safety(buf: *const u8, len: usize) -> bool {
    let buf_start = buf as usize;
    let buf_end = buf_start + len;
    let stack_top = USER_STACK.get_sp();
    let stack_bottom = USER_STACK.data.as_ptr() as usize;

    extern "C" {
        fn ekernel();
    }

    if buf_start >= ekernel as usize {
        return true;
    }
    if buf_start >= stack_bottom && buf_end <= stack_top {
        return true;
    }

    false
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            if !check_memory_safety(buf, len) {
                // panic!("memory not safe");
                return -1;
            }
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            println!("[kernel] Unsupported fd in sys_write!");
            -1
            // panic!("Unsupported fd in sys_write");
        }
    }
}
