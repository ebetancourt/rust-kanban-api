use serde::{Serialize, Deserialize};

/// A Kanban board
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Board {
    /// The board's unique identifier
    id: i32,

    /// The board's title
    title: String,

    /// The board's description
    description: String,

    /// The board's columns
    columns: Vec<Column>,
}

/// A Kanban column
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Column {
    /// The column's unique identifier
    id: i32,

    /// The ID of the board the column is on
    board: i32,

    /// The column's title
    title: String,
}

impl Column {
    /// the constructor for a new column
    pub fn new(id: i32, title: String, board: i32) -> Column {
        Column { id, board, title }
    }
}

impl Board {
    /// the constructor for a new board
    pub fn new(id: i32, title: String, description: String) -> Board {
        Board {
            id,
            title,
            description,
            columns: Vec::new(),
        }
    }

    /// Sets the board's columns
    pub fn set_columns(&mut self, columns: Vec<Column>) {
        self.columns = columns;
    }
}
