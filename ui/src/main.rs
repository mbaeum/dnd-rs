mod app;

use app::App;

use log::{info, Level};

pub mod components;

fn main() {
    let _ = console_log::init_with_level(Level::Debug);

    info!("App is a starting up....");

    yew::start_app::<App>();
}
