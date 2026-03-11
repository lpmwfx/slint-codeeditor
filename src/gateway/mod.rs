// Gateway — the only layer allowed to perform file IO

pub struct FileResult {
    pub content:   String,
    pub file_name: String,
    pub language:  &'static str,
}

pub fn read_file(path: &std::path::Path) -> FileResult {
    let content   = std::fs::read_to_string(path).unwrap_or_default();
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("untitled")
        .to_string();
    let language  = crate::core::detect_language(&file_name);
    FileResult { content, file_name, language }
}

pub fn demo() -> FileResult {
    FileResult {
        content:   include_str!("../demo.js").to_string(),
        file_name: "demo.js".to_string(),
        language:  "JavaScript",
    }
}

pub fn pick_and_read() -> Option<FileResult> {
    let path = rfd::FileDialog::new()
        .add_filter("Code", &["js","ts","rs","slint","sql","json","toml","txt","md"])
        .add_filter("All files", &["*"])
        .pick_file()?;
    Some(read_file(&path))
}
