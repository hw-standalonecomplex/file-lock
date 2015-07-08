use std::os::unix::io::RawFd;

use errno;

pub use util::{Kind, Mode, ParseError};
pub use functions::Error;
use functions;


/// Represents a write lock on a file.
///
/// The `lock(Kind)` method tries to obtain a write-lock on the
/// file identified by a file-descriptor. 
/// One can obtain different kinds of write-locks.
///
/// * Kind::NonBlocking - immediately return with an `Errno` error.
/// * Kind::Blocking - waits (i.e. blocks the running thread) for the current
/// owner of the lock to relinquish the lock.
///
/// # Example
///
/// Please note that the examples use `tempfile` merely to quickly create a file
/// which is removed automatically. In the common case, you would want to lock
/// a file which is known to multiple processes.
///
/// ```
/// extern crate file_lock;
/// extern crate tempfile;
///
/// use file_lock::fd::{Lock, Error, Mode, Kind};
/// use std::os::unix::io::AsRawFd;
///
/// fn main() {
///     let f = tempfile::TempFile::new().unwrap();
///
///     match Lock::new(f.as_raw_fd()).lock(Kind::NonBlocking, Mode::Write) {
///         Ok(_)  => {
///             // we have a lock, which is discarded automatically. Otherwise you could call
///             // `unlock()` to make it explicit
///             // 
///             println!("Got lock");
///         },
///         Err(Error::WouldBlock)
///               => println!("Lock already taken by other process"),
///         Err(Error::Errno(i))
///               => println!("Got filesystem error: {}", i),
///     }
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Lock {
    fd: RawFd,
}

impl Lock {
    /// Create a new lock instance from the given file descriptor `fd`.
    /// 
    /// You will have to call `lock(...)` on it to acquire any lock.
    pub fn new(fd: RawFd) -> Lock {
        Lock {
            fd:   fd,
        }
    }

    /// Obtain a write-lock the file-descriptor
    /// 
    /// For an example, please see the documentation of the [`Lock`](struct.Lock.html) structure.
    pub fn lock(&self, kind: Kind, mode: Mode) -> Result<(), Error> {
        functions::lock(self.fd, kind.clone(), mode.clone())
    }

    /// Unlocks the file held by `Lock`.
    ///
    /// In reality, you shouldn't need to call `unlock()`. As `Lock` implements
    /// the `Drop` trait, once the `Lock` reference goes out of scope, `unlock()`
    /// will be called automatically.
    ///
    /// For an example, please see the documentation of the [`Lock`](struct.Lock.html) structure.
    pub fn unlock(&self) -> Result<(), errno::Errno> {
        functions::unlock(self.fd)
    }
}

#[allow(unused_must_use)]
impl Drop for Lock {
    fn drop(&mut self) {
        self.unlock().ok();
    }
}
