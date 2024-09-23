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
        println!{"Redis: get_task(id = {})", id};
        connection.get(id)
    }

    pub fn put_task(&mut self, id: String, title: String) -> RedisResult<()> {
        let mut connection = self.client.get_connection()?;
        println!{"Redis: put_task(id = {}, title = {})", id, title};
        connection.set(id, title)
    }
}