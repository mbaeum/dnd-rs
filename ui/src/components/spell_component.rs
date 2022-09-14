use yew::prelude::*;

use lib::config::handler::SettingsHandler;
use lib::config::settings::{Settings, SpellApi, SpellDatasource, SpellSettings};
use lib::core::entity::spell::Spell;
use lib::core::usecase::spell::SpellInterface;

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(String),
}

pub enum Msg {
    SetSpellFetchState(FetchState<Spell>),
    GetSpell,
    // GetError,
}
pub struct SpellComponent {
    spell: FetchState<Spell>,
}

impl SpellComponent {
    fn get_settings() -> Settings {
        Settings {
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
        }
    }

    fn setup_spell_usecase() -> impl SpellInterface {
        let settings = SpellComponent::get_settings();
        let handler = SettingsHandler::new(settings);
        handler.setup_spell_usecase()
    }

    async fn get_random_spell() -> Result<Spell, String> {
        let level: Option<f64> = None;
        let classes: Vec<String> = vec![];
        let exact_level = false;

        let mut usecase = SpellComponent::setup_spell_usecase();
        let res = usecase.get_random_spell(level, classes, exact_level).await;

        match res {
            Ok(spell) => Ok(spell),
            Err(_) => Err("Error getting spell".to_string()),
        }
    }
}

impl Component for SpellComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            spell: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetSpellFetchState(fetch_state) => {
                self.spell = fetch_state;
                true
            }
            Msg::GetSpell => {
                ctx.link().send_future(async {
                    match SpellComponent::get_random_spell().await {
                        Ok(s) => Msg::SetSpellFetchState(FetchState::Success(s)),
                        Err(err) => Msg::SetSpellFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetSpellFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.spell {
            FetchState::NotFetching => html! {
                <>
                    <button onclick={ctx.link().callback(|_| Msg::GetSpell)}>
                        { "Get Spell" }
                    </button>
                </>
            },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(spell) => html! { <h2>{spell.name.clone()}</h2> },
            FetchState::Failed(err) => html! { err },
        }
    }
}
