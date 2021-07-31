use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use serde::Serialize;
use warp::{
    hyper::{Method, StatusCode},
    reject::Reject,
    Filter, Rejection, Reply,
};

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

/// A custom error type that reflects the invalid question id.
#[derive(Debug)]
struct InvalidId;

impl Reject for InvalidId {}

#[derive(Debug, Serialize)]
struct QuestionId(String);

impl FromStr for QuestionId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.is_empty() {
            false => Ok(QuestionId(s.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

// ------------------------------
//  HANDLERS
// ------------------------------

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First question".to_string(),
        "Content of the question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    match question.id.0.is_empty() {
        true => Err(warp::reject::custom(InvalidId)),
        false => Ok(warp::reply::json(&question)),
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND,
        ))
    }
}

// -----------
//  APP START
// -----------

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([Method::PUT, Method::DELETE]);

    let get_questions_filter = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_questions_filter.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8083)).await;
}
