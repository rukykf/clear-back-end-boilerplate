use super::context::Context;
use crate::domain::PhotoEntry;
use juniper::graphql_object;

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn get_entries(context: &Context) -> Vec<PhotoEntry> {
        let entries =
            sqlx::query!(r#"SELECT entry_id, created_at, base64_image FROM photo_entries"#)
                .fetch_all(&mut context.0.conn().await.unwrap())
                .await
                .unwrap()
                .iter()
                .map(|row| PhotoEntry {
                    entry_id: row.entry_id.to_string(),
                    created_at: row.created_at.to_string(),
                    base64_image: row.base64_image.to_owned(),
                })
                .collect();

        entries
    }

    async fn get_entry(context: &Context, entry_id: String) -> PhotoEntry {
        PhotoEntry {
            entry_id: entry_id,
            created_at: "".to_string(),
            base64_image: "".to_string(),
        }
    }

    async fn hello(context: &Context, number: i32, word: String) -> String {
        std::thread::sleep(std::time::Duration::from_secs(15));
        format!("my word")
    }
}
