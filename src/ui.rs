pub const TITLE: &str = r#"
 ____   __    ___  __ _  ____  ____    
/ ___) /  \  / __)(  / )(  __)(_  _) 
\___ \(  O )( (__  )  (  ) _)   )(  
(____/ \__/  \___)(__\_)(____) (__)
"#;

pub const GREETING: &str = r#"
--------------------------------------------------------------------
                    Hi! "#;

pub const MENU_HINT: &str = r#"
                    Welcome To Socket!
                    (1) Become server
                    (2) Become client
                    (3) Exit
--------------------------------------------------------------------
Please type in your option:
"#;
pub const CHOICE_RANGE: std::ops::Range::<u8> = 1..4;

use std::io::{ stdin, stdout, Error, ErrorKind, Result };
use std::net::Ipv4Addr;

// use rand::{ thread_rng, Rng };
// use rand::prelude::SliceRandom;
// pub use crossterm::{
//     ExecutableCommand, QueueableCommand, Result,
//     terminal::{Clear, ClearType},
//     cursor,
//     style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
// };
pub enum UsersIdea {
    servergame,
    clientgame,
    exitgame,
}

pub fn show_main_menu() -> Result<UsersIdea,> {
    // stdout()
    //     // .execute(Clear(ClearType::All))?
    //     // .execute(cursor::MoveTo(0, 0))?
    //     .execute(Print(TITLE))?
    //     .execute(Print(GREETING))?
    //     .execute(Print(&format!("{}", &name)))?
    //     .execute(Print(MENU_HINT));
    //     // .execute(cursor::Show).unwrap();
    println!("Welcome To Socket!");
    println!("(1) Become server");
    println!("(2) Become client");
    println!("(3) Exit");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let choice = loop {
        match line.trim().parse::<u8>() {
            Ok(num) if CHOICE_RANGE.contains(&num)
                => { break num; },
            _ => {
                println!("Please type in a number:");
                line = String::new();
                stdin().read_line(&mut line).unwrap();
            }
        }
    };
    match choice {
        1 => {
            Ok(UsersIdea::servergame)
        },
        2 => {
            // let addr = input_ip_addr_port();
            // Ok(UsersIdea::JoinGame(addr))
            Ok(UsersIdea::clientgame)
        },
        3 => {
            Ok(UsersIdea::exitgame)
        },
        _ => Err(Error::new(ErrorKind::Other, "Choice out of range")),
    }
}