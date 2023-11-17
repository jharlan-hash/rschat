//messaging.rs
use std::io;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use crate::Message;
use crate::user_data::get_ip_by_name;



pub fn get_message() -> String {
    println!("message: ");
    let mut message = String::new();
    io::stdin().read_line(&mut message)
        .expect("Failed to read line");

    message = message.trim().to_string();  

    commands(&message);

    message
}

pub fn msg_destination() -> String {
    println!("message destination: ");
    let mut message_destination = String::new();
    io::stdin().read_line(&mut message_destination)
        .expect("Failed to read line");

    message_destination = message_destination.trim().to_string();

    if let Some(ip) = get_ip_by_name(&message_destination) {
        println!("Destination IP: {}", ip);
    } else {
        println!("User not found.");
        // Handle the case when the user is not found, perhaps ask the user to enter the destination again.
    }

    message_destination
}

pub fn commands(message: &String){
    if message == "clear" {
        println!("clearing data");

        //fs::remove_file("users.json").expect("Error clearing file data.json");
        fs::remove_file("message.json").expect("Error clearing file message.json");
        std::process::exit(1);
    }

    if message == "exit" {
        std::process::exit(1);
    }

    if message == "help" {
        println!("commands: clear, exit, help");
    }
}

pub fn update_message() {

    let updated_message: Message = Message {
        message: get_message(),
        destination: msg_destination()
    };

    save_message_data(&updated_message);
}

pub fn save_message_data(message: &Message) {
    let serialized_message = serde_json::to_string(message).expect("serialization failed");
    
    let mut data_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("message.json")
        .expect("creation failed");

    data_file.write_all(serialized_message.as_bytes()).expect("write failed");
}