// BAppBuilder Code Editor — Rust Backend
// Tokenizer + editor state + Slint model updates
//
// Architecture mirrors the HTML/JS prototype:
//   HTML <textarea> hidden    →  Slint FocusScope + TextInput (hidden)
//   HTML <div> per line       →  Slint for-loop over lines model
//   HTML <span> per token     →  Slint for-loop over flat tokens model
//   JS tokenizer              →  Rust tokenizer (below)
//   JS state (cursorLine/Col) →  Rust EditorState struct

use slint::{Color, ModelRc, SharedString, VecModel};
use std::rc::Rc;

slint::include_modules!();

fn detect_language(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "js" | "mjs" | "cjs" => "JavaScript",
        "ts" | "tsx"         => "TypeScript",
        "rs"                 => "Rust",
        "slint"              => "Slint",
        "sql"                => "SQL",
        "json"               => "JSON",
        "toml"               => "TOML",
        "md"                 => "Markdown",
        _                    => "Text",
    }
}

// ============================================================
// TOKENIZER — Same logic as the JS version, but in Rust
// In a production BAppBuilder, this would be syntect or tree-sitter
// ============================================================

#[derive(Clone, Debug)]
struct RustToken {
    text: String,
    color: Color,
}

/// Syntax colors matching EditorTheme in .slint
struct SyntaxColors {
    keyword: Color,
    string: Color,
    number: Color,
    comment: Color,
    function: Color,
    operator: Color,
    punctuation: Color,
    plain: Color,
    bool_val: Color,
    builtin: Color,
}

impl Default for SyntaxColors {
    fn default() -> Self {
        Self {
            keyword:     Color::from_argb_u8(255, 0xc6, 0x78, 0xdd),
            string:      Color::from_argb_u8(255, 0x98, 0xc3, 0x79),
            number:      Color::from_argb_u8(255, 0xd1, 0x9a, 0x66),
            comment:     Color::from_argb_u8(255, 0x5c, 0x63, 0x70),
            function:    Color::from_argb_u8(255, 0x61, 0xaf, 0xef),
            operator:    Color::from_argb_u8(255, 0x56, 0xb6, 0xc2),
            punctuation: Color::from_argb_u8(255, 0x63, 0x6d, 0x83),
            plain:       Color::from_argb_u8(255, 0xab, 0xb2, 0xbf),
            bool_val:    Color::from_argb_u8(255, 0xd1, 0x9a, 0x66),
            builtin:     Color::from_argb_u8(255, 0x56, 0xb6, 0xc2),
        }
    }
}

const KEYWORDS: &[&str] = &[
    "function", "return", "const", "let", "var", "if", "else", "for", "while",
    "do", "switch", "case", "break", "continue", "new", "this", "class",
    "extends", "import", "export", "from", "default", "try", "catch",
    "finally", "throw", "async", "await", "yield", "typeof", "instanceof",
    "in", "of", "void", "delete",
];

const BOOLEANS: &[&str] = &["true", "false", "null", "undefined", "NaN", "Infinity"];

const BUILTINS: &[&str] = &[
    "console", "Math", "JSON", "Array", "Object", "String", "Number",
    "Date", "Promise", "Map", "Set", "RegExp", "Error", "setTimeout",
    "setInterval", "parseInt", "parseFloat", "require",
];

