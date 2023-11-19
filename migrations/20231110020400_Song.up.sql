CREATE TABLE IF NOT EXISTS Song (
    id TEXT PRIMARY KEY NOT NULL,
    song_collection_id TEXT NOT NULL,
    song_number INTEGER NOT NULL,
    song_title TEXT NOT NULL,
    created_timestamp TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    UNIQUE (song_collection_id, song_title),
    FOREIGN KEY (song_collection_id) REFERENCES SongCollection(id)
) STRICT, WITHOUT ROWID;

CREATE INDEX IF NOT EXISTS idxSongTitle ON Song (
    song_title,
    song_collection_id
);

CREATE INDEX IF NOT EXISTS idxSongNumber ON Song (
    song_collection_id,
    song_number
);
