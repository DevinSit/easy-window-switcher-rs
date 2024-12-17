use anyhow::Result;

use easy_window_switcher_rs::cli;

fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    cli::run()
}
