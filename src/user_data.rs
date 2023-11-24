//user_data.rs
use std::io;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader};
use std::path::Path;
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
    let mut users = load_all_users();
    let user_index = users.iter().position(|u| u.name == user.name);

    match user_index {
        Some(index) => {
            // User exists, update the data
            users[index] = user.clone();
        }
        None => {
            // User doesn't exist, add to the list
            users.push(user.clone());
        }
    }

    let serialized_users = serde_json::to_string(&users).expect("serialization failed");

    let mut data_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("users.json")
        .expect("creation failed");

    data_file.write_all(serialized_users.as_bytes()).expect("write failed");
}

fn load_all_users() -> Vec<User> {
    if let Ok(data_file) = File::open("users.json") {
        let reader = BufReader::new(data_file);
        if let Ok(users) = serde_json::from_reader(reader) {
            return users;
        }
    }
    Vec::new()
}

pub fn load_user_data() -> Result<User, Box<dyn std::error::Error>> {
    let path = Path::new("users.json");

    if path.exists() {
        let data_file = File::open(path)?;
        let reader = BufReader::new(data_file);

        let users: Vec<User> = serde_json::from_reader(reader)?;

        // Find the user whose IP matches the local IP
        if let Some(user) = users.iter().find(|u| u.ip == local_ip().unwrap().to_string()) {
            Ok(user.clone())
        } else {
            // Handle the case when the user is not found
            Ok(User {
                name: String::new(),
                ip: local_ip().unwrap().to_string(),
            })
        }
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

pub fn clear(){
    println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}

pub fn get_ip_by_name(destination: &str) -> Option<String> {
    if let Ok(data_file) = File::open("users.json") {
        let reader = BufReader::new(data_file);
        if let Ok(users) = serde_json::from_reader::<_, Vec<User>>(reader) {
            if let Some(user) = users.iter().find(|u| u.name == destination) {
                return Some(user.ip.clone());
            }
        }
    }
    None
}
