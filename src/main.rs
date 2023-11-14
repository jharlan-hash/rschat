use serde::{Serialize, Deserialize};
mod messaging;
use messaging::{get_message, msg_destination};
mod user_data;
use user_data::{set_user_name, save_user_data, load_user_data, save_message_data};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    message: String,
    destination: String,
}

fn main() {
    let mut counter = 1;
    loop{
        //dbg!(counter);
        let mut user = load_user_data().expect("failed to load user data");
        
        if user.name.is_empty() {
            user.name = set_user_name();
            save_user_data(&user);
        }

        user_info(&user, &mut counter);

        counter += 1;

        message_info();

}
}

pub fn user_info(user: &User, counter: &mut i32) {
    if counter == &mut 1{
        println!("Welcome, {}!", user.name.trim());
    }

    let updated_user   = User {
        name: user.name.clone(),
        ip: user.ip.clone(),
    };

    save_user_data(&updated_user);
}

pub fn message_info() {

    let updated_message: Message = Message {
        message: get_message(),
        destination: msg_destination()
    };

    save_message_data(&updated_message);
}