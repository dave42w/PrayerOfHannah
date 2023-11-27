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

use std::collections::BTreeMap;

use axum::{extract::State, response::IntoResponse};

use super::render_into_response;
use crate::AppState;

pub async fn get_home(state: State<AppState<'_>>) -> impl IntoResponse {
    let mut data = BTreeMap::new();
    data.insert("Dave".to_string(), "dave".to_string());
    render_into_response(state, "home.html", &data)
}
