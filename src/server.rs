use libc::*;
use std::io::Error;
use std::mem;
use std::thread;
use std::collections::HashMap;
use std::time::Duration;

use crate::utils::*;
const MAX_BUF: usize = 1460;

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
            let mut cliaddr: sockaddr_storage = mem::zeroed();
            let mut len = mem::size_of_val(&cliaddr) as u32;

            // let client_socket = accept(socket, &mut cliaddr as *mut sockaddr_storage as *mut sockaddr, &mut len);
            // if client_socket < 0 {
            //     println!("last OS error: {:?}", Error::last_os_error());
            //     break;
            // }
            // println!("Server connected to client");
            let db_clone = db.clone();
            let _handler = thread::spawn(move || {
                // let mut buf = [0u8; MAX_BUF];
                // let n = recvfrom(socket, &mut buf as *mut _ as *mut c_void, buf.len(), 0i32, &mut cliaddr as *mut sockaddr_storage as *mut sockaddr, &mut len);
                // let rmsg = std::str::from_utf8(&buf[..n as usize]).unwrap().to_string();
                // // let rmsg = udp_recv(socket, *mut cliaddr as *mut sockaddr_storage as *mut sockaddr).unwrap();
                // println!("Server received from client");
                // println!("{:?}", rmsg);

                // let msg = "Hi, client!".to_string();
                // println!("Server prepared for sending");
                // let n = sendto(socket, msg.as_bytes().as_ptr() as *const c_void, msg.len(), 0i32, &cliaddr as *const sockaddr_storage as *const sockaddr, mem::size_of_val(&cliaddr) as u32);
                // println!("Server sended 'Hi, client!' successfully");
                // println!("");
                let mut cnt = 0;
                loop {
                    let mut buf = [0u8; MAX_BUF];
                    let n = recvfrom(socket, &mut buf as *mut _ as *mut c_void, buf.len(), 0i32, &mut cliaddr as *mut sockaddr_storage as *mut sockaddr, &mut len);
                    let rmsg = std::str::from_utf8(&buf[..n as usize]).unwrap().to_string();
                    // let rmsg = udp_recv(socket, *mut cliaddr as *mut sockaddr_storage as *mut sockaddr).unwrap();
                    println!("Someone tries to sign in using {:?}", rmsg);
                    let pwd;
                    match db_clone.get(rmsg.as_str()) {
                        Some(s) => {
                            let msg = "Please input password:".to_string();
                            let n = sendto(socket, msg.as_bytes().as_ptr() as *const c_void, msg.len(), 0i32, &cliaddr as *const sockaddr_storage as *const sockaddr, mem::size_of_val(&cliaddr) as u32);             
                            pwd = s;
                        },
                        None => {
                            let msg = "User doesn't exist!".to_string();
                            let n = sendto(socket, msg.as_bytes().as_ptr() as *const c_void, msg.len(), 0i32, &cliaddr as *const sockaddr_storage as *const sockaddr, mem::size_of_val(&cliaddr) as u32);
                            continue;
                        },
                    };
                    let n = recvfrom(socket, &mut buf as *mut _ as *mut c_void, buf.len(), 0i32, &mut cliaddr as *mut sockaddr_storage as *mut sockaddr, &mut len);
                    let rmsg = std::str::from_utf8(&buf[..n as usize]).unwrap().to_string();
                    // let rmsg = udp_recv(socket, *mut cliaddr as *mut sockaddr_storage as *mut sockaddr).unwrap();
                    if rmsg==*pwd{
                        let msg = "Success".to_string();
                        let n = sendto(socket, msg.as_bytes().as_ptr() as *const c_void, msg.len(), 0i32, &cliaddr as *const sockaddr_storage as *const sockaddr, mem::size_of_val(&cliaddr) as u32);
                        break;
                    }
                    else{
                        if cnt < 3 {
                            let msg = "Failure".to_string();
                            let n = sendto(socket, msg.as_bytes().as_ptr() as *const c_void, msg.len(), 0i32, &cliaddr as *const sockaddr_storage as *const sockaddr, mem::size_of_val(&cliaddr) as u32);
                            }
                        else{
                            let msg = "You are banned!".to_string();
                            let n = sendto(socket, msg.as_bytes().as_ptr() as *const c_void, msg.len(), 0i32, &cliaddr as *const sockaddr_storage as *const sockaddr, mem::size_of_val(&cliaddr) as u32);
                            break;
                        }
                    }     
                    cnt += 1;
                }

            });
            thread::sleep(Duration::from_millis(10000));
        }
        close(socket);
    }

}