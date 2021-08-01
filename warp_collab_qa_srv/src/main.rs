use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use warp::{
    body::BodyDeserializeError,
    cors::CorsForbidden,
    hyper::{Method, StatusCode},
    reject::Reject,
    Filter, Rejection, Reply,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
struct QuestionId(String);

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Answer {
    id: String,
    content: String,
    question_id: String,
}

// ------------------------------
/// A local store for questions.
#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    answers: Arc<RwLock<HashMap<String, Answer>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::load())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn load() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("cannot read questions.json")
    }
}

// --------------------
#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}

// --------------------
//  Custom Error enum
#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}

impl Reject for Error {}

// ------------------------------
//  HANDLERS
// ------------------------------

async fn get_questions_handler(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut res: Vec<Question> = store.questions.read().values().cloned().collect();
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        res = res[pagination.start..pagination.end].to_vec();
    }
    Ok(warp::reply::json(&res))
}

async fn add_question_handler(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status(
        "Question added",
        StatusCode::CREATED,
    ))
}

async fn update_question_handler(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

async fn delete_question_handler(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

async fn add_answer_handler(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        id: "CI001".to_string(),
        content: params.get("content").unwrap().to_string(),
        question_id: params.get("questionId").unwrap().to_string(),
    };

    store.answers.write().insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}

// -----------
/// The common handler that is called to recover from errors.
async fn return_error_handler(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("return_error: got r={:?}", r);
    if let Some(err) = r.find::<Error>() {
        println!("return_error: got Error {}", err);
        Ok(warp::reply::with_status(
            "No valid ID provided".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        println!("return_error: got BodyDeserializeError {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
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

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions_handler);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question_handler);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question_handler);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question_handler);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(add_answer_handler);

    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .recover(return_error_handler);

    warp::serve(routes).run(([127, 0, 0, 1], 8083)).await;
}
