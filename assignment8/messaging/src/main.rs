use std::rc::Rc;

struct Message {
    sender_name: String,
    message_content: String,
}

impl Message {
    fn new(sender_name: String, message_content: String) -> Rc<Self> {
        Rc::new(Message {sender_name, message_content})
    }

    fn sender_name(&self) -> &str {
        &self.sender_name
    }

    fn message_content(&self) -> &str {
        &self.message_content
    }
}

fn main() {
    let message = Message::new("Alice".to_string(), "Hello, Bob!".to_string());
    println!("Sender: {}", message.sender_name());
    println!("Message: {}", message.message_content());
}

