use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MESSAGEBOX_RESULT};

pub fn win_messagebox(lptext: impl ToString, lpcaption : impl ToString, icon : u32) -> MESSAGEBOX_RESULT {
    unsafe {
        let lptext: Vec<u16> = OsStr::new(&lptext.to_string()).encode_wide().chain(Some(0)).collect();
        let lpcaption: Vec<u16> = OsStr::new(&lpcaption.to_string()).encode_wide().chain(Some(0)).collect();
        MessageBoxW(
            0,
            lptext.as_ptr(),
            lpcaption.as_ptr(),
            icon,
        )
    }    
}