use std::{sync::Mutex, usize};

use actix_files::{NamedFile, self as fs};
use actix_web::*;
use serde_derive::*;

#[derive(Serialize, Debug)]
struct State {
    todo_items: Mutex<Vec<String>>,
}

// ---- GET ----

#[get("/api/todo")]
async fn get_data(data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok()
        .json(data.todo_items.lock().unwrap().clone())
}

#[get("/")]
async fn page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./index.html")?)
}

// ---- POST ----

#[post("/api/newtodo")]
async fn post_data(data: web::Data<State>, request: web::Json<String>) -> impl Responder {
    let todo_string = &request.0;
    let mut todo_items = data.todo_items.lock().unwrap();
    todo_items.push(todo_string.to_string());
    format!("Item added: {}", todo_string.to_string())
}

// ---- DELETE ----

#[delete("/api/removetodo")]
async fn delete_todo(data: web::Data<State>, request: web::Json<String>) -> impl Responder {
    let index = &request.0.parse::<usize>().unwrap();
    let mut todo_items = data.todo_items.lock().unwrap();
    if todo_items.len() > 0 && todo_items.len() >= *index + 1 {
        todo_items.remove(*index);
    }
    format!("Removed item: {}", index.to_string())
}

#[delete("/api/clearall")]
async fn clear_all(data: web::Data<State>) -> impl Responder {
    let mut todo_items = data.todo_items.lock().unwrap();
    todo_items.clear();
    format!("All items removed")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(State {
        todo_items: Mutex::new(Vec::new())
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_data)
            .service(post_data)
            .service(delete_todo)
            .service(clear_all)
            .service(page)
            .service(fs::Files::new("/static", "./pkg").show_files_listing())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}