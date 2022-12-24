use super::context::Context;
use juniper::graphql_object;

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn dummy() -> bool {
        true
    }

    fn hello(context: &Context, number: i32, word: String) -> String {
        std::thread::sleep(std::time::Duration::from_secs(15));
        let output = format!("my word is {}, and number is {}", word, number);
        return output;
    }
}
