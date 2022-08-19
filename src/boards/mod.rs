use serde::{Serialize, Serializer, ser::SerializeStruct, Deserialize};

pub struct Board {
    id: i32,
    title: String,
    description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Column {
    id: i32,
    board: i32,
    title: String,
}

impl Column {
    pub fn new(id: i32, title: String, board: i32) -> Column {
        Column { id, board, title }
    }
}

impl Board {
    pub fn new(id: i32, title: String, description: String) -> Board {
        Board {
            id,
            title,
            description,
        }
    }
}

impl Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Board", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("description", &self.description)?;
        state.end()
    }
}
