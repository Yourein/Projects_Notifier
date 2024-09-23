use redis::{Commands, RedisResult};

pub(crate) struct RedisWrapper {
    client: redis::Client,
}

impl RedisWrapper {
    pub fn new(uri: &str) -> RedisResult<Self> {
        Ok(Self {
            client: redis::Client::open(uri)?
        })
    }

    pub fn get_task(&mut self, id: String) -> RedisResult<Option<String>> {
        let mut connection = self.client.get_connection()?;
        connection.get(id)
    }

    pub fn put_task(&mut self, id: String, title: String) -> RedisResult<()> {
        let mut connection = self.client.get_connection()?;
        connection.set(id, title)
    }
}