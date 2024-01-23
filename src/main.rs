pub mod app;
pub mod event;
pub mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::App::create_and_run()
}
