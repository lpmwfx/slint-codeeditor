use std::rc::Rc;
use std::cell::RefCell;
use slint::{ComponentHandle, SharedString};
use crate::core::EditorState;
use crate::AppWindow;
use crate::{gateway, adapter::state_push};

pub fn register(ui: &AppWindow, state: &Rc<RefCell<EditorState>>) {
    let ui_weak = ui.as_weak();
    let state_c = state.clone();
    ui.on_open_file(move || {
        let Some(file) = gateway::pick_and_read() else { return; };
        *state_c.borrow_mut() = EditorState::new(&file.content);
        if let Some(ui) = ui_weak.upgrade() {
            ui.set_file_name(SharedString::from(file.file_name.as_str()));
            ui.set_language(SharedString::from(file.language));
            state_push::push(&ui, &state_c.borrow());
        }
    });
}
