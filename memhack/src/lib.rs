use std::thread;
use std::ffi;

use winapi::ctypes::c_void;
use winapi::shared::ntdef::*;
use winapi::um::memoryapi::{VirtualProtect, WriteProcessMemory};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, PAGE_EXECUTE_READWRITE};
use winapi::shared::minwindef::*;
use winapi::um::winuser::MessageBoxA;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::consoleapi::AllocConsole;
use winapi::um::errhandlingapi::GetLastError;

struct Patch<'a> {
    proc_handle: HANDLE,
    addr: *mut ffi::c_uchar,
    original_bytes: &'a [u8],
    patched_bytes: &'a [u8]
}

impl<'a> Patch<'a> {
    pub fn new(proc_handle: HANDLE, addr: *mut ffi::c_uchar, original_bytes: &'a [u8], patched_bytes: &'a [u8]) -> Self {
        // Ensure patch is the same size as the original bytes
        assert!(original_bytes.len() == patched_bytes.len());

        Patch {
            proc_handle,
            addr,
            original_bytes,
            patched_bytes
        }
    }

    pub fn enable(&self) {
        unsafe {
            let mut old_protection: u32 = 0;

            // Change protection
            VirtualProtect(self.addr as *mut winapi::ctypes::c_void, self.patched_bytes.len(), PAGE_EXECUTE_READWRITE, &mut old_protection as *mut u32);

            // Write patch bytes
            for (i, b) in self.patched_bytes.iter().enumerate() {
                *self.addr.offset(i as isize) = *b;
            }

            // Restore protection
            VirtualProtect(self.addr as *mut winapi::ctypes::c_void, self.patched_bytes.len(), old_protection, &mut old_protection as *mut u32);
        }
    }
}

static msg: &str = "The DLL has been injected!\0";
static caption: &str = "Alert\0";

pub fn main() {
    unsafe {
        // Enable console
        AllocConsole();
        // Spawn a message box
        MessageBoxA(0 as HWND, msg.as_ptr() as *const i8, caption.as_ptr() as *const i8, 0);
    }

    let offset: u64 = 0x1067;
    let instr_addr = unsafe {
        let addr = GetModuleHandleA(0 as *const i8) as u64 + offset;
        addr as *mut ffi::c_uchar
    };

    let patch = unsafe { Patch::new(GetCurrentProcess(), instr_addr, &[0x83, 0x44, 0x24, 0x2C, 0x01], &[0x83, 0x44, 0x24, 0x2C, 0x32]) };
    patch.enable();
}

#[no_mangle]
pub extern "system" fn DllMain(h_inst: HINSTANCE, fdw_reason: DWORD, lpv_reserved: LPVOID) -> u32 {
    if fdw_reason == DLL_PROCESS_ATTACH {
        // Run main program
        thread::spawn(move || main());
    }
    1
}