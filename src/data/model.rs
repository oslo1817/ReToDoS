use chrono::{DateTime, Utc};

/// A ToDo item with a title and due date.
pub struct ToDoItem {
    /// The title of the ToDo item.
    pub title: String,

    /// The due date of the ToDo item.
    pub due_date: DateTime<Utc>,
}
