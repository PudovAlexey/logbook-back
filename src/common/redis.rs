use crate::common::env;
use redis::{Commands, Connection, RedisError, ToRedisArgs};

#[derive(Debug)]
pub struct RedisClientStatus {
    pub status: &'static str,
    pub message: String,
}

pub struct Redis {
    pub connection: Connection,
}

pub struct SetItem<V> {
    pub key: String,
    pub value: V,
}

pub struct SetExpireItem<V> {
    pub key: String,
    pub value: V,
    pub expires: i64,
}

impl Redis {
    pub fn set_item<V: ToRedisArgs>(mut self, v: SetItem<V>) -> RedisClientStatus {
        //   let mut con = self.new().unwrap();
        let value = v.value;

        let res: Result<(), RedisError> = self.connection.set(v.key, value);

        if res.is_ok() {
            RedisClientStatus {
                status: "success",
                message: String::from("success"),
            }
        } else {
            RedisClientStatus {
                status: "error",
                message: String::from("error"),
            }
        }
    }

    pub fn set_expire_item<V: ToRedisArgs>(&mut self, v: SetExpireItem<V>) -> RedisClientStatus {
        let res: Result<(), RedisError> = self.connection.set(&v.key, v.value);

        let req: Result<(), RedisError> = self.connection.expire(v.key, v.expires);

        if res.is_ok() && req.is_ok() {
            RedisClientStatus {
                status: "success",
                message: String::from("success"),
            }
        } else {
            RedisClientStatus {
                status: "error",
                message: String::from("error"),
            }
        }
    }

    pub fn get_item(mut self, key: String) -> Result<String, RedisError> {
        self.connection.get(key)
    }

    pub fn remove_item(mut self, key: String) -> Result<String, RedisError> {
        self.connection.del(key)
    }

    pub fn new() -> Redis {
        let client = redis::Client::open(env::ENV::new().redis_port)
            .map_err(|e| e)
            .unwrap();

        let connection = client.get_connection().unwrap();

        Redis { connection }
    }
}
