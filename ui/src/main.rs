mod app;

use app::App;

use log::{info, Level};

fn main() {
    console_log::init_with_level(Level::Debug);

    yew::start_app::<App>();
}
