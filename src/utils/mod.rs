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

use handlebars::{
    Context, DirectorySourceOptions, Handlebars, Helper, HelperDef, HelperResult, JsonRender,
    Output, RenderContext,
};
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct AppState<'a> {
    pub handlebars: Handlebars<'a>,
    pub pool: Pool<Sqlite>,
}

#[derive(Clone, Copy)]
struct UpperHelper;

impl HelperDef for UpperHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();

        out.write(param.value().render().to_uppercase().as_ref())?;
        Ok(())
    }
}

pub fn get_initialized_handlebars(template_base_dir: &String) -> Handlebars<'static> {
    let mut handlebars: Handlebars = Handlebars::new();
    // warn about using undefined variables in template
    handlebars.set_strict_mode(true);
    handlebars.register_helper("upper", Box::new(UpperHelper));
    let dir_options: DirectorySourceOptions = Default::default();

    handlebars
        .register_templates_directory(template_base_dir, dir_options)
        .unwrap();

    handlebars
}
