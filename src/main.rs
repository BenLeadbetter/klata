use std::io;

mod app;

fn main() -> Result<(), app::AppError> {
    let app = app::App::from_file("mcmurphey.txt")?;
    app.run(io::stdout())
}
