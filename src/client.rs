use libc::*;
use std::io::{Error,stdin};
use std::mem;
use std::thread;

use crate::utils::*;

pub fn user_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn start(){
    unsafe {
        let socket = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
        if socket < 0 {
            panic!("last OS error: {:?}", Error::last_os_error());
        }
        println!("Client established");

        let saddr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: 8080u16.to_be(),
            sin_addr: in_addr {
                s_addr: u32::from_be_bytes([127, 0, 0, 1]).to_be()
            },
            sin_zero: mem::zeroed()
        };

        // POINTER FORCE CAST!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        let server_addr = &*(&saddr as *const sockaddr_in as *const sockaddr) as &sockaddr;

        let client_core = thread::spawn(move || {
            'try_login:
            loop {
                println!("Please input username:");
                udp_send(socket, &user_input(), server_addr).unwrap();

                'try_pwd:
                loop {
                    let (rmsg, _from_addr) = udp_recv(socket).unwrap();
                    println!("{}", rmsg);
                    if rmsg == "User doesn't exist!".to_string() {
                        continue 'try_login;
                    }
                    udp_send(socket, &user_input(), server_addr).unwrap();

                    let (rmsg, _from_addr) = udp_recv(socket).unwrap();
                    println!("{}", rmsg);

                    match rmsg.as_str() {
                        "Success" => {
                            println!("Congratulations! GoodBYE!");
                            break 'try_login;
                        },
                        "Failure, please retry" => {
                            continue 'try_pwd;
                        },
                        "You are banned!" => {
                            break 'try_login;
                        },
                        _ => {
                            println!("Unknown Message");
                            break 'try_login;
                        },
                    }
                }
            }
        });
        client_core.join().unwrap();
        close(socket);
    }
}