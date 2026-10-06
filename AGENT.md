# AGENT.md

## Overview

Todo list TUI (Terminal User Interface) application written in Rust. Allows creating, editing, marking as completed, and deleting tasks, with automatic persistence to a JSON file.

## Tech Stack

- **Language:** Rust (edition 2021)
- **UI:** [ratatui](https://github.com/ratatui-org/ratatui) 0.28 — TUI framework
- **Events/Terminal:** [crossterm](https://github.com/crossterm-rs/crossterm) 0.28 — Terminal manipulation and events
- **Serialization:** [serde](https://github.com/serde-rs/serde) + [serde_json](https://github.com/serde-rs/json) — JSON persistence

## Project Structure

```
RustTui/
├── Cargo.toml          # Crate dependencies and metadata
├── Cargo.lock          # Dependency lockfile
├── src/
│   └── main.rs         # All application logic (single-file)
├── todos.json          # Persistence file (created at runtime)
└── target/             # Build artifacts
```

> **Note:** The entire project is contained in `src/main.rs` (~300 lines). There are no separate modules.

## Architecture

### Main Components

| Component    | Description                                               |
|--------------|-----------------------------------------------------------|
| `TodoItem`   | Struct with `title: String` and `completed: bool`         |
| `App`        | Application state: items, selection, input, mode          |
| `InputMode`  | Enum: `Normal` or `Editing`                               |
| `ActionType` | Enum: `Adding` or `EditingTask`                           |
| `main()`     | Terminal setup (raw mode, alternate screen) and main loop |
| `run_app()`  | Event loop and action dispatch                            |
| `ui()`       | Interface rendering with ratatui                          |

### Persistence

- **File:** `todos.json` (current working directory)
- **Loading:** At startup via `App::load_from_file()`
- **Saving:** After each mutation (toggle, add, edit, delete) via `App::save_to_file()`
- **Fallback:** If the file does not exist, a sample task is created

### Operation Modes

1. **Normal:** Navigation and actions (j/k, a, e, d, t, q)
2. **Editing:** Text input for adding/editing tasks

## Commands

| Key           | Action                   |
|---------------|--------------------------|
| `j` / `↓`     | Next task                |
| `k` / `↑`     | Previous task            |
| `t` / `Space` | Toggle completion        |
| `a`           | Add new task             |
| `e`           | Edit selected task       |
| `d`           | Delete selected task     |
| `q`           | Quit                     |
| `Enter`       | Save input (edit mode)   |
| `Esc`         | Cancel input (edit mode) |

## How to Build and Run

```bash
# Build
cargo build --release

# Run
cargo run

# Run release
cargo run --release
```

## Conventions and Patterns

- **Persistence:** JSON pretty-printed via `serde_json::to_writer_pretty`
- **Terminal:** Uses alternate screen + raw mode (restored on exit)
- **Events:** Filters only `KeyEventKind::Press` to avoid key repeat
- **Layout:** `Constraint::Min(0)` for list, `Constraint::Length(3)` for input

## Dependencies

```toml
ratatui = "0.28"
crossterm = "0.28"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Possible Future Improvements

- Split into modules (`app.rs`, `ui.rs`, `main.rs`)
- 
- Task filtering (all/pending/completed)
- Search within tasks
- Unit and integration tests
