use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::{ConsoleService};

pub struct Model {
    console: ConsoleService
}

pub enum Msg {
  Click,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model {
        console: ConsoleService::new()
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Click => {
          self.console.log("Clicked!");
      }
    }
    true
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
      <div>
        <button class="ui button", onclick=|_| Msg::Click,>{ "Click" }</button>
      </div>
    }
  }
}
