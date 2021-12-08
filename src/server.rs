use libc::*;
use std::io::Error;
use std::mem;
use std::thread;
use std::collections::HashMap;

use crate::utils::*;

pub fn start(){
    let db = std::sync::Arc::new(HashMap::from([
        ("zwz".to_string(), "zwzzwzzwz".to_string()),
        ("ilove".to_string(),"network".to_string()),
        ("socket".to_string(),"interesting".to_string()),
        ("rust_string".to_string(),"stupid".to_string()),
        ("rust_refs_cast".to_string(),"also_stupid".to_string()),
    ]));
    unsafe {
        // server core, Using UDP
        let socket = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
        if socket < 0 {
            panic!("last OS error: {:?}", Error::last_os_error());
        }

        let servaddr_in = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: 8080u16.to_be(),
            sin_addr: in_addr {
                s_addr: u32::from_be_bytes([127, 0, 0, 1]).to_be()
            },
            sin_zero: mem::zeroed()
        };
        let servaddr = &servaddr_in as *const sockaddr_in as *const sockaddr;

        println!("Server established");
        let result = bind(socket, servaddr, mem::size_of::<sockaddr>() as u32);
        if result < 0 {
            println!("last OS error: {:?}", Error::last_os_error());
            close(socket);
        }
        println!("Server binded to 127.0.0.1:8080");

        loop {
            let db_arc = db.clone();
            let (rmsg, from_addr) = udp_recv(socket).unwrap();
            println!("Someone tries to sign in using {:?}", rmsg);
            let handler = thread::spawn(move || {
                let mut cnt = 0;
                let mut success_flag = false;
                while cnt < 3 {
                    let pwd;
                    match db_arc.get(rmsg.as_str()) {
                        Some(s) => {
                            let msg = "Please input password:".to_string();
                            udp_send(socket, &msg, &from_addr).unwrap();           
                            pwd = s;
                        },
                        None => {
                            let msg = "User doesn't exist!".to_string();
                            udp_send(socket, &msg, &from_addr).unwrap();
                            return;
                        },
                    };
                    let (rmsg, from_addr) = udp_recv(socket).unwrap();
                    if rmsg == *pwd {
                        success_flag = true;
                        break;
                    }
                    else {
                        let msg = "Failure, please retry".to_string();
                        udp_send(socket, &msg, &from_addr).unwrap();
                    }
                    cnt += 1;
                }
                if success_flag {
                    let msg = "Success".to_string();
                    udp_send(socket, &msg, &from_addr).unwrap();
                }
                else {
                    let msg = "You are banned!".to_string();
                    udp_send(socket, &msg, &from_addr).unwrap();
                }
            });
            handler.join().unwrap();
        }

        close(socket);
    } // unsafe block ends
}