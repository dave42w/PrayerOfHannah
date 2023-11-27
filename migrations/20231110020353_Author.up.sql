CREATE TABLE IF NOT EXISTS Author (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    first_name TEXT NOT NULL,
    surname TEXT NOT NULL,
    display_name TEXT NOT NULL,
    created_timestamp TEXT NOT NULL,
    created_by_user_id TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    updated_by_user_id TEXT NOT NULL,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id) REFERENCES Tenant(id),
    UNIQUE (tenant_id, display_name)
) STRICT, WITHOUT ROWID;


CREATE INDEX IF NOT EXISTS idxAuthorName ON Author (
    tenant_id,
    surname,
    first_name,
    display_name
);
