use std::io;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader};
use std::path::Path;
//use termion;
use local_ip_address::local_ip;

use crate::User;

pub fn set_user_name() -> String {
    println!("Please enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");
    name.trim().to_string()
}

pub fn save_user_data(user: &User) {
    let serialized_user = serde_json::to_string(user).expect("serialization failed");
    
    let mut data_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("users.json")
        .expect("creation failed");

    data_file.write_all(serialized_user.as_bytes()).expect("write failed");
}

pub fn load_user_data() -> Result<User, Box<dyn std::error::Error>> {
    let path = Path::new("users.json");

    if path.exists() {
        let data_file = File::open(path)?;
        let reader = BufReader::new(data_file);

        let user: User = serde_json::from_reader(reader)?;

        Ok(user)
    } else {
        Ok(User {
            name: String::new(),
            ip: local_ip().unwrap().to_string(),
        })
    }
}

pub fn update_user(user: &User, counter: &mut i32) {
    if counter == &mut 1{
        println!("Welcome, {}!", user.name.trim());
    }

    let updated_user   = User {
        name: user.name.clone(),
        ip: user.ip.clone(),
    };

    save_user_data(&updated_user);
}
/* 
pub fn clear(){
    println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}
*/