#![feature(proc_macro_hygiene, decl_macro)]

mod schema;

#[macro_use]
extern crate rocket;

use crate::schema::*;
use rocket::response::content::Html;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rusqlite::Connection;

pub fn connection() -> Connection {
    Connection::open("albums.db").unwrap()
}

#[get("/")]
fn index() -> Html<&'static str> {
    let page = std::fs::read_to_string("src/index.html").unwrap();
    let page = Box::leak(page.into_boxed_str());
    Html(page)
}

#[get("/all?<category>&<producer>&<order>&<order2>")]
fn all(
    category: Option<String>,
    producer: Option<String>,
    order: Option<String>,
    order2: Option<String>,
) -> Json<Vec<AlbumPreview>> {
    let mut vec = get_all_albums(
        order.unwrap_or_else(|| "id".to_string()),
        order2.unwrap_or_else(|| "asc".to_string()),
    );

    if let Some(category) = category {
        if !category.is_empty() && category != "all" {
            vec.retain(|album| album.category == category)
        }
    }

    if let Some(producer) = producer {
        if !producer.is_empty() && producer != "all" {
            vec.retain(|album| album.producer == producer)
        }
    }

    let vec = vec
        .into_iter()
        .map(|a| AlbumPreview {
            id: a.id,
            category: a.category,
            name: a.name,
            price: a.price,
            photo_filename: a.photo_filename,
        })
        .collect();

    Json(vec)
}

#[get("/detailed/<id>")]
fn detailed(id: i32) -> Json<Album> {
    Json(get_album_by_id(id))
}

#[get("/categories")]
fn categories() -> Json<Vec<String>> {
    let mut vec: Vec<String> = get_all_albums_previews()
        .iter()
        .map(|a| a.category.clone())
        .collect();
    vec.sort_unstable();
    vec.dedup();
    Json(vec)
}

#[get("/producers")]
fn producers() -> Json<Vec<String>> {
    let mut vec: Vec<String> = get_all_albums("id".to_string(), "asc".to_string())
        .iter()
        .map(|a| a.producer.clone())
        .collect();
    vec.sort_unstable();
    vec.dedup();
    Json(vec)
}

fn main() {
    schema::initialize();

    rocket::ignite()
        .mount("/", routes![index, all, detailed, categories, producers])
        // .mount("/media", StaticFiles::from("/media"))
        .mount(
            "/media",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/media")),
        )
        .launch();
}
