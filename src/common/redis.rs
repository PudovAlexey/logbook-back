use std::sync::{Arc, Mutex};

use crate::common::env;
use redis::{Commands, Connection, RedisError, ToRedisArgs};

#[derive(Debug)]
pub struct RedisClientStatus {
    pub status: &'static str,
    pub message: String,
}

#[derive(Clone)]
pub struct Redis {
    pub connection: Arc<Mutex<Connection>>,
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
    pub fn set_item<V: ToRedisArgs>(&mut self, v: SetItem<V>) -> RedisClientStatus {
        //   let mut con = self.new().unwrap();
        let value = v.value;

        let mut connection = self.connection.lock().unwrap();

        let res: Result<(), RedisError> = connection.set(v.key, value);

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

    pub fn set_expire_item<V: ToRedisArgs>(&self, v: SetExpireItem<V>) -> RedisClientStatus {
        let mut connection = self.connection.lock().unwrap();

        let res: Result<(), RedisError> = connection.set(&v.key, v.value);

        let req: Result<(), RedisError> = connection.expire(v.key, v.expires);

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

    pub fn get_item(&self, key: String) -> Result<String, RedisError> {
        let mut connection = self.connection.lock().unwrap();

        connection.get(key)
    }

    pub fn remove_item(mut self, key: String) -> Result<String, RedisError> {
        let mut connection = self.connection.lock().unwrap();
        connection.del(key)
    }

    pub fn new() -> Self {
        let client = redis::Client::open(env::ENV::new().redis_port)
            .map_err(|e| e)
            .unwrap();

        let connection = client.get_connection().unwrap();

        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }
}
