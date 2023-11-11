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

pub async fn exists(txn: &mut Transaction<'_, Sqlite>, display_name: &str) -> bool {
    sqlx::query!("SELECT id from Author where display_name = ?1", display_name).fetch_optional(&mut **txn).await.unwrap_or_default().is_some()
}

pub async fn insert(txn: &mut Transaction<'_, Sqlite>, first_name: &str, surname: &str, display_name: &str)  -> Result<(), Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO Author 
        (id, first_name, surname, display_name, created_timestamp, updated_timestamp) 
        VALUES
        (?1, ?2, ?3, ?4, ?5, ?6)
        "#, id, first_name, surname, display_name, now, now)
    .execute(&mut **txn)    
    .await?;
    Ok(())
}

pub async fn insert_after_check(txn: &mut Transaction<'_, Sqlite>, first_name: &str, surname: &str, display_name: &str) -> Result<(), Error> {
    if !exists(txn, display_name).await {
        insert(txn, first_name, surname, display_name).await?;
    }
    Ok(())
}

pub async fn select_id(txn: &mut Transaction<'_, Sqlite>, display_name: &str) -> Result<String, Error> {
    let record = sqlx::query!("SELECT id from Author where display_name = ?1", display_name).fetch_optional(&mut **txn).await?;
    match record {
        Some(r) => {
            Ok(r.id.into())
        }
        None => {
            Err(sqlx::Error::RowNotFound)
        }
    }
}

