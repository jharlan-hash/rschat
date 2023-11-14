use std::io;
use std::fs;

pub fn get_message() -> String {
    println!("message: ");
    let mut message = String::new();
    io::stdin().read_line(&mut message)
        .expect("Failed to read line");

    message = message.trim().to_string();  

    commands(&message);

    message
}

pub fn msg_destination() -> String{
    println!("message destination: ");
    let mut message_destination = String::new();
    io::stdin().read_line(&mut message_destination)
        .expect("Failed to read line");

    message_destination = message_destination.trim().to_string();  

    commands(&message_destination);
    
    message_destination
}  

pub fn commands(message: &String){
    if message == "clear" {
        println!("clearing data.json");

        fs::remove_file("data.json").expect("Error clearing file data.json");
        fs::remove_file("message.json").expect("Error clearing file message.json");
        std::process::exit(2);
    }

    if message == "exit" {
        std::process::exit(1);
    }

    if message == "chdest"{
        println!("changing destination");
        msg_destination();
    }

    if message == "help" {
        println!("commands: clear, exit, help, chdest");
        
    }
}