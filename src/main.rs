// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]
use uml_composer::gui::ui_builer::UIBuilder;

fn main() {
    UIBuilder::new().build()
}
