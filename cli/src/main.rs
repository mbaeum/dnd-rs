mod cli;
use cli::MainCli;
use lib::config::{handler::SettingsHandler, settings::Settings};

use std::env;

// mod entity;
mod entity {
    pub mod dice_set;
    pub mod spell;
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let folder_path = env::var("DND_SETTINGS_DIR").unwrap_or_else(|_| ".config".into());
    let settings_handler = SettingsHandler::new(Settings::new(&folder_path).unwrap());
    let spell_usecase = settings_handler.setup_spell_usecase();
    let dice_usecase = settings_handler.setup_dice_usecase();

    let mut cli = MainCli::new(spell_usecase, dice_usecase);
    cli.run().unwrap();
}
