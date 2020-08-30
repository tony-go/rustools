use crate::types::{Task, Status};
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
  pub task: Task
}

pub struct TaskComponent {
  pub props: Props
}

impl Component for TaskComponent {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {
      props
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    fn get_statuses(current_status: &Status) -> Vec<Html> {
      Status::get_statuses().iter().map(|status: &String| {
        html! {
          <option value={&status} selected={status.to_string() == Status::to_string(*current_status)}>{&status}</option>
        }
      }).collect()
    }

    html! {
      <div>
        <h3>{&self.props.task.name}</h3>
        <p>{&self.props.task.description}</p>
        <select name="task status">
            {get_statuses(&self.props.task.status)}
        </select>
      </div>
    }
  }
}

#[derive(Properties, Clone)]
pub struct ListProps {
  pub tasks: Vec<Task>
}

pub struct TaskListComponent {
 pub props: ListProps
}

impl Component for TaskListComponent {
  type Message = ();
  type Properties = ListProps;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {
      props
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    self.props.tasks.iter().map(|task: &Task| {
      html! {
        <TaskComponent task={task}/>
      }
    }).collect()
  }
}