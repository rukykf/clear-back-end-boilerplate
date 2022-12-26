use chrono::{DateTime, Local};
use juniper::graphql_value;
use server::domain::PhotoEntry;

use crate::helpers::App;
use sqlx::{Connection, Executor, PgConnection};
use uuid::Uuid;

#[tokio::test]
async fn returns_previously_saved_entries() {
    let app = App::new().await;

    // Insert one entry into the DB
    let id = Uuid::new_v4();
    let date_time = Local::now();
    let base64_image = test_utils::sample_base64_image();
    let id_string = id.to_string();
    let date_time_string = date_time.to_string();

    sqlx::query!(
        r#"INSERT INTO photo_entries (entry_id, created_at, base64_image) Values ($1, $2, $3)"#,
        id,
        date_time,
        base64_image
    )
    .execute(&mut app.get_db_conn().await)
    .await
    .unwrap();

    // Attempt to retrieve the entry from the API and assert on it
    let query = "query { getEntries { entryId, base64Image } }";
    let (res, _errors) = app
        .execute_query(query)
        .await
        .expect("Something is wrong with the provided photo entries query");

    let expected_response: juniper::Value =
        graphql_value!({ "getEntries": [{ "entryId": id_string, "base64Image": base64_image}] });

    assert_eq!(res, expected_response);
}

#[tokio::test]
async fn successfully_creates_new_entry() {
    let app = App::new().await;

    // Assert that there are currently no entries in the DB
    let entries: Vec<PhotoEntry> = sqlx::query!(r#"SELECT entry_id FROM photo_entries"#)
        .fetch_all(&mut app.get_db_conn().await)
        .await
        .unwrap()
        .iter()
        .map(|row| PhotoEntry {
            entry_id: row.entry_id.to_string(),
            created_at: "".to_string(),
            base64_image: "".to_string(),
        })
        .collect();

    assert_eq!(entries.len(), 0);

    // Execute a graphql mutation to create a new entry
    let mutation = format!(
        "mutation {{ createNewEntry(base64Image: \"{}\") }}",
        test_utils::sample_base64_image()
    );

    app.execute_query(mutation.as_str())
        .await
        .expect("Something is wrong with the create entry mutation");

    // Attempt to retrieve the newly created record and assert on it
    let entries: Vec<PhotoEntry> =
        sqlx::query!(r#"SELECT entry_id, created_at, base64_image FROM photo_entries"#)
            .fetch_all(&mut app.get_db_conn().await)
            .await
            .unwrap()
            .iter()
            .map(|row| PhotoEntry {
                entry_id: row.entry_id.to_string(),
                created_at: row.created_at.to_string(),
                base64_image: row.base64_image.to_owned(),
            })
            .collect();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].base64_image, test_utils::sample_base64_image());
}

#[tokio::test]
async fn returns_error_when_attempting_to_create_new_entry_with_invalid_base64() {}
