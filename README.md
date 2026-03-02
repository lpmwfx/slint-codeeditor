# Slint Code Editor — Syntax Highlighting Prototype

En fungerende code editor med syntax highlighting bygget i **Slint + Rust**, baseret på præcis de samme primitiver som HTML/JS-versionen.

## Arkitektur-princip

Samme tilgang som HTML/JS — oversat til Slint-ækvivalenter:

| HTML/JS primitiv | Slint-ækvivalent |
|---|---|
| `<textarea>` (skjult) | `FocusScope` + `TextInput` (off-screen) |
| `<div>` scroll-container | `Flickable` |
| `for`-loop over linjer | `for line in lines` |
| `<span>` med farve per token | `for token in all-tokens : Text { color: token.color }` |
| Absolut-positioneret cursor | `Rectangle` med `animate opacity` |
| JS tokenizer | Rust `tokenize_line()` |

## Nøgleindsigt: Flat Token Model

Slint understøtter ikke nested `for`-loops (en `for` inde i en `for`). Løsningen er at **flatte** alle tokens fra alle linjer til én enkelt model. Hver token bærer sin linje-index og x-offset, så den kan absolut-positioneres i `Flickable`:

```
Linje 0: "const x = 42;"
  → FlatToken { text: "const", line_idx: 0, x_offset: 0.0,   color: keyword }
  → FlatToken { text: "x",     line_idx: 0, x_offset: 6.0,   color: plain }
  → FlatToken { text: "=",     line_idx: 0, x_offset: 8.0,   color: operator }
  → FlatToken { text: "42",    line_idx: 0, x_offset: 10.0,  color: number }
  → FlatToken { text: ";",     line_idx: 0, x_offset: 12.0,  color: punctuation }
```

## Kør projektet

```bash
# Kræver Rust toolchain
cargo run
```

## Filer

```
├── Cargo.toml           # Dependencies: slint 1.12
├── build.rs             # Kompilerer .slint filen
├── ui/
│   └── editor.slint     # UI-definition med tokens, cursor, gutter
├── src/
│   └── main.rs          # Tokenizer, editor-state, Slint-integration
└── README.md
```

## Features

- JavaScript syntax highlighting (keywords, strings, numbers, comments, functions, operators)
- Piletaster, Home/End, Enter med auto-indent
- Backspace/Delete, Tab (2 spaces)
- Auto-lukning af brackets: `()`, `[]`, `{}`, `""`, `''`, ` `` `
- Klik-til-position
- Animeret cursor med blink
- Scroll via Flickable
- Linjenumre med aktiv-linje highlight
- Status bar med cursor position

## BAppBuilder-relevans

Dette demonstrerer at Slint KAN bygge en code editor med syntax highlighting — præcis som HTML/JS kan. Princippet er identisk:

1. **Skjult input** fanger tastatur-events
2. **Rust backend** tokenizer teksten og producerer farvede spans
3. **Slint for-loop** renderer tokens som absolutt-positionerede `Text`-elementer
4. **Cursor** er en animeret `Rectangle`

I BAppBuilder-kontekst ville denne editor:
- Leve i **Builder** som CORE-editoren (til SQL og JavaScript)
- Bruge **syntect** eller **tree-sitter** i stedet for den simple tokenizer
- Binde til **state-databasen** for undo/redo via variables
