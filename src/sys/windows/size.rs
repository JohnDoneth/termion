use std::{io, mem};

use super::cvt;
use super::libc::{c_ushort};

extern crate winapi;

#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    _x: c_ushort,
    _y: c_ushort,
}
/// Get the size of the terminal.
/// Code from https://github.com/eminence/terminal-size
pub fn terminal_size() -> io::Result<(u16, u16)> {
    unsafe {

        use self::winapi::um::winnt::HANDLE;
        use self::winapi::um::processenv::{GetStdHandle};
        use self::winapi::um::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
        use self::winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT};

        let hand: HANDLE = unsafe {
            GetStdHandle(STD_OUTPUT_HANDLE)
        };

        let zc = COORD{X: 0, Y: 0};
        let mut csbi = CONSOLE_SCREEN_BUFFER_INFO{
            dwSize: zc.clone(),
            dwCursorPosition: zc.clone(),
            wAttributes: 0,
            srWindow: SMALL_RECT{Left:0, Top: 0, Right: 0, Bottom: 0},
            dwMaximumWindowSize: zc

        };
        let success:bool = unsafe {
            GetConsoleScreenBufferInfo(hand, &mut csbi) != 0
        };
        if success {
            let w: u16 = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16;
            let h: u16 = (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16;
            Ok((w, h))
        } else {
            Err(io::Error::last_os_error())
        }
    }
}
