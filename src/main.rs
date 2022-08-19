//! This is a simple example of using warp to serve a JSON API.
//! I am working on learning Rust and this is my first project.
use db::Db;
use serde::{Deserialize, Serialize};
use warp::{reply::Json, Filter};
mod boards;
mod db;

#[tokio::main]
async fn main() {
    db::Db::new().unwrap().initialize().unwrap();
    let root_path = warp::path::end().map(|| "Hello, World!");

    let boards_index = warp::path("boards")
        .and(warp::get())
        .and(warp::path::end())
        .and(db::with_database())
        .and_then(get_boards);

    let boards_create = warp::path!("boards" / "new")
        .and(warp::post())
        .and(warp::path::end())
        .and(db::with_database())
        .and(warp::body::json())
        .and_then(create_board);

    let board_get = warp::path("boards")
        .and(warp::get())
        .and(warp::path::param())
        .and(db::with_database())
        .and_then(get_board_by_id);
    
    let add_column = warp::path!("boards" / i32 / "columns" / "new")
        .and(warp::post())
        .and(warp::path::end())
        .and(db::with_database())
        .and(warp::body::json())
        .and_then(add_column);

    let routes = root_path.or(boards_index).or(boards_create).or(board_get).or(add_column);

    println!("API Server Started!!!");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn get_boards(db: Db) -> Result<Json, warp::Rejection> {
    let boards = db.get_boards().unwrap();
    Ok(warp::reply::json(&boards))
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BoardCreateRequest {
    pub title: String,
    pub description: String,
}

async fn create_board(
    db: Db,
    body: BoardCreateRequest,
) -> Result<Json, warp::Rejection> {
    let new_board = db.create_board(body.title, body.description).unwrap();
    Ok(warp::reply::json(&new_board))
}

async fn get_board_by_id(
    id: i32,
    db: Db,
) -> Result<Json, warp::Rejection> {
    let mut board = db.get_board_by_id(id).unwrap();
    let columns = db.get_columns_for_board(id).unwrap();
    board.set_columns(columns);

    Ok(warp::reply::json(&board))
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ColumnCreateRequest {
    pub title: String,
    pub order: i32,
}
async fn add_column(
    board_id: i32,
    db: Db,
    body: ColumnCreateRequest,
) -> Result<Json, warp::Rejection> {
    let column = db.add_column(board_id, body.title, body.order).unwrap();
    Ok(warp::reply::json(&column))
}
