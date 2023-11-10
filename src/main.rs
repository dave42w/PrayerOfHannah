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

// Source code at https://github.com/dave42w/PrayerOfHannah

use handlebars::Handlebars;
use sqlx::{sqlite, Pool, Sqlite};
use dotenvy::dotenv;
use std::env;

use crate::{seed_data::seed, templates::get_initialized_handlebars};
mod seed_data;
mod routes;
mod controllers;
mod templates;
mod models;

#[derive(Clone)]
pub struct AppState<'a> {
    pub handlebars: Handlebars<'a>,
    pub pool: Pool<Sqlite>,
    pub uri: String,
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let templates_dir = env::var("TEMPLATES_DIR").expect(".env missing TEMPLATES_DIR");
    let server_uri: String = env::var("SERVER_URI").expect(".env missing SERVER");
    let database_url: String = env::var("DATABASE_URL").expect(".env missing DATABASE_URL");

    let pool = sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.expect("Could not create db pool");

    println!("Before Migration");
    sqlx::migrate!().run(&pool).await.expect("Migration fail");
    println!("After Migration");

    println!("Before Seeding");    
    seed(&pool).await.expect("Data seeding fail");
    println!("After Seeding");

    let state = AppState {
        handlebars: get_initialized_handlebars(templates_dir),
        pool: pool,
        uri: server_uri,
    };

    run(state).await    
}


pub async fn run(state: AppState<'static>) {
    let server_uri = state.uri.clone();
    let app = routes::create_routes(state);
    
    axum::Server::bind(&server_uri.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
