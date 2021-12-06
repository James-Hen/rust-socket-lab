pub const TITLE: &str = r#"
 ____   __    ___  __ _  ____  ____    
/ ___) /  \  / __)(  / )(  __)(_  _) 
\___ \(  O )( (__  )  (  ) _)   )(  
(____/ \__/  \___)(__\_)(____) (__)
"#;

pub const MENU_HINT: &str = r#"
--------------------------------------------------------------------
                    Welcome To Socket!
                    (1) Become server
                    (2) Become client
                    (3) Exit
--------------------------------------------------------------------
Please type in your option:
"#;

pub const CHOICE_RANGE: std::ops::Range::<u8> = 1..4;

use std::io::{ stdin, stdout, Write, Error, ErrorKind, Result };

pub enum UsersIdea {
    BecomeServer,
    BecomeClient,
    Exit,
}

pub fn show_main_menu() -> Result<UsersIdea,> {
    stdout().write(TITLE.as_bytes()).unwrap();
    stdout().write(MENU_HINT.as_bytes()).unwrap();
    // read choices
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
            Ok(UsersIdea::BecomeServer)
        },
        2 => {
            Ok(UsersIdea::BecomeClient)
        },
        3 => {
            Ok(UsersIdea::Exit)
        },
        _ => Err(Error::new(ErrorKind::Other, "Choice out of range")),
    }
}