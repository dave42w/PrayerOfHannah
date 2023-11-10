CREATE TABLE IF NOT EXISTS SongCollection (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    url TEXT,
    created_timestamp TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    UNIQUE (code),
    UNIQUE (name)
) STRICT, WITHOUT ROWID;
