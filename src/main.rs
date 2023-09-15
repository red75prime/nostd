#![no_std]
#![no_main]
#![windows_subsystem = "console"]
#![feature(lang_items, panic_info_message)]
#[link(name = "vcruntime")]
extern {}
use core::panic::PanicInfo;
use core::ptr::null_mut;

use windows_sys::Win32::{
    Foundation::{BOOL, HANDLE},
    Storage::FileSystem::WriteFile,
    System::Console::{GetStdHandle, STD_OUTPUT_HANDLE},
};

pub fn write_stdout(bytes: &[u8]) -> BOOL {
    unsafe {
        let stdout: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);

        let mut written: u32 = 0;

        let ok = WriteFile(
            stdout,
            bytes.as_ptr(),
            bytes.len() as u32,
            &mut written,
            null_mut(),
        );

        ok
    }
}

struct MemBuf<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl core::fmt::Write for MemBuf<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let rest = &mut self.buf[self.pos..];
        if !rest.is_empty() {
            let len = core::cmp::min(rest.len(), s.len());
            rest[..len].copy_from_slice(&s.as_bytes()[..len]);
            self.pos += len;
        }
        Ok(())
    }
}


#[cfg(windows)]
#[no_mangle]
pub extern "C" fn mainCRTStartup() {
    panic!("Whatever it is {}", 2);
    write_stdout(b"Ok");
    unsafe {
        windows_sys::Win32::System::Threading::ExitProcess(0);
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[lang = "panic_impl"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(info: &PanicInfo) -> ! {
    if let Some(msg) = info.message() {
        let mut buf = [0; 2048];
        let mut wrt = MemBuf { buf: &mut buf[..], pos: 0 };
        let _ = core::fmt::write(&mut wrt, *msg);
        let len = wrt.pos;
        write_stdout(&buf[..len]);
    } else {
        write_stdout(b"Panic with no message");
    }
    unsafe {
        windows_sys::Win32::System::Threading::ExitProcess(128);
    }
}
