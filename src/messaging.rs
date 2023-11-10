use std::io;
use std::fs;

pub fn get_message() -> String {
    println!("message: ");
    let mut message = String::new();
    io::stdin().read_line(&mut message)
        .expect("Failed to read line");

    message = message.trim().to_string();

    if message == "clear" {
        println!("clearing data.json");

        if let Err(err) = fs::remove_file("data.json") {
            if err.kind() == std::io::ErrorKind::NotFound {
                println!("File not found.");
            } else {
                eprintln!("Error: {}", err);
            }
        } else {
            println!("File successfully removed.");
            std::process::exit(1);
        }
    }

    if message == "exit" {
        std::process::exit(0);
    }

    message
}
