use std::rc::Rc;
use std::cell::RefCell;
use slint::ComponentHandle;
use crate::core::EditorState;
use crate::AppWindow;
use crate::adapter::state_push;

pub fn register(ui: &AppWindow, state: &Rc<RefCell<EditorState>>) {
    let ui_weak = ui.as_weak();
    let state_c = state.clone();
    ui.on_key_pressed(move |key, ctrl, _shift, _alt| {
        let key_str = key.to_string();
        if ctrl && key_str == "o" {
            if let Some(ui) = ui_weak.upgrade() { ui.invoke_open_file(); }
            return;
        }
        apply_key(&mut state_c.borrow_mut(), &key_str, ctrl);
        if let Some(ui) = ui_weak.upgrade() {
            state_push::push(&ui, &state_c.borrow());
        }
    });
}

fn apply_key(st: &mut EditorState, key: &str, ctrl: bool) {
    match key {
        "\u{F700}" => st.move_up(),
        "\u{F701}" => st.move_down(),
        "\u{F702}" => st.move_left(),
        "\u{F703}" => st.move_right(),
        "\u{F729}" => st.home(),
        "\u{F72B}" => st.end(),
        "\n" | "\r" => st.enter(),
        "\u{0008}" => st.backspace(),
        "\u{F728}" | "\u{007F}" => st.delete(),
        "\t" => st.tab(),
        _ => insert_printable(st, key, ctrl),
    }
}

fn insert_printable(st: &mut EditorState, key: &str, ctrl: bool) {
    if ctrl || key.len() != 1 { return; }
    if let Some(ch) = key.chars().next() {
        if !ch.is_control() { st.insert_char(ch); }
    }
}
