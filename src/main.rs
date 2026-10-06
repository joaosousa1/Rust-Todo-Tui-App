use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Terminal,
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::File,
    io::{self, BufReader},
    path::Path,
};

const FILE_PATH: &str = "todos.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TodoItem {
    title: String,
    completed: bool,
}

enum InputMode {
    Normal,
    Editing,
}

enum ActionType {
    Adding,
    EditingTask,
}

struct App {
    items: Vec<TodoItem>,
    list_state: ListState,
    input: String,
    input_mode: InputMode,
    action_type: Option<ActionType>,
    confirm_delete: bool,
}

impl App {
    fn new() -> App {
        let items = Self::load_from_file();
        let mut list_state = ListState::default();
        if !items.is_empty() {
            list_state.select(Some(0));
        }

        App {
            items,
            list_state,
            input: String::new(),
            input_mode: InputMode::Normal,
            action_type: None,
            confirm_delete: false,
        }
    }

    // Persistence: Load from JSON
    fn load_from_file() -> Vec<TodoItem> {
        if Path::new(FILE_PATH).exists() {
            if let Ok(file) = File::open(FILE_PATH) {
                let reader = BufReader::new(file);
                if let Ok(items) = serde_json::from_reader(reader) {
                    return items;
                }
            }
        }
        // Default sample if file does not exist
        vec![
            TodoItem {
                title: "Press e to edit this Task".to_string(),
                completed: false,
            },
        ]
    }

    // Persistence: Save to JSON
    fn save_to_file(&self) {
        if let Ok(file) = File::create(FILE_PATH) {
            let _ = serde_json::to_writer_pretty(file, &self.items);
        }
    }

    fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn toggle(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.items[i].completed = !self.items[i].completed;
            self.save_to_file();
        }
    }

    fn start_add(&mut self) {
        self.input.clear();
        self.input_mode = InputMode::Editing;
        self.action_type = Some(ActionType::Adding);
    }

    fn start_edit(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if !self.items.is_empty() {
                self.input = self.items[i].title.clone();
                self.input_mode = InputMode::Editing;
                self.action_type = Some(ActionType::EditingTask);
            }
        }
    }

    fn submit_input(&mut self) {
        if self.input.trim().is_empty() {
            self.cancel_input();
            return;
        }

        match self.action_type {
            Some(ActionType::Adding) => {
                self.items.push(TodoItem {
                    title: self.input.drain(..).collect(),
                    completed: false,
                });
                self.list_state.select(Some(self.items.len() - 1));
            }
            Some(ActionType::EditingTask) => {
                if let Some(i) = self.list_state.selected() {
                    self.items[i].title = self.input.drain(..).collect();
                }
            }
            None => {}
        }

        self.input_mode = InputMode::Normal;
        self.action_type = None;
        self.save_to_file();
    }

    fn cancel_input(&mut self) {
        self.input.clear();
        self.input_mode = InputMode::Normal;
        self.action_type = None;
    }

    fn request_delete(&mut self) {
        if !self.items.is_empty() && self.list_state.selected().is_some() {
            self.confirm_delete = true;
        }
    }

    fn confirm_delete(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if self.items.is_empty() {
                self.confirm_delete = false;
                return;
            }
            self.items.remove(i);
            if self.items.is_empty() {
                self.list_state.select(None);
            } else if i >= self.items.len() {
                self.list_state.select(Some(self.items.len() - 1));
            } else {
                self.list_state.select(Some(i));
            }
            self.save_to_file();
        }
        self.confirm_delete = false;
    }

    fn cancel_delete(&mut self) {
        self.confirm_delete = false;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            // Ensures capture only on key press
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if app.confirm_delete {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => app.confirm_delete(),
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => app.cancel_delete(),
                    _ => {}
                }
                continue;
            }

            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Char('t') | KeyCode::Char(' ') => app.toggle(),
                    KeyCode::Char('a') => app.start_add(),
                    KeyCode::Char('e') => app.start_edit(),
                    KeyCode::Char('d') => app.request_delete(),
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => app.submit_input(),
                    KeyCode::Esc => app.cancel_input(),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let constraints = match app.input_mode {
        InputMode::Normal => vec![Constraint::Min(0)],
        InputMode::Editing => vec![Constraint::Min(0), Constraint::Length(3)],
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.area());

    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|item| {
            let symbol = if item.completed { "[x] " } else { "[ ] " };
            ListItem::new(format!("{}{}", symbol, item.title))
        })
        .collect();

    let list_title = match app.input_mode {
        InputMode::Normal => " Tasks (a: Add, e: Edit, d: Delete, t/Space: Toggle, j/k: Navigate, q: Quit) ",
        InputMode::Editing => " Tasks (Edit Mode Active) ",
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(list_title))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.list_state);

    if let InputMode::Editing = app.input_mode {
        let input_title = match app.action_type {
            Some(ActionType::Adding) => " New Task (Enter to save, Esc to cancel) ",
            Some(ActionType::EditingTask) => " Edit Task (Enter to save, Esc to cancel) ",
            None => " Input ",
        };

        let input_widget = Paragraph::new(app.input.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(input_title));

        f.render_widget(input_widget, chunks[1]);
    }

    if app.confirm_delete {
        let area = centered_rect(50, 30, f.area());
        let selected_title = app
            .list_state
            .selected()
            .and_then(|i| app.items.get(i))
            .map(|item| item.title.as_str())
            .unwrap_or("");

        let popup = Paragraph::new(format!(
            "Delete this task?\n\n\"{}\"\n\n[y/Enter] Confirm  [n/Esc] Cancel",
            selected_title
        ))
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Confirm Delete ")
                .border_style(Style::default().fg(Color::Red))
                .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        )
        .alignment(Alignment::Center);

        f.render_widget(Clear, area);
        f.render_widget(popup, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
