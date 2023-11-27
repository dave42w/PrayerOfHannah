CREATE TABLE IF NOT EXISTS SongAuthor (
    tenant_id TEXT NOT NULL,
    song_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    role TEXT,
    created_timestamp TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    PRIMARY KEY (tenant_id, song_id, author_id),
    FOREIGN KEY (tenant_id, song_id) REFERENCES Song(tenant_id, id),
    FOREIGN KEY (tenant_id, author_id) REFERENCES Author(tenant_id, id)
) STRICT, WITHOUT ROWID;

CREATE UNIQUE INDEX IF NOT EXISTS idxAuthorSong ON SongAuthor (
    tenant_id,
    author_id,
    song_id
);
