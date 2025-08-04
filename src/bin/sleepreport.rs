use sleepreport::{app, root};

pub fn main() -> anyhow::Result<()> {
    let container = root::startup(std::env::args())?;
    let app = app::App::new(container);

    app.run()
}
