#[derive(Debug)]
pub struct TodoItem {
    pub todo: String,
    pub info: String,
    pub status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Status {
    Todo = 0,
    Completed = 1,
}

impl TodoItem {
    pub fn new(status: Status, todo: &str, info: &str) -> Self {
        Self {
            status,
            todo: todo.to_string(),
            info: info.to_string(),
        }
    }
}

impl TryFrom<u8> for Status {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::Todo),
            1 => Ok(Status::Completed),
            _ => Err(()),
        }
    }
}
