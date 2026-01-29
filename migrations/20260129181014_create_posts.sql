CREATE TABLE posts(
    id              uuid PRIMARY KEY,
    title           TEXT NOT NULL,
    content         TEXT NOT NULL,
    author_id       uuid NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL,
    updated_at      TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);