fn tokenize_line(line: &str, colors: &SyntaxColors) -> Vec<RustToken> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let remaining: String = chars[i..].iter().collect();

        // Line comment
        if remaining.starts_with("//") {
            tokens.push(RustToken {
                text: remaining.clone(),
                color: colors.comment,
            });
            break;
        }

        // Block comment start (simplified — single line only)
        if remaining.starts_with("/*") {
            if let Some(end) = remaining.find("*/") {
                let comment = &remaining[..end + 2];
                tokens.push(RustToken {
                    text: comment.to_string(),
                    color: colors.comment,
                });
                i += comment.chars().count();
                continue;
            } else {
                tokens.push(RustToken {
                    text: remaining.clone(),
                    color: colors.comment,
                });
                break;
            }
        }

        // Strings (double quote, single quote, backtick)
        if chars[i] == '"' || chars[i] == '\'' || chars[i] == '`' {
            let quote = chars[i];
            let mut j = i + 1;
            while j < chars.len() {
                if chars[j] == '\\' && j + 1 < chars.len() {
                    j += 2; // skip escaped char
                } else if chars[j] == quote {
                    j += 1;
                    break;
                } else {
                    j += 1;
                }
            }
            let s: String = chars[i..j].iter().collect();
            tokens.push(RustToken {
                text: s,
                color: colors.string,
            });
            i = j;
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() || (chars[i] == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit()) {
            let mut j = i;
            // Hex
            if chars[j] == '0' && j + 1 < chars.len() && (chars[j + 1] == 'x' || chars[j + 1] == 'X') {
                j += 2;
                while j < chars.len() && chars[j].is_ascii_hexdigit() {
                    j += 1;
                }
            } else {
                while j < chars.len() && (chars[j].is_ascii_digit() || chars[j] == '.') {
                    j += 1;
                }
            }
            let s: String = chars[i..j].iter().collect();
            tokens.push(RustToken {
                text: s,
                color: colors.number,
            });
            i = j;
            continue;
        }

        // Identifiers, keywords, builtins
        if chars[i].is_ascii_alphabetic() || chars[i] == '_' || chars[i] == '$' {
            let mut j = i;
            while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '_' || chars[j] == '$') {
                j += 1;
            }
            let word: String = chars[i..j].iter().collect();

            let color = if KEYWORDS.contains(&word.as_str()) {
                colors.keyword
            } else if BOOLEANS.contains(&word.as_str()) {
                colors.bool_val
            } else if BUILTINS.contains(&word.as_str()) {
                colors.builtin
            } else if j < chars.len() && chars[j] == '(' {
                colors.function
            } else {
                colors.plain
            };

            tokens.push(RustToken {
                text: word,
                color,
            });
            i = j;
            continue;
        }

        // Operators
        if "=!<>+-*/%&|^~?:".contains(chars[i]) {
            let mut j = i + 1;
            // Check for multi-char operators (===, !==, =>, etc.)
            while j < chars.len() && j < i + 3 && "=!<>+-*/%&|^~?:".contains(chars[j]) {
                j += 1;
            }
            let s: String = chars[i..j].iter().collect();
            tokens.push(RustToken {
                text: s,
                color: colors.operator,
            });
            i = j;
            continue;
        }

        // Punctuation
        if "{}()[];,.".contains(chars[i]) {
            tokens.push(RustToken {
                text: chars[i].to_string(),
                color: colors.punctuation,
            });
            i += 1;
            continue;
        }

        // Whitespace — keep as token for positioning
        if chars[i].is_whitespace() {
            let mut j = i;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            let s: String = chars[i..j].iter().collect();
            tokens.push(RustToken {
                text: s,
                color: colors.plain, // doesn't matter, whitespace is invisible
            });
            i = j;
            continue;
        }

        // Unknown char
        tokens.push(RustToken {
            text: chars[i].to_string(),
            color: colors.plain,
        });
        i += 1;
    }

    tokens
}

// ============================================================
// EDITOR STATE — Same as the JS state object
// ============================================================

struct EditorState {
    lines: Vec<String>,
    cursor_line: usize,
    cursor_col: usize,
    colors: SyntaxColors,
}

impl EditorState {
    fn new(initial_code: &str) -> Self {
        Self {
            lines: initial_code.lines().map(String::from).collect(),
            cursor_line: 0,
            cursor_col: 0,
            colors: SyntaxColors::default(),
        }
    }

    fn current_line(&self) -> &str {
        &self.lines[self.cursor_line]
    }

