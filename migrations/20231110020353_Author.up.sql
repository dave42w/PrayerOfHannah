CREATE TABLE IF NOT EXISTS Author (
    id TEXT PRIMARY KEY,
    first_name TEXT NOT NULL,
    surname TEXT NOT NULL,
    display_name TEXT NOT NULL,
    created_timestamp TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    UNIQUE (display_name)
) STRICT, WITHOUT ROWID;


CREATE INDEX IF NOT EXISTS idxAuthorName ON Author (
    surname,
    first_name,
    display_name
);
