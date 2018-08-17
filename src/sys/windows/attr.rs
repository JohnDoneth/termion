use std::{io, mem};

use super::{cvt, Termios};
use super::libc::c_int;

pub fn get_terminal_attr() -> io::Result<Termios> {
    extern "C" {
        pub fn tcgetattr(fd: c_int, termptr: *mut Termios) -> c_int;
    }
    println!("tcgetattr");
    unsafe {
        let mut termios = mem::zeroed();
        cvt(tcgetattr(0, &mut termios))?;
        Ok(termios)
    }
}

pub fn set_terminal_attr(termios: &Termios) -> io::Result<()> {
    extern "C" {
        pub fn tcsetattr(fd: c_int, opt: c_int, termptr: *const Termios) -> c_int;
    }
    println!("tcsetattr");
    //cvt(unsafe { tcsetattr(0, 0, termios) }).and(Ok(()))
    unsafe { tcsetattr(0, 0, termios); }
    Ok(())
}

pub fn raw_terminal_attr(termios: &mut Termios) {
    extern "C" {
        pub fn cfmakeraw(termptr: *mut Termios);
    }
    println!("cfmakeraw");
    unsafe { cfmakeraw(termios) }
}
