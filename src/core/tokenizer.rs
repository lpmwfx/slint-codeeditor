// Pure Rust tokenizer — no platform or UI dependencies

/// RGB color value (UI layer maps to slint::Color)
#[derive(Clone, Copy, Debug)]
pub struct SynColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl SynColor {
    const fn rgb(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
}

#[derive(Clone, Debug)]
pub struct RustToken {
    pub text:  String,
    pub color: SynColor,
}

pub struct SyntaxColors {
    pub keyword:     SynColor,
    pub string:      SynColor,
    pub number:      SynColor,
    pub comment:     SynColor,
    pub function:    SynColor,
    pub operator:    SynColor,
    pub punctuation: SynColor,
    pub plain:       SynColor,
    pub bool_val:    SynColor,
    pub builtin:     SynColor,
}

impl Default for SyntaxColors {
    fn default() -> Self {
        Self {
            keyword:     SynColor::rgb(0xc6, 0x78, 0xdd),
            string:      SynColor::rgb(0x98, 0xc3, 0x79),
            number:      SynColor::rgb(0xd1, 0x9a, 0x66),
            comment:     SynColor::rgb(0x5c, 0x63, 0x70),
            function:    SynColor::rgb(0x61, 0xaf, 0xef),
            operator:    SynColor::rgb(0x56, 0xb6, 0xc2),
            punctuation: SynColor::rgb(0x63, 0x6d, 0x83),
            plain:       SynColor::rgb(0xab, 0xb2, 0xbf),
            bool_val:    SynColor::rgb(0xd1, 0x9a, 0x66),
            builtin:     SynColor::rgb(0x56, 0xb6, 0xc2),
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

pub fn detect_language(path: &str) -> &'static str {
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

pub fn tokenize_line(line: &str, colors: &SyntaxColors) -> Vec<RustToken> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let remaining: String = chars[i..].iter().collect();

        if remaining.starts_with("//") {
            tokens.push(RustToken { text: remaining, color: colors.comment });
            break;
        }

        if remaining.starts_with("/*") {
            if let Some(end) = remaining.find("*/") {
                let comment = remaining[..end + 2].to_string();
                i += comment.chars().count();
                tokens.push(RustToken { text: comment, color: colors.comment });
            } else {
                tokens.push(RustToken { text: remaining, color: colors.comment });
                break;
            }
            continue;
        }

        if chars[i] == '"' || chars[i] == '\'' || chars[i] == '`' {
            let quote = chars[i];
            let mut j = i + 1;
            while j < chars.len() {
                if chars[j] == '\\' && j + 1 < chars.len() {
                    j += 2;
                } else if chars[j] == quote {
                    j += 1;
                    break;
                } else {
                    j += 1;
                }
            }
            tokens.push(RustToken { text: chars[i..j].iter().collect(), color: colors.string });
            i = j;
            continue;
        }

        if chars[i].is_ascii_digit()
            || (chars[i] == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
        {
            let mut j = i;
            if chars[j] == '0'
                && j + 1 < chars.len()
                && (chars[j + 1] == 'x' || chars[j + 1] == 'X')
            {
                j += 2;
                while j < chars.len() && chars[j].is_ascii_hexdigit() { j += 1; }
            } else {
                while j < chars.len() && (chars[j].is_ascii_digit() || chars[j] == '.') { j += 1; }
            }
            tokens.push(RustToken { text: chars[i..j].iter().collect(), color: colors.number });
            i = j;
            continue;
        }

        if chars[i].is_ascii_alphabetic() || chars[i] == '_' || chars[i] == '$' {
            let mut j = i;
            while j < chars.len()
                && (chars[j].is_ascii_alphanumeric() || chars[j] == '_' || chars[j] == '$')
            {
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
            tokens.push(RustToken { text: word, color });
            i = j;
            continue;
        }

        if "=!<>+-*/%&|^~?:".contains(chars[i]) {
            let mut j = i + 1;
            while j < chars.len() && j < i + 3 && "=!<>+-*/%&|^~?:".contains(chars[j]) {
                j += 1;
            }
            tokens.push(RustToken { text: chars[i..j].iter().collect(), color: colors.operator });
            i = j;
            continue;
        }

        if "{}()[];,.".contains(chars[i]) {
            tokens.push(RustToken { text: chars[i].to_string(), color: colors.punctuation });
            i += 1;
            continue;
        }

        if chars[i].is_whitespace() {
            let mut j = i;
            while j < chars.len() && chars[j].is_whitespace() { j += 1; }
            tokens.push(RustToken { text: chars[i..j].iter().collect(), color: colors.plain });
            i = j;
            continue;
        }

        tokens.push(RustToken { text: chars[i].to_string(), color: colors.plain });
        i += 1;
    }

    tokens
}
