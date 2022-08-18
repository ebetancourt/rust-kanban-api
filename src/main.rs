use warp::{reply::Json, Filter};
mod boards;

#[tokio::main]
async fn main() {
    let root_path = warp::path::end().map(|| "Hello, World!");
    let boards_index = warp::path("boards")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(get_boards);
    let routes = root_path.or(boards_index);
    println!("API Server Started!!!");

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

fn build_random_boards() -> Vec<boards::Board> {
    let mut boards = Vec::new();
    for i in 0..10 {
        let board = boards::Board::new(
            i,
            Some("title".to_string()),
            Some("description".to_string()),
        );
        boards.push(board);
    }
    boards
}

async fn get_boards() -> Result<Json, warp::Rejection> {
    let boards = build_random_boards();
    Ok(warp::reply::json(&boards))
}
