use std::io;

use crate::{errors::EmulatorError, os::linux::syscall::*};

pub fn open(path: &str, flags: u64, mode: u64) -> Result<i64, EmulatorError> {
    let open_mode = mode & 0x7fffffff;
    let fd = syscall_3(
        LinuxSysCalls::Open as u64,
        path.as_ptr() as u64,
        flags,
        open_mode,
    );
    if fd == -1 {
        Err(EmulatorError::IOError(io::Error::last_os_error()))
    } else {
        Ok(fd)
    }
}

pub fn read(fd: u64, buf: u64, size: u64) -> Result<i64, EmulatorError> {
    let size = syscall_3(LinuxSysCalls::Read as u64, fd, buf, size);
    if size == -1 {
        Err(EmulatorError::IOError(io::Error::last_os_error()))
    } else {
        Ok(size)
    }
}

pub fn write(fd: u64, data: u64, len: u64) -> Result<i64, EmulatorError> {
    let size = syscall_3(LinuxSysCalls::Write as u64, fd, data, len);
    if size == -1 {
        Err(EmulatorError::IOError(io::Error::last_os_error()))
    } else {
        Ok(size)
    }
}

pub fn writev(fd: u64, iovec: u64, vlen: u64) -> Result<i64, EmulatorError> {
    let size = syscall_3(LinuxSysCalls::WriteV as u64, fd, iovec, vlen);
    if size == -1 {
        Err(EmulatorError::IOError(io::Error::last_os_error()))
    } else {
        Ok(size)
    }
}
