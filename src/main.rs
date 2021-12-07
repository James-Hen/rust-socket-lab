pub mod server;
pub mod client;
pub mod ui;
#[macro_use]
pub mod utils;

fn main() {
    use std::thread;

    loop{
        let choice = ui::show_main_menu().unwrap();
        match choice{
            ui::UsersIdea::BecomeServer => {
                //server
                let server_main = thread::spawn(move || {
                    server::start();
                });
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