    fn move_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = self.cursor_col.min(self.lines[self.cursor_line].len());
        }
    }

    fn move_down(&mut self) {
        if self.cursor_line < self.lines.len() - 1 {
            self.cursor_line += 1;
            self.cursor_col = self.cursor_col.min(self.lines[self.cursor_line].len());
        }
    }

    fn move_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = self.lines[self.cursor_line].len();
        }
    }

    fn move_right(&mut self) {
        if self.cursor_col < self.lines[self.cursor_line].len() {
            self.cursor_col += 1;
        } else if self.cursor_line < self.lines.len() - 1 {
            self.cursor_line += 1;
            self.cursor_col = 0;
        }
    }

    fn home(&mut self) {
        self.cursor_col = 0;
    }

    fn end(&mut self) {
        self.cursor_col = self.lines[self.cursor_line].len();
    }

    fn enter(&mut self) {
        let line = self.lines[self.cursor_line].clone();
        let before = line[..self.cursor_col].to_string();
        let after = line[self.cursor_col..].to_string();

        // Auto-indent: match leading whitespace
        let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
        let extra = if before.trim_end().ends_with('{') || before.trim_end().ends_with('(') {
            "  "
        } else {
            ""
        };

        self.lines[self.cursor_line] = before;
        let new_line = format!("{}{}{}", indent, extra, after);
        self.cursor_line += 1;
        self.lines.insert(self.cursor_line, new_line);
        self.cursor_col = indent.len() + extra.len();
    }

    fn backspace(&mut self) {
        if self.cursor_col > 0 {
            let line = &self.lines[self.cursor_line];
            let mut chars: Vec<char> = line.chars().collect();
            // Find byte position for cursor_col (handle multi-byte chars)
            if self.cursor_col <= chars.len() {
                chars.remove(self.cursor_col - 1);
                self.lines[self.cursor_line] = chars.into_iter().collect();
                self.cursor_col -= 1;
            }
        } else if self.cursor_line > 0 {
            let current = self.lines.remove(self.cursor_line);
            self.cursor_line -= 1;
            self.cursor_col = self.lines[self.cursor_line].len();
            self.lines[self.cursor_line].push_str(&current);
        }
    }

    fn delete(&mut self) {
        let line_len = self.lines[self.cursor_line].chars().count();
        if self.cursor_col < line_len {
            let mut chars: Vec<char> = self.lines[self.cursor_line].chars().collect();
            chars.remove(self.cursor_col);
            self.lines[self.cursor_line] = chars.into_iter().collect();
        } else if self.cursor_line < self.lines.len() - 1 {
            let next = self.lines.remove(self.cursor_line + 1);
            self.lines[self.cursor_line].push_str(&next);
        }
    }

    fn tab(&mut self) {
        self.insert_text("  ");
    }

    fn insert_text(&mut self, text: &str) {
        for ch in text.chars() {
            let mut chars: Vec<char> = self.lines[self.cursor_line].chars().collect();
            chars.insert(self.cursor_col, ch);
            self.lines[self.cursor_line] = chars.into_iter().collect();
            self.cursor_col += 1;
        }
    }

    fn insert_char(&mut self, ch: char) {
        // Auto-close brackets
        let pair = match ch {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '"' => Some('"'),
            '\'' => Some('\''),
            '`' => Some('`'),
            _ => None,
        };

        let mut chars: Vec<char> = self.lines[self.cursor_line].chars().collect();
        chars.insert(self.cursor_col, ch);
        if let Some(close) = pair {
            chars.insert(self.cursor_col + 1, close);
        }
        self.lines[self.cursor_line] = chars.into_iter().collect();
        self.cursor_col += 1;
    }

    fn click(&mut self, x: f32, y: f32, char_width: f32) {
        let line_height = 22.0_f32;
        let padding_left = 12.0_f32;

        let clicked_line = (y / line_height) as usize;
        self.cursor_line = clicked_line.min(self.lines.len().saturating_sub(1));

        let clicked_col = ((x - padding_left) / char_width).round() as usize;
        let line_len = self.lines[self.cursor_line].chars().count();
        self.cursor_col = clicked_col.min(line_len);
    }

    /// Build the flat token model for Slint.
    /// Each token gets absolute position info (line index + x offset in char units).
    fn build_flat_tokens(&self) -> Vec<FlatToken> {
        let mut flat = Vec::new();

        for (line_idx, line_text) in self.lines.iter().enumerate() {
            let tokens = tokenize_line(line_text, &self.colors);
            let mut x_offset: f32 = 0.0;

            for tok in &tokens {
                // Skip pure whitespace tokens (they're invisible, just advance offset)
                let char_count = tok.text.chars().count() as f32;
                if tok.text.trim().is_empty() {
                    x_offset += char_count;
                    continue;
                }

                flat.push(FlatToken {
                    text: tok.text.clone().into(),
                    token_color: tok.color,
                    line_idx: line_idx as i32,
                    x_offset,
                });

                x_offset += char_count;
            }
        }

        flat
    }

    /// Build the lines model (for gutter line numbers and active line highlighting)
    fn build_lines_model(&self) -> Vec<CodeLine> {
        self.lines
            .iter()
            .enumerate()
            .map(|(i, text)| CodeLine {
                full_text: text.clone().into(),
                line_number: (i + 1) as i32,
                is_active: i == self.cursor_line,
            })
            .collect()
    }
}

