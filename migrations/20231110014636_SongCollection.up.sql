CREATE TABLE IF NOT EXISTS SongCollection (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    url TEXT,
    created_timestamp TEXT NOT NULL,
    create_by_user_id TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    updated_by_user_id TEXT NOT NULL,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id) REFERENCES Tenant(id),

    UNIQUE (tenant_id, code),
    UNIQUE (tenant_id, name)
) STRICT, WITHOUT ROWID;
