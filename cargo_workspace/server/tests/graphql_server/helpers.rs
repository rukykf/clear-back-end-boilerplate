use juniper::{
    DefaultScalarValue, EmptySubscription, ExecutionError, GraphQLError, Value, Variables,
};
use server::filters::{app_api, Mutation, Query, Schema};
use sqlx::pool::PoolConnection;
use sqlx::{Connection, Executor, PgConnection, Postgres};

pub struct App {
    pub db_connection_pool: db_client::Pool,
    pub graphql_schema: Schema,
    pub graphql_context: app_api::Context,
}

impl App {
    pub async fn new() -> Self {
        // Create a new database in the Postgres instance for each test
        // To ensure that each test is running on a "fresh" slate
        let db_name = uuid::Uuid::new_v4().to_string();
        let mut pg_connect_options = sqlx::postgres::PgConnectOptions::new()
            .host("localhost")
            .username("postgres")
            .password("mysecretpassword")
            .port(5431);

        let mut connection = PgConnection::connect_with(&pg_connect_options)
            .await
            .expect("Failed to connect to Postgres instance for testing");

        connection
            .execute(&*format!(r#"CREATE DATABASE "{}";"#, db_name))
            .await
            .expect("Failed to create test database");

        // Migrate the dtabase
        let connection_pool = db_client::Pool::connect_with(pg_connect_options.database(&db_name))
            .await
            .expect("Failed to connect to test database");

        dbg!(&connection_pool);

        sqlx::migrate!("../migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the test database");

        let db_context = db_client::Context::new(connection_pool.clone());
        let juniper_context = app_api::Context(db_context.clone());
        let juniper_schema = Schema::new(Query, Mutation, EmptySubscription::new());

        App {
            db_connection_pool: connection_pool,
            graphql_schema: juniper_schema,
            graphql_context: juniper_context,
        }
    }

    pub async fn execute_query<'a>(
        &'a self,
        graphql_query: &'a str,
    ) -> Result<(Value, Vec<ExecutionError<DefaultScalarValue>>), GraphQLError> where {
        juniper::execute(
            graphql_query,
            None,
            &self.graphql_schema,
            &Variables::new(),
            &self.graphql_context,
        )
        .await
    }

    pub async fn get_db_conn(&self) -> PoolConnection<Postgres> {
        self.db_connection_pool
            .acquire()
            .await
            .expect("Could not get connection")
    }
}
