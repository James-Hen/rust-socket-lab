use libc::*;
use std::io::{Error,stdin};
use std::mem;
use std::thread;
use std::time::Duration;

use crate::utils::*;
use crate::cstr;
const MAX_BUF: usize = 1460;

pub fn start(){
    unsafe {
        // let socket = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
        let socket = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
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

        // let addr = &servaddr as sockaddr_in as sockaddr;

        // let result = connect(socket, &servaddr as *const sockaddr_in as *const sockaddr, mem::size_of_val(&servaddr) as u32);
        // if result < 0 {
        //     println!("last OS error: {:?}", Error::last_os_error());
        //     close(socket);
        // }
        // println!("Client connected to server");

        // let msg = "Hello, server!".to_string();
        // println!("Client prepared for sending");
        // let n = sendto(socket, msg.as_bytes().as_ptr() as *const c_void, msg.len(), 0i32, &servaddr as *const sockaddr_in as *const sockaddr, mem::size_of_val(&servaddr) as u32);
        // println!("Client sended 'Hello, server!' successfully");

        // println!("Client prepared for receiving");
        // let mut buf = [0u8; MAX_BUF];
        // let n = recvfrom(socket, &mut buf as *mut _ as *mut c_void, buf.len(), 0i32, &mut servaddr as *mut sockaddr_in as *mut sockaddr, &mut len);
        // let rmsg = std::str::from_utf8(&buf[..n as usize]).unwrap().to_string();
        // // let rmsg = udp_recv(socket, addr).unwrap();
        // println!("Client received from server");
        // println!("{:?}", &rmsg);
        // println!("");

        let client_core = thread::spawn(move ||{
            loop{
                let mut buf = [0u8; MAX_BUF];
                let mut input = String::new();
                println!("Please input username:");
                stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                let n = sendto(socket, input.as_bytes().as_ptr() as *const c_void, input.len(), 0i32, &servaddr as *const sockaddr_in as *const sockaddr, mem::size_of_val(&servaddr) as u32);
            
                let n = recvfrom(socket, &mut buf as *mut _ as *mut c_void, buf.len(), 0i32, &mut servaddr as *mut sockaddr_in as *mut sockaddr, &mut len);
                let rmsg = std::str::from_utf8(&buf[..n as usize]).unwrap().to_string();
                println!("{}", rmsg);
                if strcmp(cstr!(rmsg), cstr!("User doesn't exist!")) == 0{
                    continue;
                }

                input = String::new();
                stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                let n = sendto(socket, input.as_bytes().as_ptr() as *const c_void, input.len(), 0i32, &servaddr as *const sockaddr_in as *const sockaddr, mem::size_of_val(&servaddr) as u32);

                let n = recvfrom(socket, &mut buf as *mut _ as *mut c_void, buf.len(), 0i32, &mut servaddr as *mut sockaddr_in as *mut sockaddr, &mut len);
                let rmsg = std::str::from_utf8(&buf[..n as usize]).unwrap().to_string();

                println!("{}", rmsg);
                if rmsg=="Success".to_string() {
                    break;
                    println!("Congratulations! GoodBYE!")
                }
                if rmsg=="You are banned!".to_string() {
                    break;
                }
            }
        });
        client_core.join().unwrap();
        close(socket);
    }
}