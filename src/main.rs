extern crate yew;
extern crate stdweb;

use yew::prelude::*;
use stdweb::web::*;

use choresheet::Model;

fn main() {
  let app_div = document().query_selector("#app").unwrap().unwrap();
  yew::initialize();
  App::<Model>::new().mount(app_div);
  yew::run_loop();
}
