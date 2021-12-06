use crate::unix_socket::*;
use std::io::Error;
use std::mem;

pub fn start(){
    unsafe {
        let socket = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
        // let socket = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
        if socket < 0 {
            panic!("last OS error: {:?}", Error::last_os_error());
        }
        println!("Client established");

        let mut servaddr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: 8080u16.to_be(),
            sin_addr: in_addr {
                s_addr: u32::from_be_bytes([127, 0, 0, 1]).to_be()
            },
            sin_zero: mem::zeroed()
        };
        let mut len = mem::size_of_val(&servaddr) as u32;

        let msg = b"Hello, server!";
        println!("Client prepared for sending");
        
        let n = sendto(socket, msg as *const _ as *const c_void, msg.len(), 0i32, &servaddr as *const sockaddr_in as *const sockaddr, mem::size_of_val(&servaddr) as u32);
        if n <= 0 {
            println!("last OS error: {:?}", Error::last_os_error());
            close(socket);
        }
        println!("Client sended 'Hello, server!' successfully");
        let mut buf = [0u8; 64];
        println!("Client prepared for receiving");
        
        let n = recvfrom(socket, &mut buf as *mut _ as *mut c_void, buf.len(), 0i32, &mut servaddr as *mut sockaddr_in as *mut sockaddr, &mut len);
        if n <= 0 {
            println!("last OS error: {:?}", Error::last_os_error());
        }
        println!("Client received from server");
        println!("{:?}", String::from_utf8_lossy(&buf[0..n as usize]));
        println!("");
        close(socket);
    }
}