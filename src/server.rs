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
        let socket = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
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
        println!("Server binded to 127.0.0.1:8081");
        println!("Server is listening");
        listen(socket, 128);

        loop {
            let mut cliaddr: sockaddr_storage = mem::zeroed();
            let mut len = mem::size_of_val(&cliaddr) as u32;

            let client_socket = accept(socket, &mut cliaddr as *mut sockaddr_storage as *mut sockaddr, &mut len);
            if client_socket < 0 {
                println!("last OS error: {:?}", Error::last_os_error());
                break;
            }
            println!("Server connected to client");
            let db_clone = db.clone();
            let _handler = thread::spawn(move || {
                
                let rmsg = tcp_recv(client_socket).unwrap();
                println!("Server received from client");
                println!("{:?}", rmsg);

                let msg = "Hi, client!".to_string();
                println!("Server prepared for sending");
                tcp_send(client_socket, &msg).unwrap();
                println!("Server sended 'Hi, client!' successfully");
                println!("");
                let mut cnt = 0;
                loop {
                    let rmsg = tcp_recv(client_socket).unwrap();
                    println!("Someone tries to sign in using {:?}", rmsg);
                    let pwd;
                    match db_clone.get(rmsg.as_str()) {
                        Some(s) => {
                            tcp_send(client_socket, &"Please input password:".to_string()).unwrap();
                            pwd = s;
                        },
                        None => {
                            tcp_send(client_socket, &"User doesn't exist!".to_string()).unwrap();
                            continue;
                        },
                    };
                    let rmsg = tcp_recv(client_socket).unwrap();
                    if rmsg==*pwd{
                        tcp_send(client_socket, &"Success".to_string()).unwrap();
                        break;
                    }
                    else{
                        if cnt < 3 {
                            tcp_send(client_socket, &"Failure".to_string()).unwrap();
                        }
                        else{
                            tcp_send(client_socket, &"You are banned!".to_string()).unwrap();
                            break;
                        }
                    }     
                    cnt += 1;
                }

                close(client_socket);
            });
        }
        close(socket);
    }

}