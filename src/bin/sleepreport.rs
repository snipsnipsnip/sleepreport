use anyhow::Context as _;
use sleepreport::{app, root};

/// The main function for the sleepreport application.
pub fn main() -> anyhow::Result<()> {
    let config = root::startup(std::env::args())?;
    let deps = app::configure(config).context("initializing")?;
    let app = app::App::new(deps);

    app.run()
}
