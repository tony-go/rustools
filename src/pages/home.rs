use anyhow::Error;
use yew::prelude::*;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;

use crate::types::{Todo};
use crate::db;
use crate::components::TaskListComponent;

// state
#[derive(Debug)]
struct State {
  todos: Vec<Todo>,
  is_loading: bool,
  error: Option<Error>
}

// component
pub struct Home {
  state: State,
  link: ComponentLink<Self>,
  task: Option<FetchTask>
}

pub enum Msg {
  GetTodos,
  GetTodosSuccess(Vec<Todo>),
  GetTodosError(Error)
}

impl Component for Home {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
   let todos = vec![];

   link.send_message(Msg::GetTodos);

   Self {
     state: State {
       todos,
       is_loading: false,
       error: None,
      },
    link,
    task: None
   }
  }

  fn update(&mut self, message: Self::Message) -> ShouldRender {
    match message {
      Msg::GetTodos => {
        self.state.is_loading = true;
        let handler = self.link.callback(move |response: db::FetchResponse<Vec<Todo>>| {
          let (_, Json(data)) = response.into_parts();
          match data {
            Ok(todos) => Msg::GetTodosSuccess(todos),
            Err(err) => Msg::GetTodosError(err)
          }
        });
        self.task = Some(db::get_todos(handler));
        true
      },
      Msg::GetTodosSuccess(todos) => {
        ConsoleService::log("GetTodosSuccess");
        self.state.is_loading = false;
        self.state.todos = todos;
        self.state.error = None; 
        true
      },
      Msg::GetTodosError(err) => {
        ConsoleService::log("GetTodosError");
        self.state.is_loading = false;
        ConsoleService::warn(format!("err: {:?}", err).as_str());
        self.state.error = Some(err); 
        true
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    let todos: Vec<Html> = self
      .state
      .todos
      .iter()
      .map(|todo: &Todo| {
        html! {
          <div>
            <h2 class="todo-title">{&todo.name}</h2>
            <p class="todo-description">{&todo.description}</p>
            <TaskListComponent tasks={&todo.items}/>
          </div>
        }
      })
      .collect();

    if self.state.is_loading {
      html! {
        <span>
          <h1>{"Loading ..."}</h1>
        </span>
      }
    } else {
      html! {
        <div class="todos-wrapper">
          <div class="todos-container">
            <h1>{"Todos"}</h1>
            {todos}
          </div>
        </div>
      }
    }

  }
}