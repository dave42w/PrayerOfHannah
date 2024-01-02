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

use sqlx::{self, Error, Pool, Sqlite};

use super::{UserTenant, UserTenants};

pub async fn list_out(pool: &Pool<Sqlite>, id: &str) -> UserTenants {
    UserTenants {
        user_tenants: sqlx::query_as!(
            UserTenant,
            "SELECT id, name from Tenant WHERE NOT EXISTS (SELECT 1 FROM UserTenant WHERE user_id \
             = ?1 AND tenant_id = id) ORDER BY name",
            id
        )
        .fetch_all(pool)
        .await
        .unwrap_or_default(),
    }
}

pub async fn list_in(pool: &Pool<Sqlite>, id: &str) -> UserTenants {
    UserTenants {
        user_tenants: sqlx::query_as!(
            UserTenant,
            "SELECT id, name from Tenant WHERE EXISTS (SELECT 1 FROM UserTenant WHERE user_id = \
             ?1 AND tenant_id = id) ORDER BY name",
            id
        )
        .fetch_all(pool)
        .await
        .unwrap_or_default(),
    }
}

pub async fn add(pool: &Pool<Sqlite>, user_id: &str, tenant_id: &str) -> Result<(), Error> {
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO UserTenant 
        (user_id, tenant_id, created_timestamp, updated_timestamp) 
        VALUES
        (?1, ?2, ?3, ?4)
        "#,
        user_id,
        tenant_id,
        now,
        now
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete(pool: &Pool<Sqlite>, user_id: &str, tenant_id: &str) -> Result<(), Error> {
    sqlx::query!(
        r#"
        DELETE FROM UserTenant 
        WHERE user_id = ?1 AND tenant_id = ?2"#,
        user_id,
        tenant_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
