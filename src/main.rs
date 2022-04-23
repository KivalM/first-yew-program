use yew::prelude::*;

// the function component that will hold all our content
// Everything within the html! macro will be displayed on our page
#[function_component(Model)]
fn model() -> Html {
    html! {
        <div>{"Hello World"}</div>
    }
}

fn main() {
    yew::start_app::<Model>();
}
