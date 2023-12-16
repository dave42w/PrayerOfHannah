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

use super::{User, Users};

pub async fn exists(pool: &Pool<Sqlite>, user_name: &str) -> bool {
    sqlx::query!("SELECT id from User where user_name = ?1", user_name)
        .fetch_optional(pool)
        .await
        .unwrap_or_default()
        .is_some()
}

pub async fn insert(
    pool: &Pool<Sqlite>,
    user_name: &str,
    display_name: &str,
    is_admin: i64,
    email: &str,
    mobile_phone: &str,
) -> Result<(), Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    let p = "".to_string();

    sqlx::query!(
        r#"
        INSERT INTO User 
        (id, user_name, hash_password, display_name, is_admin, email, mobile_phone, created_timestamp, updated_timestamp) 
        VALUES
        (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
        id,
        user_name,
        p,
        display_name,
        is_admin,
        email,
        mobile_phone,
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
    user_name: &str,
    display_name: &str,
    is_admin: i64,
    email: &str,
    mobile_phone: &str,
) -> Result<(), Error> {
    let now = chrono::Utc::now();
    sqlx::query!(
        r#"
        UPDATE User 
        SET user_name = ?2,
            display_name = ?3,
            is_admin = ?4,
            email = ?5,
            mobile_phone = ?6,
            updated_timestamp = ?7
        WHERE 
            id = ?1
        "#,
        id,
        user_name,
        display_name,
        is_admin,
        email,
        mobile_phone,
        now
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn save(
    pool: &Pool<Sqlite>,
    id: &str,
    user_name: &str,
    display_name: &str,
    is_admin: i64,
    email: &str,
    mobile_phone: &str,
) -> Result<(), Error> {
    if id.is_empty() {
        insert(pool, user_name, display_name, is_admin, email, mobile_phone).await
    } else {
        update(pool, id, user_name, display_name, is_admin, email, mobile_phone).await
    }
}

pub async fn insert_after_check(
    pool: &Pool<Sqlite>,
    user_name: &str,
    display_name: &str,
    is_admin: i64,
    email: &str,
    mobile_phone: &str,
) -> Result<(), Error> {
    if !exists(pool, user_name).await {
        insert(pool, user_name, display_name, is_admin, email, mobile_phone).await?;
    }
    Ok(())
}

pub async fn list_all(pool: &Pool<Sqlite>) -> Users {
    Users {
        users: sqlx::query_as!(
            User,
            "SELECT id, user_name, display_name, is_admin, email, mobile_phone from User ORDER BY \
             display_name"
        )
        .fetch_all(pool)
        .await
        .unwrap_or_default(),
    }
}

pub async fn select_by_id(pool: &Pool<Sqlite>, id: &str) -> User {
    sqlx::query_as!(
        User,
        "SELECT id, user_name, display_name, is_admin, email, mobile_phone from User where id = ?1",
        id
    )
    .fetch_one(pool)
    .await
    .unwrap_or_default()
}

pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<bool, sqlx::Error> {
    let res: Result<SqliteQueryResult, sqlx::Error> =
        sqlx::query!("DELETE from User where id = ?1", id)
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
    insert_after_check(
        pool,
        "dave42w",
        "Dave Warnock",
        0,
        "dwarnock@gmail.com",
        "+447886553376",
    )
    .await?;
    print!("!");

    insert_after_check(
        pool,
        "dw",
        "Dave (Methodist) Warnock",
        1,
        "dave.warnock@methodist.org.uk",
        "+447886553376",
    )
    .await?;
    print!("!");

    Ok(())
}
