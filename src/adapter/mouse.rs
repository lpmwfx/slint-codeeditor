use std::rc::Rc;
use std::cell::RefCell;
use slint::ComponentHandle;
use crate::core::EditorState;
use crate::AppWindow;
use crate::adapter::state_push;

pub fn register(ui: &AppWindow, state: &Rc<RefCell<EditorState>>) {
    let ui_weak = ui.as_weak();
    let state_c = state.clone();
    ui.on_editor_clicked(move |x, y| {
        state_c.borrow_mut().click(x, y, 7.8_f32);
        if let Some(ui) = ui_weak.upgrade() {
            state_push::push(&ui, &state_c.borrow());
        }
    });
}
