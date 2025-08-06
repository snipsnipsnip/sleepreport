use anyhow::Context as _;
use sleepreport::{app, root};

/// The main function for the sleepreport application.
pub fn main() -> anyhow::Result<()> {
    let deps = root::startup(std::env::args())?;
    let app = app::App::new(deps);

    app.run()
}
