use std::rc::Rc;
use slint::{Color, ModelRc, SharedString, VecModel};
use crate::core::{EditorState, editor_state::{CoreLine, CoreToken}};
use crate::{AppWindow, CodeLine, FlatToken};

pub fn push(ui: &AppWindow, state: &EditorState) {
    ui.set_lines(ModelRc::from(Rc::new(VecModel::from(map_lines(state.lines_model())))));
    ui.set_all_tokens(ModelRc::from(Rc::new(VecModel::from(map_tokens(state.flat_tokens())))));
    ui.set_cursor_line(state.cursor_line as i32);
    ui.set_cursor_col(state.cursor_col as i32);
    ui.set_total_lines(state.lines.len() as i32);
    ui.set_status_text(SharedString::from(
        format!("Ln {}, Col {}", state.cursor_line + 1, state.cursor_col + 1).as_str(),
    ));
}

fn map_lines(rows: Vec<CoreLine>) -> Vec<CodeLine> {
    rows.into_iter().map(|r| CodeLine {
        full_text:   SharedString::from(r.text.as_str()),
        line_number: r.number as i32,
        is_active:   r.is_active,
    }).collect()
}

fn map_tokens(tokens: Vec<CoreToken>) -> Vec<FlatToken> {
    tokens.into_iter().map(|t| FlatToken {
        text:        SharedString::from(t.text.as_str()),
        token_color: Color::from_argb_u8(255, t.color.r, t.color.g, t.color.b),
        line_idx:    t.line_idx as i32,
        x_offset:    t.x_offset,
    }).collect()
}
