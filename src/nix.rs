extern crate libc;
extern crate atty;

use self::super::Size;
use self::libc::{c_ulong, c_ushort, STDOUT_FILENO};
use self::libc::ioctl;

#[cfg(not(target_os = "macos"))]
const TIOCGWINSZ: c_ulong = 0x00005413;

#[cfg(target_os = "macos")]
const TIOCGWINSZ: c_ulong = 0x40087468;

/// A representation of the size of the current terminal
#[repr(C)]
#[derive(Debug)]
pub struct UnixSize {
  /// number of rows
  pub rows: c_ushort,
  /// number of columns
  pub cols: c_ushort,
  x: c_ushort,
  y: c_ushort
}

/// Gets the current terminal size
pub fn get() -> Option<Size> {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    if atty::isnt() {
        return None;
    }
    let us = UnixSize { rows: 0, cols: 0, x: 0, y: 0 };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &us) };
    if r == 0 { Some(Size { rows: us.rows, cols: us.cols }) } else { None }
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::process::Stdio;
    use super::super::Size;
    use super::get;

    #[test]
    #[cfg(target_os = "macos")]
    fn test_shell() {
        let output = Command::new("stty")
            .arg("-f").arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()
            .unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8(
             output.stdout
        ).unwrap();
        let mut data = stdout.split_whitespace();
        let rs = data.next().unwrap().parse::<u16>().unwrap();
        let cs = data.next().unwrap().parse::<u16>().unwrap();
        if let Some(Size { rows, cols }) = get() {
            assert_eq!(rows, rs);
            assert_eq!(cols, cs);
        }
    }
}