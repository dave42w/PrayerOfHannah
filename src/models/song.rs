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

pub async fn insert(txn: &mut Transaction<'_, Sqlite>, song_collection_id: &str, song_number: i32, song_title: &str) -> Result<(), Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO Song 
        (id, song_collection_id, song_number, song_title, created_timestamp, updated_timestamp) 
        VALUES
        (?1, ?2, ?3, ?4, ?5, $6)
        "#, id, song_collection_id, song_number, song_title, now, now)
    .execute(&mut **txn)    
    .await?;
    Ok(())
}

pub async fn exists(txn: &mut Transaction<'_, Sqlite>, song_collection_id: &str, song_title: &str) -> bool {
    sqlx::query!("SELECT id from Song where song_collection_id = ?1 AND song_title = ?2", song_collection_id, song_title).fetch_optional(&mut **txn).await.unwrap_or_default().is_some()
}

pub async fn select_id(txn: &mut Transaction<'_, Sqlite>, song_collection_id: &str, song_number: i32) -> Result<String, Error> {
    let record = sqlx::query!("SELECT id from Song where song_collection_id = ?1 AND song_number = ?2", song_collection_id, song_number).fetch_optional(&mut **txn).await?;
    match record {
        Some(r) => {
            Ok(r.id.into())
        }
        None => {
            Err(sqlx::Error::RowNotFound)
        }
    }
}

pub async fn insert_after_check(txn: &mut Transaction<'_, Sqlite>, collection_code: &str, song_number: i32, song_title: &str) -> Result<(), Error> {
    let song_collection_id = song_collection::select_id(txn, collection_code).await?;

    if !exists(txn, &song_collection_id, song_title).await {
        insert(txn, &song_collection_id, song_number, song_title).await?;
    }
    Ok(())
}


