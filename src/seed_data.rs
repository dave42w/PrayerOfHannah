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

use sqlx::{Pool, Sqlite, Error, Transaction};
use sqlx;

use crate::models::song_collection;

pub async fn seed(pool: &Pool<Sqlite>) -> Result<(), Error> {
    let mut txn: Transaction<'_, Sqlite> = pool.begin().await?;
    seed_collection(&mut txn).await?;
    seed_author(&mut txn).await?;
    seed_song(&mut txn).await?;
    seed_song_author(&mut txn).await?;
    txn.commit().await?;    
    println!("");
    println!("done");
    Ok(())
 }

pub async fn seed_collection(txn: &mut Transaction<'_, Sqlite>) -> Result<(), Error> {
    song_collection::insert_after_code_check(txn, 
        "StF", 
        "Singing the Faith", 
        "https://www.methodist.org.uk/our-faith/worship/singing-the-faith-plus/"
    )
    .await.unwrap();

    song_collection::insert_after_code_check(txn, 
        "H&P",
        "Hymns & Psalms",
        ""
    )
    .await.unwrap();

    song_collection::insert_after_code_check(txn, 
        "SoF1",
        "Songs of Fellowship book 1",
        ""
    )
    .await.unwrap();

    Ok(())
}

pub async fn seed_author(txn: &mut Transaction<'_, Sqlite>) -> Result<(), Error> {
    insert_author(txn, 
        "Charles", 
        "Wesley", 
        "Charles Wesley",
    )
    .await.unwrap();

    insert_author(txn,
        "John",
        "Wesley",
        "John Wesley",
    )
    .await.unwrap();
    
    insert_author(txn,
        "John",
        "Bell",
        "John L. Bell",
    )
    .await.unwrap();
    
    insert_author(txn,
        "Graham",
        "Maule",
        "Graham Maule",
    )
    .await.unwrap();
    
    Ok(())
}

async fn insert_author(txn: &mut Transaction<'_, Sqlite>, first_name: &str, surname: &str, display_name: &str)  -> Result<(), Error> {
    let rec = sqlx::query!("SELECT id from Author where display_name = ?1", display_name)
    .fetch_optional(&mut **txn).await?;

    if rec.is_none() {
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
        print!(".");
    }
    Ok(())
}

pub async fn seed_song(txn: &mut Transaction<'_, Sqlite>) -> Result<(), Error> {
    insert_song(txn, 
        "StF", 
        202,
        "Hark! The herald-angels sing"
    )
    .await.unwrap();

    insert_song(txn, 
        "H&P", 
        106,
        "Hark! The herald-angels sing"
    )
    .await.unwrap();

    insert_song(txn, 
        "StF", 
        5,
        "Father, in whom we live"
    )
    .await.unwrap();

    insert_song(txn, 
        "StF", 
        671,
        "What shall we offer our good Lord"
    )
    .await.unwrap();

    insert_song(txn, 
        "StF", 
        49,
        "God beyond all names"
    )
    .await.unwrap();
 
    insert_song(txn, 
        "StF", 
        101,
        "Before the world began"
    )
    .await.unwrap();
 
    insert_song(txn, 
        "StF", 
        1,
        "All people that on earth do dwell"
    )
    .await.unwrap();
 
    Ok(())
}

async fn insert_song(txn: &mut Transaction<'_, Sqlite>, collection_code: &str, song_number: i32, song_title: &str) -> Result<(), Error> {
    let song_collection_id = song_collection::select_id_for_code(txn, collection_code).await?;

    let rec2 = sqlx::query!("SELECT id from Song where song_collection_id = ?1 and song_title = ?2", song_collection_id, song_title)
    .fetch_optional(&mut **txn).await?;

    if rec2.is_none() {

        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO Song 
            (id, song_collection_id, song_number, song_title, created_timestamp, updated_timestamp) 
            VALUES
            (?1, ?2, ?3, ?4, ?5, $6)
            "#, id, song_collection_id, song_number, song_title, now, now)
        .execute(&mut **txn)    
        .await?;
        print!(".");
    }
    Ok(())

}

