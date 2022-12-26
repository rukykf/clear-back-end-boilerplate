use super::context::Context;
use chrono::{DateTime, Local};
use juniper::{graphql_object, FieldError, FieldResult};
use uuid::Uuid;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_new_entry(
        context: &Context,
        base64_image: String,
        auth_token: String,
    ) -> FieldResult<String> {
        let created_at: DateTime<Local> = Local::now();
        let entry_id = Uuid::new_v4();

        // Get the user associated with the provided auth token
        let user_id = context.get_user_id_for_auth_token(auth_token);

        // Validate the provided base64_image by attempting to decode it
        // Proceed only if base64 decoding is successful + we have a valid user id for the token
        match (base64::decode(&base64_image), user_id) {
            (Ok(_), Ok(user_id)) => {
                db_client::photo_entries::create_new_photo_entry(
                    &mut context.0.conn().await?,
                    entry_id,
                    created_at,
                    base64_image,
                    user_id,
                )
                .await?;
                Ok(entry_id.to_string())
            }
            _ => FieldResult::Err(FieldError::from("Invalid Input".to_string())),
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

    async fn get_auth_token(_username: String, _password: String) -> FieldResult<String> {
        Ok(test_utils::sample_auth_token())
    }

    async fn refresh_auth_token(_auth_token: String) -> FieldResult<String> {
        // TODO
        FieldResult::Err(FieldError::from("Not Implemented".to_string()))
    }

    async fn reset_password() -> FieldResult<String> {
        // TODO
        FieldResult::Err(FieldError::from("Not Implemented".to_string()))
    }
}
