use crate::protocol::{Request, Response};
use crate::store::Store;

pub fn handle_command(req: Request, store: &Store) -> Response {
    match req.cmd.as_str() {
        "PING" => Response::ok(),

        "SET" => {
            let key = match req.key {
                Some(k) => k,
                None => return Response::error("missing key"),
            };
            let value = match req.value {
                Some(v) => v,
                None => return Response::error("missing value"),
            };
            store.set(key, value);
            Response::ok()
        }

        "GET" => {
            let key = match req.key {
                Some(k) => k,
                None => return Response::error("missing key"),
            };
            Response::ok_value(store.get(&key))
        }

        "DEL" => {
            let key = match req.key {
                Some(k) => k,
                None => return Response::error("missing key"),
            };
            Response::ok_count(store.del(&key))
        }

        "KEYS" => {
            Response::ok_keys(store.keys())
        }

        "EXPIRE" => {
            let key = match req.key {
                Some(k) => k,
                None => return Response::error("missing key"),
            };
            let seconds = match req.seconds {
                Some(s) => s,
                None => return Response::error("missing seconds"),
            };
            if store.expire(&key, seconds) {
                Response::ok()
            } else {
                Response::error("key not found")
            }
        }

        "TTL" => {
            let key = match req.key {
                Some(k) => k,
                None => return Response::error("missing key"),
            };
            Response::ok_ttl(store.ttl(&key))
        }

        "INCR" => {
            let key = match req.key {
                Some(k) => k,
                None => return Response::error("missing key"),
            };
            match store.incr(&key) {
                Ok(n) => Response::ok_int(n),
                Err(e) => Response::error(&e),
            }
        }

        "DECR" => {
            let key = match req.key {
                Some(k) => k,
                None => return Response::error("missing key"),
            };
            match store.decr(&key) {
                Ok(n) => Response::ok_int(n),
                Err(e) => Response::error(&e),
            }
        }

        "SAVE" => {
            match store.save() {
                Ok(_) => Response::ok(),
                Err(e) => Response::error(&e),
            }
        }

        _ => Response::error("unknown command"),
    }
}