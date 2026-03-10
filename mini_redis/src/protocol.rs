use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Request {
    pub cmd: String,
    pub key: Option<String>,
    pub value: Option<String>,
    pub seconds: Option<u64>,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Response {
    pub fn ok() -> Self {
        Response {
            status: "ok",
            value: None, count: None,
            keys: None, ttl: None, message: None,
        }
    }

    pub fn ok_value(value: Option<String>) -> Self {
        Response {
            status: "ok",
            value: Some(match value {
                Some(v) => serde_json::Value::String(v),
                None => serde_json::Value::Null,
            }),
            count: None, keys: None, ttl: None, message: None,
        }
    }

    pub fn ok_count(count: u32) -> Self {
        Response {
            status: "ok",
            count: Some(count),
            value: None, keys: None, ttl: None, message: None,
        }
    }

    pub fn ok_keys(keys: Vec<String>) -> Self {
        Response {
            status: "ok",
            keys: Some(keys),
            value: None, count: None, ttl: None, message: None,
        }
    }

    pub fn ok_ttl(ttl: i64) -> Self {
        Response {
            status: "ok",
            ttl: Some(ttl),
            value: None, count: None, keys: None, message: None,
        }
    }

    pub fn ok_int(value: i64) -> Self {
        Response {
            status: "ok",
            value: Some(serde_json::Value::Number(value.into())),
            count: None, keys: None, ttl: None, message: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Response {
            status: "error",
            message: Some(message.to_string()),
            value: None, count: None, keys: None, ttl: None,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}   