// Adapter — wires Slint UI to core domain logic.
// This file is the mother: it composes sub-adapters and owns the shared state.
mod file_ops;
mod keyboard;
mod mouse;
mod state_push;
pub mod theme;

use std::rc::Rc;
use std::cell::RefCell;
use slint::SharedString;
use crate::core::EditorState;
use crate::{gateway, AppWindow};

pub struct EditorAdapter_adp;

impl EditorAdapter_adp {
    pub fn init(ui: &AppWindow, initial_path: Option<&str>) -> Self {
        // Push design tokens before first render
        theme::push_theme(ui);

        // Load initial file or demo content
        let file = match initial_path {
            Some(p) => gateway::read_file(std::path::Path::new(p)),
            None    => gateway::demo(),
        };
        ui.set_file_name(SharedString::from(file.file_name.as_str()));
        ui.set_language(SharedString::from(file.language));

        // Shared editor state — owned here, borrowed by sub-adapters via Rc<RefCell>
        // Rc because Slint runs single-threaded; RefCell for interior mutability in callbacks
        let editor_sta = Rc::new(RefCell::new(EditorState::new(&file.content)));
        state_push::push(ui, &editor_sta.borrow());

        // Register all event listeners (sub-adapters wire their own callbacks)
        file_ops::register(ui, &editor_sta);
        keyboard::register(ui, &editor_sta);
        mouse::register(ui, &editor_sta);

        Self
    }
}
