use libc::*;
use std::io::Error;
use std::mem;
use std::thread;
use std::collections::HashMap;

use crate::utils::*;


pub fn start(){
    let db = HashMap::from([
        ("zwz".to_string(), "zwzzwzzwz".to_string()),
        ("ilove".to_string(),"network".to_string()),
        ("socket".to_string(),"interesting".to_string()),
        ("rust_string".to_string(),"stupid".to_string()),
    ]);
    unsafe {
        // server core
        // let socket = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
        let socket = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
        if socket < 0 {
            panic!("last OS error: {:?}", Error::last_os_error());
        }

        let servaddr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: 8080u16.to_be(),
            sin_addr: in_addr {
                s_addr: u32::from_be_bytes([127, 0, 0, 1]).to_be()
            },
            sin_zero: mem::zeroed()
        };
        println!("Server established");
        let result = bind(socket, &servaddr as *const sockaddr_in as *const sockaddr, mem::size_of_val(&servaddr) as u32);
        if result < 0 {
            println!("last OS error: {:?}", Error::last_os_error());
            close(socket);
        }
        println!("Server binded to 127.0.0.1:8080");
        // println!("Server is listening");
        // listen(socket, 128);

        loop {
            let mut cliaddr: sockaddr = mem::zeroed();
            let mut len = mem::size_of_val(&cliaddr) as u32;

            // let client_socket = accept(socket, &mut cliaddr as *mut sockaddr_storage as *mut sockaddr, &mut len);
            // if client_socket < 0 {
            //     println!("last OS error: {:?}", Error::last_os_error());
            //     break;
            // }
            // println!("Server connected to client");
            let db_clone = db.clone();
            let _handler = thread::spawn(move || {
                
                let rmsg = udp_recv(socket, cliaddr).unwrap();
                println!("Server received from client");
                println!("{:?}", rmsg);

                let msg = "Hi, client!".to_string();
                println!("Server prepared for sending");
                udp_send(socket, &msg, cliaddr).unwrap();
                println!("Server sended 'Hi, client!' successfully");
                println!("");
                let mut cnt = 0;
                loop {
                    let rmsg = udp_recv(socket, cliaddr).unwrap();
                    println!("Someone tries to sign in using {:?}", rmsg);
                    let pwd;
                    match db_clone.get(rmsg.as_str()) {
                        Some(s) => {
                            udp_send(socket, &"Please input password:".to_string(), cliaddr).unwrap();
                            pwd = s;
                        },
                        None => {
                            udp_send(socket, &"User doesn't exist!".to_string(), cliaddr).unwrap();
                            continue;
                        },
                    };
                    let rmsg = udp_recv(socket, cliaddr).unwrap();
                    if rmsg==*pwd{
                        udp_send(socket, &"Success".to_string(), cliaddr).unwrap();
                        break;
                    }
                    else{
                        if cnt < 3 {
                            udp_send(socket, &"Failure".to_string(), cliaddr).unwrap();
                        }
                        else{
                            udp_send(socket, &"You are banned!".to_string(), cliaddr).unwrap();
                            break;
                        }
                    }     
                    cnt += 1;
                }

            });
        }
        close(socket);
    }

}