mod patch;

use crate::patch::Patch;

use std::thread;
use std::ffi;

use winapi::um::winnt::{DLL_PROCESS_ATTACH};
use winapi::shared::minwindef::*;
use winapi::um::winuser::MessageBoxA;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::consoleapi::AllocConsole;

static MSG: &str = "The DLL has been injected!\0";
static CAPTION: &str = "Alert\0";

pub fn main() {
    unsafe {
        // Enable console
        AllocConsole();
        // Spawn a message box
        MessageBoxA(0 as HWND, MSG.as_ptr() as *const i8, CAPTION.as_ptr() as *const i8, 0);
    }

    let offset: u64 = 0x1067;
    let instr_addr = unsafe {
        let addr = GetModuleHandleA(0 as *const i8) as u64 + offset;
        addr as *mut ffi::c_uchar
    };

    let patch = Patch::new(instr_addr, &[0x83, 0x44, 0x24, 0x2C, 0x32]);
    patch.set_state(true);
}

#[no_mangle]
pub extern "system" fn DllMain(_h_inst: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> u32 {
    if fdw_reason == DLL_PROCESS_ATTACH {
        // Run main program
        thread::spawn(move || main());
    }
    1
}