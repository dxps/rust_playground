use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    str::FromStr,
};
use warp::{
    hyper::{Method, StatusCode},
    query,
    reject::Reject,
    Filter, Rejection, Reply,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
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
/// A local store for questions.
#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::load(),
        }
    }

    fn load() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("cannot read questions.json")
    }

    fn add(&mut self, question: &Question) -> &mut Self {
        self.questions.insert(question.id.clone(), question.clone());
        self
    }
}

// ------------------------------
//  HANDLERS
// ------------------------------

async fn get_questions(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_status(
        "Route not found",
        StatusCode::NOT_FOUND,
    ))
}

// -----------
//  APP START
// -----------

#[tokio::main]
async fn main() {
    let store = Store::new();

    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([Method::PUT, Method::DELETE]);

    let get_questions_filter = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_questions_filter.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8083)).await;
}
