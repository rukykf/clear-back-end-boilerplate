use db_client;
use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap)]
pub struct Context(pub db_client::Context);

impl Context {
    pub fn start_me(&self) {
        let hello = &self.0.say_hello();
        println!("The word on the street is: {}", hello)
    }
}

impl juniper::Context for Context {}
