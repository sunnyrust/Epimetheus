use std::collections::HashMap;
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};
use serde::{Serialize};

use std::error::Error;
use warp::http::StatusCode;
extern crate epimetheus_tools;
use epimetheus_tools::util::sunny_log::*;

pub fn api() -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let hello = warp::path!("hello" / String).and_then(hello);
    let route = warp::path("static").and(warp::fs::dir("www/static"));
    let query = warp::path("query")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(query);

    hello.or(query).or(route).recover(handle_rejection)
}
//async fn hello(name: String) -> Result<impl Reply, Infallible> {
 async fn hello(name: String) -> Result<impl Reply, Infallible> {
    Ok(format!("hello {} !", name))
}
async fn query(q: HashMap<String, String>) -> Result<impl Reply, Infallible> {

    Ok(format!("query is {:?}", q))
}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
        let s=format!("{:?}", err);
        eprintln!("----------------{:?}", err);
        error(s);
    }
    //  else if let Some(DivideByZero) = err.find() {
    //     code = StatusCode::BAD_REQUEST;
    //     message = "DIVIDE_BY_ZERO";
    // } 
    else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
       // eprintln!("unhandled rejection: {:?}", err);
       let s=format!("unhandled rejection: {:?}", err);
       info(s);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}