use crate::types::Todo;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_todos(call_back: FetchCallback<Vec<Todo>>) -> FetchTask {
  let request = Request::get("/db.json")
    .body(Nothing).unwrap();

  FetchService::fetch(request, call_back).unwrap()
}