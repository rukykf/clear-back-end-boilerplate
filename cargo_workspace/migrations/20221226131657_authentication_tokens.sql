CREATE TABLE authentication_tokens(
    user_id uuid NOT NULL REFERENCES users (user_id),
    token TEXT NOT NULL,
    expires_at timestamptz NOT NULL
)