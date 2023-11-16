use serde::{Serialize, Deserialize};
mod messaging;
use messaging::update_message;
mod user_data;
use user_data::{set_user_name, save_user_data, load_user_data, update_user, clear};

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

        update_user(&user, &mut counter);
        
        update_message();
        
        counter += 1;

        clear();

        println!("Welcome, {}!", user.name.trim());

}
}

