use super::context::Context;
use crate::domain::PhotoEntry;
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn get_entries(context: &Context) -> FieldResult<Vec<PhotoEntry>> {
        let entries = db_client::photo_entries::get_photo_entries(&mut context.0.conn().await?)
            .await?
            .iter()
            .map(|entry| PhotoEntry {
                entry_id: entry.entry_id.to_string(),
                created_at: entry.created_at.to_string(),
                base64_image: entry.base64_image.to_owned(),
            })
            .collect();

        Ok(entries)
    }

    async fn get_entry(entry_id: String) -> FieldResult<PhotoEntry> {
        Ok(PhotoEntry {
            entry_id: entry_id,
            created_at: "".to_string(),
            base64_image: test_utils::sample_base64_image(),
        })
    }
}
