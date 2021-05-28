use modules_assignment::Facebook;
use std::io;

fn main() {
    let mut username = String::new();
    let mut password = String::new();
    let mut content = String::new();

    loop {
        username.clear();
        password.clear();

        println!("Enter your username:");
        io::stdin().read_line(&mut username).expect("Failed");

        println!("Enter your password:");
        io::stdin().read_line(&mut password).expect("Failed");

        username = username.trim().to_string();
        password = password.trim().to_string();


        if Facebook::Login::enter_login_detail(&username, &password) {
            println!("You've logged in successfully!");
            println!("Say something:");
            io::stdin().read_line(&mut content).expect("Failed");

            Facebook::Post::post_content(content);
            Facebook::Logout::logout();
            break;
        }
        else {
            println!("Wrong username or password. Try again");
        }
    }
}