pub mod unix_socket;

use unix_socket::*;

fn main() {
    use std::io::Error;
    use std::mem;
    use std::thread;
    use std::time::Duration;

    thread::spawn(|| {

        // server
        unsafe {
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
            println!("Server binded");
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

                thread::spawn(move || {
                    loop {
                        let mut buf = [0u8; 64];
                        let n = read(client_socket, &mut buf as *mut _ as *mut c_void, buf.len());
                        if n <= 0 {
                            break;
                        }
                        println!("Server received from client");
                        println!("{:?}", String::from_utf8_lossy(&buf[0..n as usize]));

                        let msg = b"Hi, client!";
                        println!("Server prepared for sending");
                        let n = write(client_socket, msg as *const _ as *const c_void, msg.len());
                        if n <= 0 {
                            break;
                        }
                    }

                    close(client_socket);
                });
            }

            close(socket);
        }

    });

    thread::sleep(Duration::from_millis(100));

    // client
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

        let msg = b"Hello, server!";
        println!("Client prepared for sending");
        let n = write(socket, msg as *const _ as *const c_void, msg.len());
        if n <= 0 {
            println!("last OS error: {:?}", Error::last_os_error());
            close(socket);
        }

        let mut buf = [0u8; 64];
        println!("Client prepared for receiving");
        let n = read(socket, &mut buf as *mut _ as *mut c_void, buf.len());
        if n <= 0 {
            println!("last OS error: {:?}", Error::last_os_error());
        }
        println!("Client received from server");
        println!("{:?}", String::from_utf8_lossy(&buf[0..n as usize]));

        close(socket);
    }
}
