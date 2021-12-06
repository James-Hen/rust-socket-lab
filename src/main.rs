pub mod unix_socket;
pub mod server;
pub mod client;
pub mod ui;

use unix_socket::*;

fn main() {
    use std::io::Error;
    use std::mem;
    use std::thread;
    use std::time::Duration;

    // let mut name = "weizhan";
    //menu
    loop{
        let choice = ui::show_main_menu().unwrap();
        match choice{
            ui::UsersIdea::BecomeServer => {
                //server
                let server_main = thread::spawn(move || {
                    server::start();
                });
    
                thread::sleep(Duration::from_millis(100));
    
                server_main.join().unwrap();
            },
    
            ui::UsersIdea::BecomeClient => {
                // client
                client::start();
            },
    
            ui::UsersIdea::Exit => {
                break;
            }
        }
    }
}