// ============================================================
// MAIN — Wire everything together
// ============================================================

fn load_file(ui: &CodeEditor, state: &Rc<std::cell::RefCell<EditorState>>, path: &std::path::Path) {
    let content = std::fs::read_to_string(path).unwrap_or_default();
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("untitled");
    let lang = detect_language(file_name);

    *state.borrow_mut() = EditorState::new(&content);
    ui.set_file_name(SharedString::from(file_name));
    ui.set_language(SharedString::from(lang));
    update_ui(ui, &state.borrow());
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = CodeEditor::new()?;

    // Load file from command-line arg, or show demo code
    let args: Vec<String> = std::env::args().collect();
    let initial_content = if args.len() > 1 {
        let path = std::path::Path::new(&args[1]);
        let lang = detect_language(&args[1]);
        ui.set_file_name(SharedString::from(path.file_name().and_then(|n| n.to_str()).unwrap_or("untitled")));
        ui.set_language(SharedString::from(lang));
        std::fs::read_to_string(path).unwrap_or_default()
    } else {
        ui.set_file_name(SharedString::from("demo.js"));
        ui.set_language(SharedString::from("JavaScript"));
        include_str!("demo.js").to_string()
    };

    let state = Rc::new(std::cell::RefCell::new(EditorState::new(&initial_content)));
    update_ui(&ui, &state.borrow());

    // === Open file dialog ===
    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    ui.on_open_file(move || {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Code", &["js", "ts", "rs", "slint", "sql", "json", "toml", "txt", "md"])
            .add_filter("All files", &["*"])
            .pick_file()
        else { return; };

        if let Some(ui) = ui_weak.upgrade() {
            load_file(&ui, &state_clone, &path);
        }
    });

    // === Handle keyboard events ===
    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    ui.on_key_pressed(move |key, ctrl, _shift, _alt| {
        let mut st = state_clone.borrow_mut();
        let key_str = key.to_string();

        if ctrl {
            match key_str.as_str() {
                "o" => { drop(st); if let Some(ui) = ui_weak.upgrade() { ui.invoke_open_file(); } return; }
                _ => {}
            }
        }

        match key_str.as_str() {
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
            _ => {
                if !ctrl && key_str.len() == 1 {
                    let ch = key_str.chars().next().unwrap();
                    if !ch.is_control() {
                        st.insert_char(ch);
                    }
                }
            }
        }

        drop(st);
        if let Some(ui) = ui_weak.upgrade() {
            update_ui(&ui, &state_clone.borrow());
        }
    });

    // === Handle mouse clicks ===
    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    ui.on_editor_clicked(move |x, y| {
        state_clone.borrow_mut().click(x, y, 7.8_f32);
        if let Some(ui) = ui_weak.upgrade() {
            update_ui(&ui, &state_clone.borrow());
        }
    });

    ui.run()
}

/// Push updated state to Slint UI
fn update_ui(ui: &CodeEditor, state: &EditorState) {
    // Update lines model (for gutter)
    let lines = state.build_lines_model();
    let lines_model = Rc::new(VecModel::from(lines));
    ui.set_lines(ModelRc::from(lines_model));

    // Update flat tokens model (for syntax-highlighted code)
    let flat_tokens = state.build_flat_tokens();
    let tokens_model = Rc::new(VecModel::from(flat_tokens));
    ui.set_all_tokens(ModelRc::from(tokens_model));

    // Update cursor
    ui.set_cursor_line(state.cursor_line as i32);
    ui.set_cursor_col(state.cursor_col as i32);
    ui.set_total_lines(state.lines.len() as i32);

    // Update status bar
    ui.set_status_text(
        SharedString::from(format!("Ln {}, Col {}", state.cursor_line + 1, state.cursor_col + 1))
    );
}
