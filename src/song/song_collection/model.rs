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

use sqlx::{self, Pool, Sqlite, Error, sqlite::SqliteQueryResult};

use crate::song::song_collection::SongCollection;

use super::SongCollections;


pub async fn exists(pool: &Pool<Sqlite>, code: &str) -> bool {
    sqlx::query!("SELECT id from SongCollection where code = ?1", code).fetch_optional(pool).await.unwrap_or_default().is_some()
}

pub async fn insert(pool: &Pool<Sqlite>, code: &str, name: &str, url: &str)  -> Result<(), Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO SongCollection 
        (id, code, name, url, created_timestamp, updated_timestamp) 
        VALUES
        (?1, ?2, ?3, ?4, ?5, ?6)
        "#, 
        id, code, name, url, now, now)
    .execute(pool)    
    .await?;
    Ok(())
}

pub async fn update(pool: &Pool<Sqlite>, id: &str, code: &str, name: &str, url: &str) -> Result<(), Error> {
    let now = chrono::Utc::now();
    sqlx::query!(
        r#"
        UPDATE SongCollection 
        SET code = ?2,
            name = ?3,
            url = ?4,
            updated_timestamp = ?5
        WHERE 
            id = ?1
        "#, 
        id, code, name, url, now)
    .execute(pool)    
    .await?;
    Ok(())
}

pub async fn save(pool: &Pool<Sqlite>, id: &str, code: &str, name: &str, url: &Option<String>) -> Result<(), Error> {
    let mut surl = "".to_string();
    if (&url).is_some() { surl = url.clone().unwrap() };

    if id.is_empty() {
        return insert(&pool, &code, &name, &surl).await;
    } else {
        return update(&pool, &id, &code, &name, &surl).await;
    }
}

pub async fn insert_after_check(pool: &Pool<Sqlite>, code: &str, name: &str, url: &str) -> Result<(), Error> {
    if !exists(pool, code).await {
        insert(pool, code, name,url).await?;
    }
    Ok(())
}

// pub async fn select_id(pool: &Pool<Sqlite>, code: &str) -> Result<String, Error> {
//     let record = sqlx::query!("SELECT id from SongCollection where code = ?1", code).fetch_optional(pool).await?;
//     match record {
//         Some(r) => {
//             Ok(r.id.into())
//         }
//         None => {
//             Err(sqlx::Error::RowNotFound)
//         }
//     }
// }

pub async fn list_all(pool: &Pool<Sqlite>) -> SongCollections {
    SongCollections{song_collection: sqlx::query_as!(SongCollection, "SELECT id, code, name, url from SongCollection ORDER BY name").fetch_all(pool).await.unwrap_or_default()}
}

pub async fn select_by_id (pool: &Pool<Sqlite>, id: &str) -> SongCollection {
    sqlx::query_as!(SongCollection, "SELECT id, code, name, url from SongCollection where id = ?1", id).fetch_one(pool).await.unwrap_or_default()
}

pub async fn delete (pool: &Pool<Sqlite>, id: &str) -> Result<bool, sqlx::Error> {
    let res: Result<SqliteQueryResult, sqlx::Error> = sqlx::query!("DELETE from SongCollection where id = ?1", id).execute(pool).await;
    match res {
        Ok(r) => {
            if r.rows_affected() == 0 {
                Err(sqlx::Error::RowNotFound)
            } else {
                Ok(true)                
            }
        },
        Err(e) => Err(e),
    }
}

pub async fn seed_db(pool: &Pool<Sqlite>) -> Result<(), Error> {
    insert_after_check(pool, 
        "StF", 
        "Singing the Faith", 
        "https://www.methodist.org.uk/our-faith/worship/singing-the-faith-plus/"
    )
    .await?;
    print!(".");

    insert_after_check(pool, 
        "H&P",
        "Hymns & Psalms",
        ""
    )
    .await?;
    print!(".");

    insert_after_check(pool, 
        "SoF1",
        "Songs of Fellowship book 1",
        ""
    )
    .await?;
    print!(".");

    Ok(())
}

