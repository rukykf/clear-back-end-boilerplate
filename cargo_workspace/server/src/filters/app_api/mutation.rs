use super::context::Context;
use juniper::graphql_object;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn dummy() -> bool {
        true
    }

    fn hello_world(context: &Context, steps: i32, word: String) -> String {
        context.start_me();
        format!("My word {} is going {} steps", word, steps)
    }
}
