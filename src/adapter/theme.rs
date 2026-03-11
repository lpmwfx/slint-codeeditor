// Injects all EditorTheme color tokens into the Slint global.
// Called once in Adapter::init() before ui.run().
use slint::{Color, ComponentHandle};
use crate::{AppWindow, EditorTheme};

// One Dark Pro palette
const fn rgb(r: u8, g: u8, b: u8) -> Color { Color::from_argb_u8(255, r, g, b) }
const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color { Color::from_argb_u8(a, r, g, b) }

pub fn push_theme(ui: &AppWindow) {
    let t = ui.global::<EditorTheme>();

    // Background
    t.set_bg_editor(rgb(0x0f, 0x0f, 0x1a));
    t.set_bg_gutter(rgb(0x12, 0x12, 0x1f));
    t.set_bg_active_line(rgba(0xff, 0xff, 0xff, 0x08));
    t.set_bg_titlebar(rgb(0x12, 0x12, 0x1f));
    t.set_border_color(rgb(0x1e, 0x1e, 0x35));

    // Text
    t.set_text_gutter(rgb(0x3a, 0x3a, 0x55));
    t.set_text_gutter_active(rgb(0x6a, 0x6a, 0x8a));
    t.set_text_title(rgb(0x4a, 0x4a, 0x6a));
    t.set_text_info(rgb(0x3a, 0x3a, 0x55));

    // Cursor
    t.set_cursor_color(rgb(0x63, 0x8c, 0xff));

    // Syntax (One Dark Pro)
    t.set_syn_keyword(rgb(0xc6, 0x78, 0xdd));
    t.set_syn_string(rgb(0x98, 0xc3, 0x79));
    t.set_syn_number(rgb(0xd1, 0x9a, 0x66));
    t.set_syn_comment(rgb(0x5c, 0x63, 0x70));
    t.set_syn_function(rgb(0x61, 0xaf, 0xef));
    t.set_syn_type(rgb(0xe5, 0xc0, 0x7b));
    t.set_syn_property(rgb(0xe0, 0x6c, 0x75));
    t.set_syn_operator(rgb(0x56, 0xb6, 0xc2));
    t.set_syn_punctuation(rgb(0x63, 0x6d, 0x83));
    t.set_syn_plain(rgb(0xab, 0xb2, 0xbf));
    t.set_syn_bool(rgb(0xd1, 0x9a, 0x66));
    t.set_syn_builtin(rgb(0x56, 0xb6, 0xc2));

    // Button states
    t.set_btn_bg(rgb(0x1e, 0x1e, 0x38));
    t.set_btn_bg_hover(rgb(0x2a, 0x2a, 0x45));
    t.set_btn_text(rgb(0x88, 0x88, 0xbb));
}
