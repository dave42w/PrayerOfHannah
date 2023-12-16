CREATE TABLE IF NOT EXISTS UserTenant (
    user_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    created_timestamp TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    PRIMARY KEY (user_id, tenant_id),
    FOREIGN KEY (user_id) REFERENCES User(id),
    FOREIGN KEY (tenant_id) REFERENCES Tenant(id)
) STRICT, WITHOUT ROWID;

CREATE UNIQUE INDEX IF NOT EXISTS idxTenantUser ON UserTenant (
    tenant_id,
    user_id
);
