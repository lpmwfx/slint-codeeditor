// Pure domain logic — no UI or platform dependencies
use super::tokenizer::{tokenize_line, SynColor, SyntaxColors};

/// A syntax-highlighted token ready for rendering (UI-agnostic)
pub struct CoreToken {
    pub text:    String,
    pub color:   SynColor,
    pub line_idx: usize,
    pub x_offset: f32,
}

/// A single line of code (UI-agnostic)
pub struct CoreLine {
    pub text:      String,
    pub number:    usize,
    pub is_active: bool,
}

pub struct EditorState {
    pub lines:       Vec<String>,
    pub cursor_line: usize,
    pub cursor_col:  usize,
    colors:          SyntaxColors,
}

impl EditorState {
    pub fn new(initial_code: &str) -> Self {
        let mut lines: Vec<String> = initial_code.lines().map(String::from).collect();
        if lines.is_empty() { lines.push(String::new()); }
        Self { lines, cursor_line: 0, cursor_col: 0, colors: SyntaxColors::default() }
    }

    pub fn move_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = self.cursor_col.min(self.lines[self.cursor_line].chars().count());
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor_line < self.lines.len() - 1 {
            self.cursor_line += 1;
            self.cursor_col = self.cursor_col.min(self.lines[self.cursor_line].chars().count());
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = self.lines[self.cursor_line].chars().count();
        }
    }

    pub fn move_right(&mut self) {
        let line_len = self.lines[self.cursor_line].chars().count();
        if self.cursor_col < line_len {
            self.cursor_col += 1;
        } else if self.cursor_line < self.lines.len() - 1 {
            self.cursor_line += 1;
            self.cursor_col = 0;
        }
    }

    pub fn home(&mut self) { self.cursor_col = 0; }

    pub fn end(&mut self) {
        self.cursor_col = self.lines[self.cursor_line].chars().count();
    }

    pub fn enter(&mut self) {
        let line  = self.lines[self.cursor_line].clone();
        let chars: Vec<char> = line.chars().collect();
        let before: String = chars[..self.cursor_col].iter().collect();
        let after:  String = chars[self.cursor_col..].iter().collect();

        let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
        let extra = if before.trim_end().ends_with('{') || before.trim_end().ends_with('(') {
            "  "
        } else {
            ""
        };

        self.lines[self.cursor_line] = before;
        self.cursor_line += 1;
        self.lines.insert(self.cursor_line, format!("{}{}{}", indent, extra, after));
        self.cursor_col = indent.chars().count() + extra.len();
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            let mut chars: Vec<char> = self.lines[self.cursor_line].chars().collect();
            chars.remove(self.cursor_col - 1);
            self.lines[self.cursor_line] = chars.into_iter().collect();
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            let current = self.lines.remove(self.cursor_line);
            self.cursor_line -= 1;
            self.cursor_col = self.lines[self.cursor_line].chars().count();
            self.lines[self.cursor_line].push_str(&current);
        }
    }

    pub fn delete(&mut self) {
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

    pub fn tab(&mut self) { self.insert_text("  "); }

    pub fn insert_text(&mut self, text: &str) {
        for ch in text.chars() {
            let mut chars: Vec<char> = self.lines[self.cursor_line].chars().collect();
            chars.insert(self.cursor_col, ch);
            self.lines[self.cursor_line] = chars.into_iter().collect();
            self.cursor_col += 1;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        let close = match ch {
            '(' => Some(')'), '[' => Some(']'), '{' => Some('}'),
            '"' => Some('"'), '\'' => Some('\''), '`' => Some('`'),
            _ => None,
        };
        let mut chars: Vec<char> = self.lines[self.cursor_line].chars().collect();
        chars.insert(self.cursor_col, ch);
        if let Some(c) = close { chars.insert(self.cursor_col + 1, c); }
        self.lines[self.cursor_line] = chars.into_iter().collect();
        self.cursor_col += 1;
    }

    pub fn click(&mut self, x: f32, y: f32, char_width: f32) {
        let line_height  = 22.0_f32;
        let padding_left = 12.0_f32;
        let line = ((y / line_height) as usize).min(self.lines.len().saturating_sub(1));
        let col  = (((x - padding_left) / char_width).round() as usize)
            .min(self.lines[line].chars().count());
        self.cursor_line = line;
        self.cursor_col  = col;
    }

    pub fn flat_tokens(&self) -> Vec<CoreToken> {
        let mut flat = Vec::new();
        for (line_idx, line_text) in self.lines.iter().enumerate() {
            collect_line_tokens(line_idx, line_text, &self.colors, &mut flat);
        }
        flat
    }

    pub fn lines_model(&self) -> Vec<CoreLine> {
        self.lines.iter().enumerate().map(|(i, text)| CoreLine {
            text:      text.clone(),
            number:    i + 1,
            is_active: i == self.cursor_line,
        }).collect()
    }
}

fn collect_line_tokens(
    line_idx: usize,
    line_text: &str,
    colors: &SyntaxColors,
    out: &mut Vec<CoreToken>,
) {
    let tokens = tokenize_line(line_text, colors);
    let mut x_offset: f32 = 0.0;
    for tok in &tokens {
        let char_count = tok.text.chars().count() as f32;
        if !tok.text.trim().is_empty() {
            out.push(CoreToken { text: tok.text.clone(), color: tok.color, line_idx, x_offset });
        }
        x_offset += char_count;
    }
}
