CREATE TABLE photo_entries(
    entry_id uuid PRIMARY KEY,
    created_at timestamptz NOT NULL,
    base64_image TEXT NOT NULL
)