use db_client;
use shrinkwraprs::Shrinkwrap;
use uuid::{Error, Uuid};

#[derive(Shrinkwrap)]
pub struct Context(pub db_client::Context);

impl Context {
    pub fn start_me(&self) {
        let hello = &self.0.say_hello();
        println!("The word on the street is: {}", hello)
    }

    pub fn get_user_id_for_auth_token(&self, _token: String) -> Result<Uuid, Error> {
        // TODO
        let user_id = Uuid::parse_str(test_utils::sample_user_id().as_str()).unwrap();
        Ok(user_id)
    }
}

impl juniper::Context for Context {}
