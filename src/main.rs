use anyhow::Result;

use easy_window_switcher_rs::{cli, external_tools};

fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    external_tools::check_if_all_tools_installed();
    cli::run()
}
