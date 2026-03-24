# slint_codeeditor_widget

Et genanvendeligt code editor-widget bygget i **Slint + Rust** med syntax highlighting, cursor, gutter og status bar.

## Brug fra git

```toml
[dependencies]
slint_codeeditor_widget = { git = "https://github.com/dit-repo/slint-code-editor" }
```

```rust
fn main() -> Result<(), slint::PlatformError> {
    slint_codeeditor_widget::run(None)                        // demo-fil
    // slint_codeeditor_widget::run(Some("path/to/file.rs"))  // specifik fil
}
```

## Kør demo

```bash
cargo run --example demo
cargo run --example demo -- path/to/file.js
```

## Arkitektur

### Mother-child

`AppWindow` er **mother** — ejer al state, sætter alle dimensioner, modtager alle events.
`CodeEditorWidget` er **child** — stateless, modtager kun `in property`, sender callbacks op.

```
AppWindow (Window — mother)
└── CodeEditorWidget (Rectangle — child, stateless)
    ├── Toolbar
    ├── EditorView
    │   ├── gutter (LineNumber × N)
    │   └── Flickable (tokens + cursor + active-line)
    └── StatusBar
```

### Theming

`EditorTheme` er en Slint global med udelukkende `in property <color>` — ingen hardkodede farveværdier.
Rust-adapteren sætter alle tokens ved opstart via `ui.global::<EditorTheme>().set_*()`.

```
tokens/theme.slint   ← EditorTheme global (kun property-declarations)
adapter/theme.rs     ← Rust sætter alle farver
```

### Flat token model

Slint understøtter ikke nested `for`-loops. Alle tokens fra alle linjer flades til én model.
Hver token bærer `line_idx` og `x_offset` og positioneres absolut i `Flickable`:

```
"const x = 42;"
→ FlatToken { text: "const", line_idx: 0, x_offset: 0.0,  color: keyword }
→ FlatToken { text: "x",     line_idx: 0, x_offset: 6.0,  color: plain }
→ FlatToken { text: "=",     line_idx: 0, x_offset: 8.0,  color: operator }
→ FlatToken { text: "42",    line_idx: 0, x_offset: 10.0, color: number }
```

### Lag

```
core/          EditorState, tokenizer — ren domænelogik, ingen UI
gateway/       Fil-I/O, filtype-detection
adapter/       Kobler Slint-callbacks til core (keyboard, mouse, file_ops, state_push, theme)
```

## Filstruktur

```
ui/
  widget.slint          ← CodeEditorWidget (child, stateless)
  app-window.slint      ← AppWindow (demo-mother, embedder widget)
  tokens/theme.slint    ← EditorTheme design-token global
  types.slint           ← FlatToken, CodeLine structs
  editor/               ← EditorView + LineNumber
  toolbar/              ← Toolbar
  status-bar/           ← StatusBar

src/
  lib.rs                ← pub fn run() + slint::include_modules!()
  core/                 ← EditorState, tokenizer
  gateway/              ← fil-læsning
  adapter/              ← UI-wiring

examples/
  demo/main.rs          ← demo-runner
```

## Features

- JavaScript syntax highlighting (keywords, strings, numbers, comments, functions, operators)
- Piletaster, Home/End, Enter med auto-indent
- Backspace/Delete, Tab (2 spaces)
- Auto-lukning: `()` `[]` `{}` `""` `''` ` `` `
- Klik-til-position
- Animeret cursor med blink
- Scroll via Flickable
- Linjenumre med aktiv-linje highlight
- Status bar med cursor-position


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=6" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->

<!-- MIB-NOTICE -->

> **Note:** This project is as-is — it is an artefact of a MIB process. See [mib.lpmwfx.com](https://mib.lpmwfx.com/) for details. It is only an MVP, not a full release. Feel free to use it for your own projects as you like.
