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

pub(crate) mod model;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::{controllers::render_into_response, utils::AppState};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserTenant {
    pub id: String,
    pub name: String,
}

impl Default for UserTenant {
    fn default() -> UserTenant {
        UserTenant {
            id: "".to_string(),
            name: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserTenants {
    user_tenants: Vec<UserTenant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageUserTenants {
    user_id: String,
    in_tenants: UserTenants,
    out_tenants: UserTenants,
}

pub fn create_routes() -> Router<AppState<'static>> {
    Router::new()
        //.route("/", get(list))
        //.route("/", get(list_tenants))
        .route("/:id", get(display))
        .route("/in/:uid/:tid", get(add))
        .route("/out/:uid/:tid", get(delete))
    //.route("/add", get(add))
    //.route("/edit/:id", get(edit))
    //.route("/save", post(save))
    //.route("/delete/:id", post(delete))
    //.route("/password/:id", get(get_password))
    //.route("/password/:id", post(set_password))
}

pub async fn display(state: State<AppState<'_>>, path_user_id: Path<String>) -> impl IntoResponse {
    let user_id = path_user_id.to_string();
    let in_tenants = model::list_in(&state.pool, &user_id).await;
    let out_tenants = model::list_out(&state.pool, &user_id).await;
    let page = PageUserTenants {
        user_id,
        in_tenants,
        out_tenants,
    };
    render_into_response(state, "admin/user_tenant/list_tenants.html", &page)
}

pub async fn add(
    state: State<AppState<'_>>,
    Path((user_id, tenant_id)): Path<(String, String)>,
) -> impl IntoResponse {
    //let user_id = Path((user_id, team_id)): Path<(Uuid,
    // Uuid)>path[0].to_string(); let tenant_id = path[1].to_string();
    model::add(&state.pool, &user_id, &tenant_id).await.unwrap();
    let _r = "/Admin/UserTenant/".to_string() + &user_id;
    display(state, Path(user_id)).await
}

pub async fn delete(
    state: State<AppState<'_>>,
    Path((user_id, tenant_id)): Path<(String, String)>,
) -> impl IntoResponse {
    //let user_id = Path((user_id, team_id)): Path<(Uuid,
    // Uuid)>path[0].to_string(); let tenant_id = path[1].to_string();
    model::delete(&state.pool, &user_id, &tenant_id)
        .await
        .unwrap();
    display(state, Path(user_id)).await
}
