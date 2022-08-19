use rusqlite::{Connection, Result};
use warp::Filter;

use crate::boards::Board;

pub struct Db {
    conn: Connection,
}

const DB_PATH: &str = "db.sqlite3";
const GET_BOARDS_QUERY :&str = "SELECT id, title, description FROM boards";
const CREATE_BOARD_QUERY: &str = "INSERT INTO boards (title, description) VALUES (?1, ?2)";
const GET_BOARD_BY_ID_QUERY: &str = "SELECT id, title, description FROM boards WHERE id = ?1";

impl Db {
    pub fn new() -> Result<Db> {
        let conn = Connection::open(DB_PATH)?;
        Ok(Db { conn })
    }

    pub fn initialize(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS boards (
				  id              INTEGER PRIMARY KEY,
				  title           TEXT NOT NULL,
				  description     TEXT NOT NULL
				  )",
            [],
        )?;
        Ok(())
    }

    pub fn get_boards(&self) -> Result<Vec<Board>> {
        let mut stmt = self
            .conn
            .prepare(GET_BOARDS_QUERY)?;
        let boards_iter = stmt.query_map([], |row| {
            Ok(Board::new(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
            ))
        })?;

        let mut boards = Vec::new();
        for board in boards_iter {
            boards.push(board?);
        }
        Ok(boards)
    }

	pub fn create_board(&self, title: String, description: String) -> Result<Board> {
		self.conn.execute(CREATE_BOARD_QUERY, [title, description])?;
		let last_id: String = self.conn.last_insert_rowid().to_string();
		let mut stmt = self.conn.prepare(GET_BOARD_BY_ID_QUERY)?;
		let board = stmt.query_row([last_id], |row| {
			Ok(Board::new(
				row.get(0).unwrap(),
				row.get(1).unwrap(),
				row.get(2).unwrap(),
			))
		})?;

		Ok(board)
	}
}

pub fn with_database() -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(|| Db::new().unwrap())
}
