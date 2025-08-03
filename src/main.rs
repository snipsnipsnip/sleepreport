mod app;
mod io;
mod root;
mod sleep;
mod util;

fn main() -> anyhow::Result<()> {
    let container = root::startup(std::env::args())?;
    let app = app::App::new(container);

    app.run()
}
