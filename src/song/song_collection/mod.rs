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

pub(crate) mod model;

use axum::http::StatusCode;
use axum::{Router, Form};
use axum::extract::{State, Path};
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use serde::{Serialize, Deserialize};

use crate::utils::AppState;

use crate::controllers::render_into_response;

#[derive(Serialize, Deserialize, Debug)]
pub struct SongCollection {
    pub id: String, 
    pub code: String,
    pub name: String,
    pub url: Option<String>,
}

impl Default for SongCollection {
    fn default() -> SongCollection {
        SongCollection {
            id: "".to_string(), 
            code: "".to_string(), 
            name: "".to_string(), 
            url: std::option::Option::Some("".to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SongCollections {
    song_collection: Vec<SongCollection>    
}

pub fn create_routes() -> Router <AppState<'static>> {
    Router::new()
    .route("/", get(list))
    .route("/:id", get(display))
    .route("/add", get(add))
    .route("/edit/:id", get(edit))
    .route("/save", post(save))
    .route("/delete/:id", post(delete))
}

pub async fn list(state: State<AppState<'_>>) -> impl IntoResponse {
    let song_collections = model::list_all(&state.pool).await;
    render_into_response(state, "song/song_collection/song_collection_list.html", &song_collections)
}

pub async fn display(state: State<AppState<'_>>, id: Path<String>) -> impl IntoResponse {
    let song_collection = model::select_by_id(&state.pool, &id).await;
    render_into_response(state, "song/song_collection/song_collection_display.html", &song_collection)
}

pub async fn add(state: State<AppState<'_>>) -> impl IntoResponse {
    let song_collection = SongCollection {..Default::default()};
    render_into_response(state, "song/song_collection/song_collection_form.html", &song_collection)
}

pub async fn edit(state: State<AppState<'_>>, id: Path<String>) -> impl IntoResponse {
    let song_collection = model::select_by_id(&state.pool, &id).await;
    render_into_response(state, "song/song_collection/song_collection_form.html", &song_collection)
}

pub async fn save(state: State<AppState<'_>>, Form(input): Form<SongCollection>) -> impl IntoResponse {
    match model::save(&state.pool, &input.id, &input.code, &input.name, &input.url).await {
        Ok(_) => Redirect::to("/SongCollection").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to Save. Error: {:?}", e)).into_response(),
    }
}

pub async fn delete(state: State<AppState<'_>>, id: Path<String>) -> impl IntoResponse {
    match model::delete(&state.pool, &id).await {
        Ok(_) => Redirect::to("/SongCollection").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to Delete. Error: {:?}", e)).into_response(),
    }
}