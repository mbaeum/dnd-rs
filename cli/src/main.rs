mod cli;
use cli::MainCli;
use lib::config::{handler::SettingsHandler, settings::Settings};

#[tokio::main]
async fn main() {
    let settings_handler = SettingsHandler::new(Settings::new().unwrap());
    let spell_usecase = settings_handler.setup_spell_usecase();
    let dice_usecase = settings_handler.setup_dice_usecase();

    let mut cli = MainCli::new(spell_usecase, dice_usecase);
    cli.run().unwrap();
}
