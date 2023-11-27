CREATE TABLE IF NOT EXISTS Song (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    song_collection_id TEXT NOT NULL,
    song_number INTEGER NOT NULL,
    song_title TEXT NOT NULL,
    created_timestamp TEXT NOT NULL,
    created_by_user_id TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    updated_by_user_id TEXT NOT NULL,
    PRIMARY KEY (tenant_id, id),
    UNIQUE (tenant_id, song_collection_id, song_title),
    FOREIGN KEY (tenant_id, song_collection_id) REFERENCES SongCollection(tenant_id, id)
) STRICT, WITHOUT ROWID;

CREATE INDEX IF NOT EXISTS idxSongTitle ON Song (
    tenant_id,
    song_title,
    song_collection_id
);

CREATE INDEX IF NOT EXISTS idxSongNumber ON Song (
    tenant_id,
    song_collection_id,
    song_number
);
