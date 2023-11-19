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

use sqlx::{self, sqlite::SqliteQueryResult, Error, Pool, Sqlite};

use super::Author;

use super::Authors;

pub async fn exists(pool: &Pool<Sqlite>, display_name: &str) -> bool {
    sqlx::query!(
        "SELECT id from Author where display_name = ?1",
        display_name
    )
    .fetch_optional(pool)
    .await
    .unwrap_or_default()
    .is_some()
}

pub async fn insert(
    pool: &Pool<Sqlite>,
    first_name: &str,
    surname: &str,
    display_name: &str,
) -> Result<(), Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO Author 
        (id, first_name, surname, display_name, created_timestamp, updated_timestamp) 
        VALUES
        (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        id,
        first_name,
        surname,
        display_name,
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
    first_name: &str,
    surname: &str,
    display_name: &str,
) -> Result<(), Error> {
    let now = chrono::Utc::now();
    sqlx::query!(
        r#"
        UPDATE Author 
        SET first_name = ?2,
            surname = ?3,
            display_name = ?4,
            updated_timestamp = ?5
        WHERE 
            id = ?1
        "#,
        id,
        first_name,
        surname,
        display_name,
        now
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn save(
    pool: &Pool<Sqlite>,
    id: &str,
    first_name: &str,
    surname: &str,
    display_name: &str,
) -> Result<(), Error> {
    if id.is_empty() {
        insert(pool, first_name, surname, display_name).await
    } else {
        update(pool, id, first_name, surname, display_name).await
    }
}

pub async fn insert_after_check(
    pool: &Pool<Sqlite>,
    first_name: &str,
    surname: &str,
    display_name: &str,
) -> Result<(), Error> {
    if !exists(pool, display_name).await {
        insert(pool, first_name, surname, display_name).await?;
    }
    Ok(())
}

pub async fn list_all(pool: &Pool<Sqlite>) -> Authors {
    Authors {
        authors: sqlx::query_as!(
            Author,
            "SELECT id, first_name, surname, display_name from Author ORDER BY display_name"
        )
        .fetch_all(pool)
        .await
        .unwrap_or_default(),
    }
}

pub async fn select_by_id(pool: &Pool<Sqlite>, id: &str) -> Author {
    sqlx::query_as!(
        Author,
        "SELECT id, first_name, surname, display_name from Author where id = ?1",
        id
    )
    .fetch_one(pool)
    .await
    .unwrap_or_default()
}

pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<bool, sqlx::Error> {
    let res: Result<SqliteQueryResult, sqlx::Error> =
        sqlx::query!("DELETE from Author where id = ?1", id)
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
    insert_after_check(pool, "Charles", "Wesley", "Charles Wesley").await?;
    print!(",");

    insert_after_check(pool, "John", "Wesley", "John Wesley").await?;
    print!(",");

    insert_after_check(pool, "John", "Bell", "John L. Bell").await?;
    print!(",");

    insert_after_check(pool, "Graham", "Maule", "Graham Maule").await?;
    print!(",");

    Ok(())
}
