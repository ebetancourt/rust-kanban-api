use rusqlite::{Connection, Result, params};
use warp::Filter;

use crate::boards::{Board, Column};

/// Database connection wrapper
pub struct Db {
    conn: Connection,
}

const DB_PATH: &str = "db.sqlite3";
const GET_BOARDS_QUERY :&str = "SELECT id, title, description FROM boards";
const CREATE_BOARD_QUERY: &str = "INSERT INTO boards (title, description) VALUES (?1, ?2)";
const GET_BOARD_BY_ID_QUERY: &str = "SELECT id, title, description FROM boards WHERE id = ?1";
const GET_COLUMN_BY_ID_QUERY: &str = "SELECT id, title, board FROM columns WHERE id = ?1";
const CREATE_COLUMN_QUERY :&str = "INSERT INTO \"columns\"(title, board, sort_order) VALUES (?1, ?2, ?3)";
const GET_COLUMNS_BY_BOARD_ID_QUERY: &str = "SELECT id, title, board FROM columns WHERE board = ?1 ORDER BY sort_order";

impl Db {
	/// Creates a new database connection
    pub fn new() -> Result<Db> {
        let conn = Connection::open(DB_PATH)?;
        Ok(Db { conn })
    }

	/// Creates the database tables if they don't exist
    pub fn initialize(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS boards (
				  id              INTEGER PRIMARY KEY,
				  title           TEXT NOT NULL,
				  description     TEXT NOT NULL
				  )",
            [],
        )?;
		self.conn.execute(
			"CREATE TABLE IF NOT EXISTS columns (
				  id              INTEGER PRIMARY KEY,
				  board           INTEGER NOT NULL,
				  sort_order      INTEGER NOT NULL,
				  title           TEXT NOT NULL
				  )",
			[],
		)?;
        Ok(())
    }

	/// Gets all [Boards](Board) from the database
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

	/// Get a specific [Board] by id
	pub fn get_board_by_id(&self, id: i32) -> Result<Board> {
		let mut stmt = self.conn.prepare(GET_BOARD_BY_ID_QUERY)?;
		let board = stmt.query_row([id], |row| {
			Ok(Board::new(
				row.get(0).unwrap(),
				row.get(1).unwrap(),
				row.get(2).unwrap(),
			))
		})?;

		Ok(board)
	}

	/// Create a [Board]
	pub fn create_board(&self, title: String, description: String) -> Result<Board> {
		self.conn.execute(CREATE_BOARD_QUERY, [title, description])?;
		let id = self.conn.last_insert_rowid();
		self.get_board_by_id(id as i32)
	}

	/// Add a [Column] to a [Board]
	pub fn add_column(&self, board_id: i32, title: String, order: i32) -> Result<Column> {
		let mut stmt = self.conn.prepare(CREATE_COLUMN_QUERY)?;
		stmt.execute(params![title, board_id, order])?;
		let id = self.conn.last_insert_rowid();
		self.get_column_by_id(id as i32)
	}

	/// get a specific [Column] by id
	pub fn get_column_by_id(&self, id: i32) -> Result<Column> {
		let mut stmt = self.conn.prepare(GET_COLUMN_BY_ID_QUERY)?;
		let column = stmt.query_row([id], |row| {
			Ok(Column::new(
				row.get(0).unwrap(),
				row.get(1).unwrap(),
				row.get(2).unwrap(),
			))
		})?;

		Ok(column)
	}

	/// Get all [Columns](Column) for a [Board]
	pub fn get_columns_for_board(&self, board_id: i32) -> Result<Vec<Column>> {
		let mut stmt = self.conn.prepare(GET_COLUMNS_BY_BOARD_ID_QUERY)?;
		let columns_iter = stmt.query_map([board_id], |row| {
			Ok(Column::new(
				row.get(0).unwrap(),
				row.get(1).unwrap(),
				row.get(2).unwrap(),
			))
		})?;

		let mut columns = Vec::new();
		for column in columns_iter {
			columns.push(column?);
		}
		Ok(columns)
	}
}

/// Warp filter to pass the database connection to a given route handler
pub fn with_database() -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(|| Db::new().unwrap())
}
