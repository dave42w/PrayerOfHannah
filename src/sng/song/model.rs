// Prayer of Hannah
// Free Software to provide Slides as a web service for Worship, Noticeboards
// and more. Named in honour of Hannah (see 1 Samuel 2:1-10) and particularly
// from verse 8:
//"He raises up the poor from the dust; he lifts the needy from the ash heap"
// Copyright (C) 2023  Dave Warnock dwarnock@gmail.com

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

// Source code at https://codeberg.org/Dave42W/PrayerOfHannah

use sqlx::{self, sqlite::SqliteQueryResult, Error, Pool, Sqlite};

use super::{Song, Songs};
use crate::sng::song_collection;

pub async fn exists(pool: &Pool<Sqlite>, song_collection_id: &str, song_title: &str) -> bool {
    sqlx::query!(
        "SELECT id from Song where song_collection_id = ?1 AND song_title = ?2",
        song_collection_id,
        song_title
    )
    .fetch_optional(pool)
    .await
    .unwrap_or_default()
    .is_some()
}

pub async fn insert(
    pool: &Pool<Sqlite>,
    song_collection_id: &str,
    song_number: i64,
    song_title: &str,
) -> Result<(), Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO Song 
        (id, song_collection_id, song_number, song_title, created_timestamp, updated_timestamp) 
        VALUES
        (?1, ?2, ?3, ?4, ?5, $6)
        "#,
        id,
        song_collection_id,
        song_number,
        song_title,
        now,
        now
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update(
    pool: &Pool<Sqlite>,
    id: &str,
    song_collection_id: &str,
    song_number: i64,
    song_title: &str,
) -> Result<(), Error> {
    let now = chrono::Utc::now();
    sqlx::query!(
        r#"
        UPDATE Song
        SET song_collection_id = ?2,
            song_number = ?3,
            song_title = ?4,
            updated_timestamp = ?5
        WHERE 
            id = ?1
        "#,
        id,
        song_collection_id,
        song_number,
        song_title,
        now
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn save(
    pool: &Pool<Sqlite>,
    id: &str,
    song_collection_id: &str,
    song_number: i64,
    song_title: &str,
) -> Result<(), Error> {
    //let surl = url.unwrap_or("".to_string()).to_string();

    if id.is_empty() {
        insert(pool, song_collection_id, song_number, song_title).await
    } else {
        update(pool, id, song_collection_id, song_number, song_title).await
    }
}

pub async fn insert_after_check(
    pool: &Pool<Sqlite>,
    collection_code: &str,
    song_number: i32,
    song_title: &str,
) -> Result<(), Error> {
    let song_collection = song_collection::model::select_by_code(pool, collection_code).await;

    if !exists(pool, &song_collection.id, song_title).await {
        insert(pool, &song_collection.id, song_number.into(), song_title).await?;
    }
    Ok(())
}

pub async fn list_all(pool: &Pool<Sqlite>) -> Songs {
    Songs {
        songs: sqlx::query_as!(
            Song,
            "SELECT id, song_collection_id, song_number, song_title from Song ORDER BY song_title"
        )
        .fetch_all(pool)
        .await
        .unwrap_or_default(),
    }
}

pub async fn select_by_id(pool: &Pool<Sqlite>, id: &str) -> Song {
    sqlx::query_as!(
        Song,
        "SELECT id, song_collection_id, song_number, song_title from Song where id = ?1",
        id
    )
    .fetch_one(pool)
    .await
    .unwrap_or_default()
}

pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<bool, sqlx::Error> {
    let res: Result<SqliteQueryResult, sqlx::Error> =
        sqlx::query!("DELETE from Song where id = ?1", id)
            .execute(pool)
            .await;
    match res {
        Ok(r) => {
            if r.rows_affected() == 0 {
                Err(sqlx::Error::RowNotFound)
            } else {
                Ok(true)
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn seed_db(pool: &Pool<Sqlite>) -> Result<(), Error> {
    insert_after_check(pool, "StF", 202, "Hark! The herald-angels sing").await?;
    print!("^");

    insert_after_check(pool, "H&P", 106, "Hark! The herald-angels sing").await?;
    print!("^");

    insert_after_check(pool, "StF", 5, "Father, in whom we live").await?;
    print!("^");

    insert_after_check(pool, "StF", 671, "What shall we offer our good Lord").await?;
    print!("^");

    insert_after_check(pool, "StF", 49, "God beyond all names").await?;
    print!("^");

    insert_after_check(pool, "StF", 101, "Before the world began").await?;
    print!("^");

    insert_after_check(pool, "StF", 1, "All people that on earth do dwell").await?;
    print!("^");

    Ok(())
}
