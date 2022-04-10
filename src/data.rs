use redis::{Client, Connection, RedisResult};

/// A manager for a connection to a Redis server.
pub struct Manager {
    /// The current connection to the Redis server.
    connection: Option<Connection>,
}

impl Manager {
    /// Creates a new manager in a disconnected state.
    pub fn new() -> Manager {
        Manager { connection: None }
    }

    /// Connects to the Redis server, if not already connected, and returns the connection.
    pub fn connect(&mut self) -> RedisResult<&mut Connection> {
        if self.connection.is_none() {
            // Connect to the Redis server at `localhost:6379`.
            let client = Client::open("redis://127.0.0.1/")?;
            self.connection = Some(client.get_connection()?);
        }

        Ok(self.connection.as_mut().unwrap())
    }
}
