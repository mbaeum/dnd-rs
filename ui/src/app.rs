use crate::components::spell_component::SpellComponent;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let message = String::from("Hello, World!!!");

    html! {
        <main class="container">
            <div class="row">
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>
            <div>
                <h2 class={"heading"}>{message}</h2>
                <SpellComponent />
            </div>
        </main>
    }
}
