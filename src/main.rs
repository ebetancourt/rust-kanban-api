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

    let routes = root_path.or(boards_index).or(boards_create);

    println!("API Server Started!!!");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

// fn build_random_boards() -> Vec<boards::Board> {
//     let mut boards = Vec::new();
//     for i in 0..10 {
//         let board = boards::Board::new(
//             i,
//             "title".to_string(),
//             "description".to_string(),
//         );
//         boards.push(board);
//     }
//     boards
// }

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
