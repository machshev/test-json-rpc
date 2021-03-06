use jsonrpc_http_server::jsonrpc_core::{Params, Result, Value};
use serde::Deserialize;

#[derive(Deserialize)]
struct HelloParams {
    name: String,
}

pub fn say_hello(params: Params) -> Result<Value> {
    let parsed: HelloParams = params.parse().unwrap();
    Ok(Value::String(format!("hello, {}", parsed.name)))
}

pub fn say_bye(_: Params) -> Result<Value> {
    Ok(Value::String(format!("bye bye")))
}
