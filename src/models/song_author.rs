// Prayer of Hannah
// Free Software to provide Slides as a web service for Worship, Noticeboards and more. 
// Named in honour of Hannah (see 1 Samuel 2:1-10) and particularly from verse 8: 
//"He raises up the poor from the dust; he lifts the needy from the ash heap" 
// Copyright (C) 2023  Dave Warnock dwarnock@gmail.com

// This program is free software: you can redistribute it and/or modify it under the terms
// of the GNU Affero General Public License as published by the Free Software Foundation, 
// either version 3 of the License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  
// See the GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License along with this program.  
// If not, see <http://www.gnu.org/licenses/>.

// Source code at https://codeberg.org/Dave42W/PrayerOfHannah


use sqlx::{self, Transaction, Sqlite, Error};

use crate::models::song_collection;
use crate::models::song;
use crate::models::author;

pub async fn song_author_exists(txn: &mut Transaction<'_, Sqlite>, song_id: &str, author_id: &str) -> bool {
    sqlx::query!("SELECT song_id from SongAuthor where song_id = ?1 AND author_id = ?2", song_id, author_id).fetch_optional(&mut **txn).await.unwrap_or_default().is_some()
}

pub async fn insert(txn: &mut Transaction<'_, Sqlite>, song_id: &str, author_id: &str)  -> Result<(), Error> {
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO SongAuthor
        (song_id, author_id, created_timestamp, updated_timestamp) 
        VALUES
        (?1, ?2, ?3, ?4)
        "#, song_id, author_id, now, now)
    .execute(&mut **txn)    
    .await?;
    Ok(())
}

pub async fn insert_after_check(txn: &mut Transaction<'_, Sqlite>, collection_code: &str, song_number: i32, display_name: &str)  -> Result<(), Error> {
    let song_collection_id = song_collection::select_id(txn, collection_code).await?;
    let song_id = song::select_song_id(txn, &song_collection_id, song_number).await?;
    let author_id = author::select_author_id(txn, &display_name).await?;

    if !song_author_exists(txn, &song_id, &author_id).await {
        insert(txn, &song_id, &author_id).await?;
    }
    Ok(())
}
