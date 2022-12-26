use std::ffi::CString;
use std::mem::{self, MaybeUninit};

use crate::prelude::*;

pub fn do_bind<A: Addr>(host_fd: HostFd, addr: &A) -> Result<()> {
    let fd = host_fd as i32;
    let (c_addr_storage, c_addr_len) = addr.to_c_storage();
    let c_addr_ptr = &c_addr_storage as *const _ as _;
    let c_addr_len = c_addr_len as u32;
    #[cfg(not(feature = "sgx"))]
    try_libc!(libc::bind(fd, c_addr_ptr, c_addr_len));
    #[cfg(feature = "sgx")]
    try_libc!(libc::ocall::bind(fd, c_addr_ptr, c_addr_len));
    Ok(())
}

pub fn do_close(host_fd: HostFd) -> Result<()> {
    let fd = host_fd as i32;
    #[cfg(not(feature = "sgx"))]
    try_libc!(libc::close(fd));
    #[cfg(feature = "sgx")]
    try_libc!(libc::ocall::close(fd));
    Ok(())
}

pub fn do_unlink(path: &String) -> Result<()> {
    let c_string =
        CString::new(path.as_bytes()).map_err(|_| errno!(EINVAL, "cstring new failure"))?;
    let c_path = c_string.as_c_str().as_ptr();
    #[cfg(not(feature = "sgx"))]
    try_libc!(libc::unlink(c_path));
    #[cfg(feature = "sgx")]
    try_libc!(libc::ocall::unlink(c_path));
    Ok(())
}

pub fn do_connect<A: Addr>(host_fd: HostFd, addr: Option<&A>) -> Result<()> {
    let fd = host_fd as i32;
    let (c_addr_storage, c_addr_len) = match addr {
        Some(addr_inner) => addr_inner.to_c_storage(),
        None => {
            let mut sockaddr_storage =
                unsafe { MaybeUninit::<libc::sockaddr_storage>::zeroed().assume_init() };
            sockaddr_storage.ss_family = libc::AF_UNSPEC as _;
            (sockaddr_storage, mem::size_of::<libc::sa_family_t>())
        }
    };
    let c_addr_ptr = &c_addr_storage as *const _ as _;
    let c_addr_len = c_addr_len as u32;
    #[cfg(not(feature = "sgx"))]
    try_libc!(libc::connect(fd, c_addr_ptr, c_addr_len));
    #[cfg(feature = "sgx")]
    try_libc!(libc::ocall::connect(fd, c_addr_ptr, c_addr_len));
    Ok(())
}
