use super::context::Context;
use chrono::{DateTime, Local};
use juniper::{graphql_object, FieldError, FieldResult};
use uuid::Uuid;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_new_entry(context: &Context, base64_image: String) -> FieldResult<String> {
        let created_at: DateTime<Local> = Local::now();
        let entry_id = Uuid::new_v4();

        // Validate the provided base64_image by attempting to decode it
        // Proceed only if decoding is successful

        // Not an exhaustive way to validate but....eh
        match base64::decode(&base64_image) {
            Ok(_) => {
                db_client::photo_entries::create_new_photo_entry(
                    &mut context.0.conn().await?,
                    entry_id,
                    created_at,
                    base64_image,
                )
                .await?;
                Ok(entry_id.to_string())
            }
            Err(err) => FieldResult::Err(FieldError::from(err)),
        }
    }

    async fn delete_entry(context: &Context, entry_id: String) -> FieldResult<bool> {
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

        Ok(true)
    }
}
