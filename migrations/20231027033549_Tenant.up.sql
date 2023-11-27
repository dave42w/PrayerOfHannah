CREATE TABLE IF NOT EXISTS Tenant (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_timestamp TEXT NOT NULL,
    created_by_user_id TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    updated_by_user_id TEXT NOT NULL,
    UNIQUE (name)
) STRICT, WITHOUT ROWID;
