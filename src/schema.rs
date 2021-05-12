use crate::*;
use rusqlite::Row;
use serde::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Album {
    pub id: i32,
    pub category: String,
    pub producer: String,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub photo_filename: String,
    pub photo_description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AlbumPreview {
    pub id: i32,
    pub category: String,
    // pub producer: String,
    pub name: String,
    // pub description: String,
    pub price: i32,
    pub photo_filename: String,
    // pub photo_description: String,
}

pub fn initialize() {
    connection()
        .execute(
            "create table if not exists album (\
                id integer primary key, \
                category text not null, \
                producer text not null, \
                name text not null, \
                description text not null, \
                price integer not null, \
                photo_filename text not null, \
                photo_description text not null\
                )",
            [],
        )
        .unwrap();
}

pub fn album_from_row(row: &Row) -> Album {
    Album {
        id: row.get(0).unwrap(),
        category: row.get(1).unwrap(),
        producer: row.get(2).unwrap(),
        name: row.get(3).unwrap(),
        description: row.get(4).unwrap(),
        price: row.get(5).unwrap(),
        photo_filename: row.get(6).unwrap(),
        photo_description: row.get(7).unwrap(),
    }
}

pub fn get_all_albums(order: String, order2: String) -> Vec<Album> {
    let connection = connection();
    let mut statement = connection
        .prepare(&*format!(
            "select * from album order by {} {}",
            order, order2
        ))
        .unwrap();
    statement
        .query_map([], |row| Ok(album_from_row(row)))
        .unwrap()
        .map(|a| a.unwrap())
        .collect()
}

pub fn get_album_by_id(id: i32) -> Album {
    get_all_albums("id".to_string(), "asc".to_string())
        .iter()
        .find(|a| a.id == id)
        .unwrap()
        .clone()
}

pub fn get_all_albums_previews() -> Vec<AlbumPreview> {
    let connection = connection();
    let mut statement = connection.prepare("select * from album").unwrap();
    statement
        .query_map([], |row| {
            Ok(AlbumPreview {
                id: row.get(0).unwrap(),
                category: row.get(1).unwrap(),
                // producer: row.get(2).unwrap(),
                name: row.get(3).unwrap(),
                // description: row.get(4).unwrap(),
                price: row.get(5).unwrap(),
                photo_filename: row.get(6).unwrap(),
                // photo_description: row.get(7).unwrap(),
            })
        })
        .unwrap()
        .map(|a| a.unwrap())
        .collect()
}
