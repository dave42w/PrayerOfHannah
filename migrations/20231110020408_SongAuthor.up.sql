CREATE TABLE IF NOT EXISTS SongAuthor (
    song_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    role TEXT,
    created_timestamp TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    PRIMARY KEY (song_id, author_id),
    FOREIGN KEY (song_id) REFERENCES Song(id),
    FOREIGN KEY (author_id) REFERENCES Author(id)
) STRICT, WITHOUT ROWID;

CREATE UNIQUE INDEX IF NOT EXISTS idxAuthorSong ON SongAuthor (
    author_id,
    song_id
);
