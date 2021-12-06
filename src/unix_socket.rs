/// pub mod unix_socket: declared extern unix socket api

pub use std::os::raw::c_char;
pub use std::ffi::c_void;

pub const AF_INET: i32 = 2;
pub const AF_INET6: i32 = 10;
pub const SOCK_STREAM: i32 = 1;
pub const IPPROTO_TCP: i32 = 6;

// const below are for UDP

pub const SOCK_DGRAM: i32 = 2;
pub const IPPROTO_UDP: i32 = 0;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    _unused: [u8; 126]
}

extern {
    pub fn socket(fanily: i32, ty: i32, protocol: i32) -> i32;
    pub fn connect(sockfd: i32, servaddr: *const sockaddr, addrlen: u32) -> i32;
    pub fn bind(sockfd: i32, myaddr: *const sockaddr, addrlen: u32) -> i32;
    pub fn listen(sockfd: i32, backlog: i32);
    pub fn accept(sockfd: i32, cliaddr: *mut sockaddr, addrlen: *mut u32) -> i32;
    pub fn close(sockfd: i32) -> i32;
    pub fn getsockname(sockfd: i32, localaddr: *mut sockaddr, addrlen: *mut u32) -> i32;
    pub fn getpeername(sockfd: i32, peeraddr: *mut sockaddr, addrlen: *mut u32) -> i32;
    pub fn read(fd: i32, buf: *mut std::ffi::c_void, count: usize) -> isize;
    pub fn write(fd: i32, buf: *const std::ffi::c_void, count: usize) -> isize;

    // Functions below are for UDP

    pub fn sendto(sockfd: i32, buf: *const std::ffi::c_void, len: usize, flags: i32, dest_addr: *const sockaddr, addrlen: u32) -> isize;
    pub fn recvfrom(sockfd: i32, buf: *mut std::ffi::c_void, len: usize, flags: i32, src_addr: *const sockaddr, addrlen: *mut u32) -> isize;

}