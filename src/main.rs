use std::io;

mod app;
mod text;
mod widgets;

fn main() -> Result<(), app::AppError> {
    let app = app::App::from_file("mcmurphey.txt")?;
    app.run(io::stdout())
}
