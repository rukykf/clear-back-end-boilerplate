use juniper::RootNode;
use server::filters::Mutation;
use server::filters::Query;
use server::filters::Schema;

#[tokio::test]
async fn testing_123() {
    assert_eq!(2 * 2, 4);
    server::filters::query::say_hello();
}

// HELPERS
// type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

struct App {
    db_context: db_client::Pool,
    schema: Schema,
}
fn get_app() {}
