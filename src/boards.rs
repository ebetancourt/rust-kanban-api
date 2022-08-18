use serde::{Serialize, Serializer, ser::SerializeStruct};

pub struct Board {
    id: i32,
    title: Option<String>,
    description: Option<String>,
}

impl Board {
    pub fn new(id: i32, title: Option<String>, description: Option<String>) -> Board {
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
