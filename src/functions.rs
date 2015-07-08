//! Contains the actual functional lock implementation
use std::error::Error as ErrorTrait;
use std::os::unix::io::RawFd;
use std::fmt;
use errno;
use libc::{self, c_int};

pub use util::{Kind, Mode, ParseError};

const WOULD_BLOCK_MSG: &'static str = "Lock is already taken by another process";

extern {
    fn c_lock(fd: i32, should_block: i32, is_write_lock: i32) -> c_int;
    fn c_unlock(fd: i32) -> c_int;
}

/// Represents the error that occurred while trying to lock or unlock a file.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Indicates that attempting to acquire the lock in Blocking mode would block.
    /// This variant is used only if non-blocking lock acquisition failed.
    WouldBlock,
    /// caused when the error occurred at the filesystem layer (see
    /// [errno](https://crates.io/crates/errno)).
    Errno(errno::Errno),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::WouldBlock
                => WOULD_BLOCK_MSG.fmt(f),
            Error::Errno(ref errno)
                => write!(f, "Lock operation failed: {}", errno)
        }
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            Error::WouldBlock
                => WOULD_BLOCK_MSG,
            Error::Errno(_) 
                => "Failed to acuire file lock",
        }
    }
}

/// Obtain a write-lock the file-descriptor
/// 
/// For an example, please see the documentation of the [`Lock`](struct.Lock.html) structure.
pub fn lock(fd: RawFd, kind: Kind, mode: Mode) -> Result<(), Error> {
    let errno = unsafe { c_lock(fd, kind.into(), mode.into()) };

    return match errno {
       0 => Ok(()),
       libc::consts::os::posix88::EAGAIN => Err(Error::WouldBlock),
       _ => Err(Error::Errno(errno::Errno(errno))),
    }
}

/// Unlocks the file held by `Lock`.
///
/// In reality, you shouldn't need to call `unlock()`. As `Lock` implements
/// the `Drop` trait, once the `Lock` reference goes out of scope, `unlock()`
/// will be called automatically.
///
/// For an example, please see the documentation of the [`Lock`](struct.Lock.html) structure.
pub fn unlock(fd: RawFd) -> Result<(), errno::Errno> {
  unsafe {
    let errno = c_unlock(fd);

    return match errno {
       0 => Ok(()),
       _ => Err(errno::Errno(errno)),
    }
  }
}


