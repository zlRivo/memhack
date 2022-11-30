use std::ffi;

use winapi::um::memoryapi::{VirtualProtect};
use winapi::um::winnt::{PAGE_EXECUTE_READWRITE};

pub struct Patch {
    addr: *mut ffi::c_uchar,
    original_bytes: Vec<u8>,
    patched_bytes: Vec<u8>
}

impl Patch {
    pub fn new(addr: *mut ffi::c_uchar, patched_bytes: &[u8]) -> Self {

        // Read original bytes
        let original_bytes: Vec<u8> = unsafe {
            (0..patched_bytes.len())
                .map(|i| *addr.offset(i as isize))
                .collect::<Vec<u8>>()
        };
        println!("{:?}", original_bytes);

        // Ensure patch is the same size as the original bytes
        assert!(original_bytes.len() == patched_bytes.len());

        Patch {
            addr,
            original_bytes,
            patched_bytes: patched_bytes.to_vec()
        }
    }

    #[allow(dead_code)]
    /// Sets the state of the patch and returns true if the operation succeeded
    pub fn set_state(&self, state: bool) -> bool {
        let mut old_protection: u32 = 0;

        // Change protection
        let ret = unsafe {
            VirtualProtect(self.addr as *mut winapi::ctypes::c_void, self.patched_bytes.len(), PAGE_EXECUTE_READWRITE, &mut old_protection as *mut u32)
        };

        if ret != 1 {
            return false;
        }

        // Check if patched enabled or not
        let bytes = if state {
            &self.patched_bytes
        } else {
            &self.original_bytes
        };

        // Write bytes
        unsafe {
            for (i, b) in bytes.iter().enumerate() {
                *self.addr.offset(i as isize) = *b;
            }
        }

        // Restore protection
        let ret = unsafe {
            VirtualProtect(self.addr as *mut winapi::ctypes::c_void, self.patched_bytes.len(), old_protection, &mut old_protection as *mut u32)
        };

        if ret != 1 {
            return false;
        }
    
        true
    }
    
    #[allow(dead_code)]
    /// Disables the patch and returns true if the operation succeeded
    pub fn enable(&self) -> bool {
        self.set_state(true)
    }

    #[allow(dead_code)]
    /// Enables the patch and returns true if the operation succeeded
    pub fn disable(&self) -> bool {
        self.set_state(false)
    }

}