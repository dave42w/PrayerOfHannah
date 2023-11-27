CREATE TABLE IF NOT EXISTS User (
    id TEXT PRIMARY KEY,
    user_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    email TEXT NOT NULL,
    mobile_phone TEXT NOT NULL,
    created_timestamp TEXT NOT NULL,
    created_by_user_id TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    updated_by_user_id TEXT NOT NULL,
    UNIQUE (user_name),
    UNIQUE (display_name)
) STRICT, WITHOUT ROWID;


CREATE INDEX IF NOT EXISTS idxUserEmail ON User (
    email
);

CREATE INDEX IF NOT EXISTS idxUserMobilePhone ON User (
    mobile_phone
);
