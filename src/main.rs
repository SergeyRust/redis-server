#[macro_use] extern crate rocket;

use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, Value};
use rocket::response::Responder;

const REDIS_URL: &str = "redis://127.0.0.1:6379";
const REDIS_GET: &str = "GET";
const REDIS_SET: &str = "SET";
const REDIS_DEL: &str = "DEL";

enum RedisCommand<'c> {
    Get { key: &'c str, },
    Post { key: &'c str, value: &'c str, },
    Put { key: &'c str, value: &'c str, },
    Delete { key: &'c str, },
}

fn request_redis(data: RedisCommand) -> redis::RedisResult<String> {
    let client = redis::Client::open(REDIS_URL)?;
    let mut con = client.get_connection()?;

    match data {
        RedisCommand::Get { key } => {
            let cached_data = redis::cmd(REDIS_GET).arg(&key).query(&mut con)?;
            Ok(cached_data)
        },
        RedisCommand::Post { key, value } => {
            let redis_result: RedisResult<Value> =
                redis::cmd(REDIS_SET).arg(&key).arg(value).arg("NX").query(&mut con);
            match redis_result {
                Ok(value) => {
                    match value {
                        Value::Nil => Err(RedisError::from((ErrorKind::ClientError, "value already exists"))),
                        Value::Data(_) => FromRedisValue::from_redis_value(&value),
                        _ => FromRedisValue::from_redis_value(&value),
                    }
                }
                Err(_) => {
                    return Err(RedisError::from((ErrorKind::IoError, "no connection")))
                }
            }
        },
        RedisCommand::Put { key, value } => {
            let redis_value =
                redis::cmd(REDIS_SET).arg(&key).arg(value).arg("XX").query(&mut con)?;
            Ok(redis_value)
        },
        RedisCommand::Delete { key } => {
            let redis_value: u64 = redis::cmd(REDIS_DEL).arg(&key).query(&mut con)?;
            match redis_value {
                0 => {
                    Err(RedisError::from((redis::ErrorKind::ClientError, "no such value")))
                },
                count if count > 0 => Ok(count.to_string()),
                _ => Ok("".to_string()),
            }
        },
    }
}

#[get("/get_data/<key>")]
fn get_data<'c>(key: &'c str) -> ApiResponse {
    match request_redis(RedisCommand::Get { key }) {
        Ok(data) => ApiResponse::Ok(format!["data: {}", data]),
        Err(e) => handle_error(e, ApiResponse::NotFound(())),
    }
}

#[post("/post_data/<key>", format = "text", data = "<value>")]
fn post_data<'c>(key: &'c str, value: &'c str) -> ApiResponse {
    match request_redis(RedisCommand::Post { key, value }) {
        Ok(_) => ApiResponse::Ok(format!["data with key {} successfully added", key]),
        Err(e) => handle_error(e, ApiResponse::Conflict(())),
    }
}

#[put("/put_data/<key>", format = "text", data = "<value>")]
fn put_data<'c>(key: &'c str, value: &'c str) -> ApiResponse {
    match request_redis(RedisCommand::Put { key, value }) {
        Ok(_) => ApiResponse::NoContent(()),
        Err(e) => handle_error(e, ApiResponse::NotFound(())),
    }
}

#[delete("/delete_data/<key>")]
fn delete_data<'c>(key: &'c str) -> ApiResponse {
    match request_redis(RedisCommand::Delete { key }) {
        Ok(_) => ApiResponse::NoContent(()),
        Err(e) => handle_error(e, ApiResponse::NotFound(())),
    }
}

fn handle_error(error: RedisError, response: ApiResponse) -> ApiResponse {
    if error.kind() == ErrorKind::IoError {
        return ApiResponse::InternalServerError(())
    }
    response
}

#[derive(Responder)]
enum ApiResponse {
    #[response(status = 200)]
    Ok(String),
    #[response(status = 204)]
    NoContent(()),
    #[response(status = 404)]
    NotFound(()),
    #[response(status = 409)]
    Conflict(()),
    #[response(status = 500)]
    InternalServerError(()),
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![get_data, post_data, put_data, delete_data])
}
