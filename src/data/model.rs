use chrono::{DateTime, Utc};
use redis::{from_redis_value, FromRedisValue, ToRedisArgs, Value};
use std::collections::hash_map::HashMap;

/// A ToDo item with a title and due date.
#[derive(Debug, Hash)]
pub struct ToDoItem {
    /// The title of the ToDo item.
    pub title: String,

    /// The due date of the ToDo item.
    pub due_date: DateTime<Utc>,
}

impl FromRedisValue for ToDoItem {
    fn from_redis_value(value: Value) -> Result<ToDoItem, redis::ParsingError> {
        let map: HashMap<String, String> = from_redis_value(value)?;

        let title = map.get("title").unwrap().to_string();
        let due_date = DateTime::parse_from_rfc3339(map.get("due_date").unwrap())
            .unwrap()
            .with_timezone(&Utc);

        Ok(ToDoItem { title, due_date })
    }
}

impl ToRedisArgs for ToDoItem {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        "title".write_redis_args(out);
        self.title.write_redis_args(out);

        "due_date".write_redis_args(out);
        self.due_date.to_rfc3339().write_redis_args(out);
    }
}
