// Hide console window in release builds on Windows
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod core;
mod gateway;
mod adapter;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let args: Vec<String> = std::env::args().collect();
    let _adapter = adapter::EditorAdapter_adp::init(&ui, args.get(1).map(String::as_str));

    ui.run()
}
