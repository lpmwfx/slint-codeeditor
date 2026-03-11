mod core;
mod gateway;
mod adapter;

slint::include_modules!();

pub fn run(file: Option<&str>) -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let _adapter = adapter::EditorAdapter_adp::init(&ui, file);
    ui.run()
}
