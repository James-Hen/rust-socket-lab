use libc::*;
use std::io::{Error,stdin};
use std::mem;
use std::thread;
use std::time::Duration;

use crate::utils::*;
use crate::cstr;

pub fn start(){
    unsafe {
        let socket = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
        if socket < 0 {
            panic!("last OS error: {:?}", Error::last_os_error());
        }
        println!("Client established");

        let servaddr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: 8080u16.to_be(),
            sin_addr: in_addr {
                s_addr: u32::from_be_bytes([127, 0, 0, 1]).to_be()
            },
            sin_zero: mem::zeroed()
        };

        let result = connect(socket, &servaddr as *const sockaddr_in as *const sockaddr, mem::size_of_val(&servaddr) as u32);
        if result < 0 {
            println!("last OS error: {:?}", Error::last_os_error());
            close(socket);
        }
        println!("Client connected to server");

        let msg = "Hello, server!".to_string();
        println!("Client prepared for sending");
        tcp_send(socket, &msg).unwrap();
        println!("Client sended 'Hello, server!' successfully");

        println!("Client prepared for receiving");
        let rmsg = tcp_recv(socket).unwrap();
        println!("Client received from server");
        println!("{:?}", &rmsg);
        println!("");

        let client_core = thread::spawn(move ||{
            loop{
                let mut input = String::new();
                println!("Please input username:");
                stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                tcp_send(socket, &input).unwrap();
            
                let rmsg = tcp_recv(socket).unwrap();
                println!("{}", rmsg);
                if strcmp(cstr!(rmsg), cstr!("User doesn't exist!")) == 0{
                    continue;
                }

                input = String::new();
                stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                tcp_send(socket, &input).unwrap();

                let rmsg = tcp_recv(socket).unwrap();
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