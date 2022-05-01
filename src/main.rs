use clap::Parser;
use std::io;

mod app;
mod cli;
mod db;
mod data_capture;
mod text_model;
mod widgets;

fn main() -> Result<(), app::AppError> {
    let cli_args = cli::Cli::parse();
    let app = app::App::from_file(cli_args.file)?;
    app.run(io::stdout())
}
