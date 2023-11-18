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

// use std::collections::BTreeMap;

// use axum::{extract::{State, Path}, response::{IntoResponse, Html, Redirect}, Form};
// use ::entity::collection::BasicCollection as BasicCollectionModel;
// use ::entity::collection;
// use ::entity::prelude::Collection;
// use ::entity::prelude::Song;
// use entity::song;
// use sea_orm::{ActiveModelTrait, EntityTrait, Set, QueryOrder, IntoActiveModel, QueryFilter, ColumnTrait};
// use serde::{Serialize, Deserialize};


use axum::Router;
use axum::extract::{State, Path};
use axum::response::IntoResponse;
use axum::routing::get;
use serde::{Serialize, Deserialize};



use crate::AppState;

use crate::models::song_collection;
use crate::models::song_collection::SongCollection;

use super::render_into_response;

pub fn create_song_collection_routes() -> Router <AppState<'static>> {
    Router::new()
    .route("/", get(list))
    .route("/:id", get(display))
    .route("/add", get(add))
    .route("/edit/:id", get(edit))
}


    // .route("/Collection/post", post(insert_collection))
    // .route("/Collection/patch", post(update_collection))
    // .route("/Collection/delete/:id", post(delete_collection))

pub async fn list(state: State<AppState<'_>>) -> impl IntoResponse {
    let song_collections = song_collection::list_all(&state.pool).await;
    render_into_response(state, "song_collection_list.html", &song_collections)
}

#[derive(Serialize, Deserialize)]
struct SongCollectionExtended {
    song_collection: SongCollection,
    //songs: Vec<song>    
}

#[derive(Serialize, Deserialize)]
struct SongCollectionForm {
    song_collection: SongCollection,
    method: String,
    //songs: Vec<song>    
}


pub async fn display(state: State<AppState<'_>>, Path(id): Path<String>) -> impl IntoResponse {
    let song_collection = song_collection::select_by_id(&state.pool, &id).await;
    let song_collection_extended = SongCollectionExtended{song_collection: song_collection};

    render_into_response(state, "song_collection_display.html", &song_collection_extended)
}

pub async fn add(state: State<AppState<'_>>) -> impl IntoResponse {
    let song_collection_form = SongCollectionForm {
        song_collection: SongCollection {..Default::default()},
        method: "post".to_string(),
    };
    render_into_response(state, "song_collection_form.html", &song_collection_form)
}

pub async fn edit(state: State<AppState<'_>>, Path(id): Path<String>) -> impl IntoResponse {
    let song_collection = song_collection::select_by_id(&state.pool, &id).await;
    let song_collection_form = SongCollectionForm{song_collection: song_collection, method: "patch".to_string()};

    render_into_response(state, "song_collection_form.html", &song_collection_form)
}


// pub async fn insert_collection(State(state): State<AppState<'_>>, Form(input): Form<BasicCollectionModel>) -> impl IntoResponse {
//     let s = state;

//     let am = input.into_active_model();

//     am.insert(&s.db).await.expect("could not insert");

//     Redirect::to("/Collection")
// }

// pub async fn insert_collection(State(state): State<AppState<'_>>, Form(input): Form<BasicCollectionModel>) -> impl IntoResponse {
//     let s = state;

//     let am = input.into_active_model();

//     am.insert(&s.db).await.expect("could not insert");

//     Redirect::to("/Collection")
// }

// pub async fn edit_collection_form(State(state): State<AppState<'_>>, Path(id): Path<String>) -> impl IntoResponse {
//     let s = state;
//     let i = id.parse::<i32>().unwrap();
	
//     let collection = Collection::find_by_id(i).one(&s.db).await
//     .expect("could not find the Collection");
//     let m = collection.unwrap();

//     let ms: Vec<song::Model> = Song::find().filter(song::Column::CollectionId.eq(i)).order_by_asc(song::Column::Number).all(&s.db).await
//     .expect("could not find any Songs");

//     let cf = CollectionForm {
//         method: "patch".to_string(),
//         id: m.id,
//         name: m.name.to_string(),
//         code: m.code.to_string(),
//         url: m.url.to_string(),
//         song: ms,
//     };

//     Html(s.handlebars.render("collection_form.html", &cf).unwrap()).into_response()
// }

// pub async fn update_collection(State(state): State<AppState<'_>>, Form(input): Form<BasicCollectionModel>) -> impl IntoResponse {
//     let s = state;
    
//     let i = input.id.to_owned();
//     let collection = Collection::find_by_id(i).one(&s.db).await
//     .expect("could not find the Collection");
//     let mut am: collection::ActiveModel = collection.unwrap().into();    
	
//     am.name = Set(input.name.to_owned());
//     am.code = Set(input.code.to_owned());
//     am.url = Set(input.url.to_owned());
    
//     am.update(&s.db).await.expect("could not update");
//     Redirect::to("/Collection")    
// }

// pub async fn delete_collection(State(state): State<AppState<'_>>, Path(id): Path<String>) -> impl IntoResponse {
//     let s = state;
    
//     let i = id.parse::<i32>().unwrap();
//     let collection = Collection::find_by_id(i).one(&s.db).await
//     .expect("could not find the Collection");
//     let am: collection::ActiveModel = collection.unwrap().into();    
	 
//     am.delete(&s.db).await.expect("could not delete");
//     Redirect::to("/Collection")        
// }