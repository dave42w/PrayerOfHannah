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

use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::{Deserialize, Serialize};

use crate::{controllers::render_into_response, utils::AppState};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub display_name: String,
    pub is_admin: i64,
    pub email: String,
    pub mobile_phone: String,
}

impl Default for User {
    fn default() -> User {
        User {
            id: "".to_string(),
            user_name: "".to_string(),
            display_name: "".to_string(),
            is_admin: 0,
            email: "".to_string(),
            mobile_phone: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Users {
    users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageUser {
    error: String,
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageUsers {
    error: String,
    users: Users,
}

pub fn create_routes() -> Router<AppState<'static>> {
    Router::new()
        .route("/", get(list))
        .route("/:id", get(display))
        .route("/add", get(add))
        .route("/edit/:id", get(edit))
        .route("/save", post(save))
        .route("/delete/:id", post(delete))
    //.route("/password/:id", get(get_password))
    //.route("/password/:id", post(set_password))

}

pub async fn list(
    state: State<AppState<'_>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let users = model::list_all(&state.pool).await;
    let e = params.get("error");
    let error = e.unwrap_or(&"".to_string()).to_string();

    let page = PageUsers { error, users };
    render_into_response(state, "admin/user/list.html", &page)
}

pub async fn display(state: State<AppState<'_>>, id: Path<String>) -> impl IntoResponse {
    let user = model::select_by_id(&state.pool, &id).await;
    let page = PageUser {
        error: "".to_string(),
        user,
    };
    render_into_response(state, "admin/user/display.html", &page)
}

pub async fn add(state: State<AppState<'_>>) -> impl IntoResponse {
    let user = User {
        ..Default::default()
    };
    let page = PageUser {
        error: "".to_string(),
        user,
    };
    render_into_response(state, "admin/user/form.html", &page)
}

pub async fn edit(state: State<AppState<'_>>, id: Path<String>) -> impl IntoResponse {
    let user = model::select_by_id(&state.pool, &id).await;
    let page = PageUser {
        error: "".to_string(),
        user,
    };
    render_into_response(state, "admin/user/form.html", &page)
}

pub async fn save(state: State<AppState<'_>>, Form(input): Form<User>) -> impl IntoResponse {
    match model::save(
        &state.pool,
        &input.id,
        &input.user_name,
        &input.display_name,
        input.is_admin,
        &input.email,
        &input.mobile_phone,
    )
    .await
    {
        Ok(_) => Redirect::to("/Admin/User").into_response(),
        Err(e) => {
            Redirect::to(&format!("/Admin/User?error=Failed to Save. {:?}", e)).into_response()
        }
    }
}

pub async fn delete(state: State<AppState<'_>>, id: Path<String>) -> impl IntoResponse {
    match model::delete(&state.pool, &id).await {
        Ok(_) => Redirect::to("/Admin/User").into_response(),
        Err(e) => {
            Redirect::to(&format!("/Admin/User?error=Failed to Delete. {:?}", e)).into_response()
        }
    }
}
