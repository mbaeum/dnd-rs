use lib::config::handler::SettingsHandler;
use lib::config::settings::{Settings, SpellApi, SpellDatasource, SpellSettings};
use lib::core::entity::spell::Spell;
use lib::core::usecase::spell::SpellInterface;
use log::{error, info};
use std::env;
use yew::prelude::*;

fn _get_settings_handler() -> SettingsHandler {
    let settings = Settings {
        debug: true,
        spell_settings: SpellSettings {
            spell_api: SpellApi {
                url: String::from("https://www.dnd5eapi.co/graphql"),
            },
            spell_datasource: SpellDatasource {
                remote_type: String::from("graphql"),
                cache_time: 20000,
            },
        },
    };

    info!("Settigns: {:?}", settings);

    SettingsHandler::new(settings)
}

fn _get_random_spell(handler: SettingsHandler) -> Spell {
    info!("Trying to get spell usecase");
    let mut spell_usecase = handler.setup_spell_usecase();

    let level: Option<f64> = None;
    let classes: Vec<String> = vec![];
    let exact_level = false;

    let spell = spell_usecase.get_random_spell(level, classes, exact_level);

    match &spell {
        Ok(s) => info!("Spell: {}", s.name),
        Err(e) => error!("SpellError: {:?}", e),
    }

    spell.unwrap()
}

#[function_component(App)]
pub fn app() -> Html {
    let message = String::from("Hello, World!!!");
    let handler = _get_settings_handler();
    let spell = _get_random_spell(handler);
    // .collect::<Html>();

    html! {
        <main class="container">
            <div class="row">
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>
            <div>
                <h2 class={"heading"}>{message}</h2>
                // <p> {spell.name} </p>
            </div>
        </main>
    }
}