pub async fn seed_song_author(txn: &mut Transaction<'_, Sqlite>) -> Result<(), Error> {
    insert_song_author(txn, 
        "StF", 
        202,
        "Charles Wesley", 
    )
    .await.unwrap();

    insert_song_author(txn, 
        "H&P", 
        106,
        "Charles Wesley", 
    )
    .await.unwrap();

    insert_song_author(txn, 
        "StF", 
        5,
        "Charles Wesley", 
    )
    .await.unwrap();

    insert_song_author(txn, 
        "StF", 
        671,
        "John Wesley", 
    )
    .await.unwrap();

    insert_song_author(txn, 
        "StF", 
        49,
        "John L. Bell", 
    )
    .await.unwrap();
 
    insert_song_author(txn, 
        "StF", 
        101,
        "John L. Bell", 
    )
    .await.unwrap();
 
    insert_song_author(txn, 
        "StF", 
        101,
        "Graham Maule", 
    )
    .await.unwrap();
 
    Ok(())
}


async fn insert_song_author(txn: &mut Transaction<'_, Sqlite>, collection_code: &str, song_number: i32, display_name: &str)  -> Result<(), Error> {

    let song_collection_id = song_collection::select_id_for_code(txn, collection_code).await?;

    let song = sqlx::query!("SELECT id from Song where song_collection_id = ?1 and song_number = ?2", song_collection_id, song_number)
    .fetch_optional(&mut **txn).await?;
    if song.is_none() {
        return Err(sqlx::Error::RowNotFound);
    };
    let song = song.unwrap();

    let author = sqlx::query!("SELECT id from Author where display_name = ?1", display_name)
    .fetch_optional(&mut **txn).await?;
    if author.is_none() {
        return Err(sqlx::Error::RowNotFound);
    };
    let author = author.unwrap();

    let song_author = sqlx::query!("SELECT song_id, author_id from SongAuthor where song_id = ?1 and author_id = $2", song.id, author.id)
    .fetch_optional(&mut **txn).await?;

    if song_author.is_none() {

        let now = chrono::Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO SongAuthor
            (song_id, author_id, created_timestamp, updated_timestamp) 
            VALUES
            (?1, ?2, ?3, ?4)
            "#, song.id, author.id, now, now)
        .execute(&mut **txn)    
        .await?;
        print!(".");
    }
    Ok(())

}
//     let cf: Vec<CollectionModel> = Collection::find()
//     .filter(collection::Column::Code.eq(code))
//     .all(txn)
//     .await?;   

//     if !cf.is_empty() {
//         let cn: i32 = cf[0].id;

//         let sn: Vec<SongModel> = Song::find()
//         .filter(song::Column::Number.eq(number.to_string()))
//         .filter(song::Column::CollectionId.eq(cn.to_string()))
//         .all(txn)
//         .await?;   

//         let a: Vec<AuthorModel> = Author::find()
//         .filter(author::Column::FirstName.eq(first_name.to_string()))
//         .filter(author::Column::Surname.eq(surname.to_string()))
//         .all(txn)
//         .await?;   

//         if !sn.is_empty() && !a.is_empty() {
//             let sid: i32 = sn[0].id;
//             let aid: i32 = a[0].id;

//             let cf: Vec<SongAuthorModel> = SongAuthor::find()
//             .filter(song_author::Column::SongId.eq(sid.to_string()))
//             .filter(song_author::Column::AuthorId.eq(aid.to_string()))
//             .all(txn)
//             .await?;   
        
//             if cf.is_empty() {
        
//                 let sa = BasicSongAuthorModel {
//                     song_id: sid,
//                     author_id: aid,
//                 };
                
//                 let am = sa.into_active_model();

//                 am.insert(txn).await.expect("could not insert");
//                 print!(".");
//             }
//         }
//     }
//     Ok(())
// }

