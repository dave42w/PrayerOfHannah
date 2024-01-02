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

use super::{Tenant, Tenants};

pub async fn exists(pool: &Pool<Sqlite>, name: &str) -> bool {
    sqlx::query!("SELECT id from Tenant where name = ?1", name)
        .fetch_optional(pool)
        .await
        .unwrap_or_default()
        .is_some()
}

pub async fn insert(pool: &Pool<Sqlite>, name: &str) -> Result<(), Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO Tenant 
        (id, name, created_timestamp, updated_timestamp) 
        VALUES
        (?1, ?2, ?3, ?4)
        "#,
        id,
        name,
        now,
        now
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update(pool: &Pool<Sqlite>, id: &str, name: &str) -> Result<(), Error> {
    let now = chrono::Utc::now();
    sqlx::query!(
        r#"
        UPDATE Tenant 
        SET name = ?2,
            updated_timestamp = ?3
        WHERE 
            id = ?1
        "#,
        id,
        name,
        now
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn save(pool: &Pool<Sqlite>, id: &str, name: &str) -> Result<(), Error> {
    if id.is_empty() {
        insert(pool, name).await
    } else {
        update(pool, id, name).await
    }
}

pub async fn insert_after_check(pool: &Pool<Sqlite>, name: &str) -> Result<(), Error> {
    if !exists(pool, name).await {
        insert(pool, name).await?;
    }
    Ok(())
}

pub async fn list_all(pool: &Pool<Sqlite>) -> Tenants {
    Tenants {
        tenants: sqlx::query_as!(Tenant, "SELECT id, name from Tenant ORDER BY name")
            .fetch_all(pool)
            .await
            .unwrap_or_default(),
    }
}

pub async fn select_by_id(pool: &Pool<Sqlite>, id: &str) -> Tenant {
    sqlx::query_as!(Tenant, "SELECT id, name from Tenant where id = ?1", id)
        .fetch_one(pool)
        .await
        .unwrap_or_default()
}

pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<bool, sqlx::Error> {
    let res: Result<SqliteQueryResult, sqlx::Error> =
        sqlx::query!("DELETE from Tenant where id = ?1", id)
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
    insert_after_check(pool, "StAndrews").await?;
    print!("!");

    insert_after_check(pool, "BrownleyGreen").await?;
    print!("!");

    insert_after_check(pool, "LawtonMoor").await?;
    print!("!");

    Ok(())
}
