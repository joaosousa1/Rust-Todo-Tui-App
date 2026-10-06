# Todo TUI

An interactive and simple Rust todo app to learn how to use ratatui library.
:
## Features

- Add, edit, and delete tasks
- Mark tasks as completed
- Keyboard navigation (vim-style)
- Automatic persistence to JSON file
- Responsive TUI with visual highlighting

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (cargo)

## Installation & Usage

```bash
# Clone the repository
git clone <repo-url>
cd RustTui

# Run
cargo run

# Or build and run
cargo build --release
./target/release/todo-tui
```

## Commands

| Key           | Action               |
|---------------|----------------------|
| `j` / `↓`     | Next task            |
| `k` / `↑`     | Previous task        |
| `t` / `Space` | Toggle completion    |
| `a`           | Add new task         |
| `e`           | Edit selected task   |
| `d`           | Delete selected task |
| `q`           | Quit                 |
| `Enter`       | Save (edit mode)     |
| `Esc`         | Cancel (edit mode)   |

## Persistence

Tasks are automatically saved to `todos.json` in the directory where the application is run. If the file does not exist, a sample task is created.

## Stack

- [ratatui](https://github.com/ratatui-org/ratatui) — TUI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Terminal manipulation
- [serde](https://github.com/serde-rs/serde) + [serde_json](https://github.com/serde-rs/json) — JSON serialization

## License

MIT
