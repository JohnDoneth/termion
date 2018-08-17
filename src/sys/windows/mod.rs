extern crate libc;

use std::io;

pub mod attr;
pub mod size;
pub mod tty;

use self::libc::{c_uint, c_uchar};

type speed_t = c_uint;
type cc_t = c_uchar;
type tcflag_t = c_uint;

#[derive(Clone, Copy)]
pub struct Termios {
    c_iflag: tcflag_t,
    c_oflag: tcflag_t,
    c_cflag: tcflag_t,
    c_lflag: tcflag_t,
    c_line: cc_t,
    c_cc: [cc_t; 32],
    c_ispeed: speed_t,
    c_ospeed: speed_t,
}

// Support functions for converting libc return values to io errors {
trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }

fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}
// } End of support functions
