use super::context::Context;
use chrono::{DateTime, Local};
use juniper::graphql_object;
use uuid::Uuid;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_new_entry(context: &Context, base64_image: String) -> bool {
        let date_time: DateTime<Local> = Local::now();
        let id = Uuid::new_v4();

        sqlx::query!(
            r#"INSERT INTO photo_entries (entry_id, created_at, base64_image) Values ($1, $2, $3)"#,
            id,
            date_time,
            base64_image
        )
        .execute(&mut context.0.conn().await.unwrap())
        .await
        .unwrap();

        true
    }

    async fn delete_entry(context: &Context, entry_id: String) -> bool {
        let entry_id = Uuid::parse_str(entry_id.as_str()).unwrap();
        sqlx::query!(
            r#"DELETE FROM photo_entries
        WHERE 
            entry_id = $1"#,
            entry_id,
        )
        .execute(&mut context.0.conn().await.unwrap())
        .await
        .unwrap();

        true
    }

    fn hello_world(context: &Context, steps: i32, word: String) -> String {
        context.start_me();
        format!("My word {} is going {} steps", word, steps)
    }
}
