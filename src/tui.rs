//! Interactive TUI: pick a persona, optionally type allergies, view the
//! ranked sachet. Reuses `engine::recommend` — no scoring logic lives here.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Terminal;
use std::io;

use crate::engine::{recommend, Recommendation};
use crate::persona::all_personas;

enum Focus {
    Personas,
    Allergy,
}

struct App {
    personas: Vec<crate::persona::Persona>,
    list_state: ListState,
    allergy_input: String,
    focus: Focus,
    scroll: u16,
}

impl App {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        App {
            personas: all_personas(),
            list_state,
            allergy_input: String::new(),
            focus: Focus::Personas,
            scroll: 0,
        }
    }

    fn selected(&self) -> &crate::persona::Persona {
        &self.personas[self.list_state.selected().unwrap_or(0)]
    }

    fn recommendation(&self) -> Recommendation {
        let base = self.selected();
        let allergies: Vec<&str> = self
            .allergy_input
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        let p = if allergies.is_empty() {
            base.clone()
        } else {
            base.with_allergies(&allergies)
        };
        recommend(&p)
    }

    fn move_selection(&mut self, delta: i32) {
        let len = self.personas.len() as i32;
        let cur = self.list_state.selected().unwrap_or(0) as i32;
        let next = (cur + delta).rem_euclid(len);
        self.list_state.select(Some(next as usize));
        self.scroll = 0;
    }
}

pub fn run() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = event_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    result
}

fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        let rec = app.recommendation();
        terminal.draw(|f| draw(f, app, &rec))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Tab => {
                    app.focus = match app.focus {
                        Focus::Personas => Focus::Allergy,
                        Focus::Allergy => Focus::Personas,
                    };
                }
                KeyCode::Down => match app.focus {
                    Focus::Personas => app.move_selection(1),
                    Focus::Allergy => app.scroll = app.scroll.saturating_add(1),
                },
                KeyCode::Up => match app.focus {
                    Focus::Personas => app.move_selection(-1),
                    Focus::Allergy => app.scroll = app.scroll.saturating_sub(1),
                },
                KeyCode::Char(c) if matches!(app.focus, Focus::Allergy) => {
                    app.allergy_input.push(c);
                }
                KeyCode::Backspace if matches!(app.focus, Focus::Allergy) => {
                    app.allergy_input.pop();
                }
                _ => {}
            }
        }
    }
}

fn draw(f: &mut ratatui::Frame, app: &mut App, rec: &Recommendation) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let header = Paragraph::new(
        "CLD-9-style recommendation engine — not medical advice, educational take-home only.",
    )
    .block(Block::default().borders(Borders::ALL).title("cld9-engine --tui"));
    f.render_widget(header, root[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)])
        .split(root[1]);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(body[0]);

    let items: Vec<ListItem> = app
        .personas
        .iter()
        .map(|p| ListItem::new(format!("{}  {}", p.id, p.label)))
        .collect();
    let persona_style = if matches!(app.focus, Focus::Personas) {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Personas")
                .border_style(persona_style),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, left[0], &mut app.list_state);

    let allergy_style = if matches!(app.focus, Focus::Allergy) {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let allergy = Paragraph::new(app.allergy_input.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Allergies (comma-separated)")
            .border_style(allergy_style),
    );
    f.render_widget(allergy, left[1]);

    let stack_lines: Vec<Line> = rec
        .stack
        .iter()
        .enumerate()
        .flat_map(|(i, item)| {
            vec![
                Line::from(vec![Span::styled(
                    format!(
                        "#{} {} — {} [{}]",
                        i + 1,
                        item.ingredient.name,
                        item.dose.display(),
                        item.ingredient.category.as_str()
                    ),
                    Style::default().add_modifier(Modifier::BOLD),
                )]),
                Line::from(format!("    why: {}", item.why)),
                Line::from(format!("    safety: {}", item.safety_note)),
                Line::from(""),
            ]
        })
        .collect();

    let mut text = vec![
        Line::from(vec![Span::styled(
            rec.persona_label.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(format!("goals: {}", rec.goals)),
        Line::from(""),
    ];
    for f in &rec.flags {
        text.push(Line::from(Span::styled(
            format!("\u{26a0} {f}"),
            Style::default().fg(Color::Red),
        )));
    }
    text.push(Line::from(""));
    text.extend(stack_lines);

    let results = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Daily sachet · {} actives", rec.stack.len())),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    f.render_widget(results, body[1]);

    let footer = Paragraph::new(
        "Tab: switch focus  ·  ↑/↓: select persona / scroll  ·  type to edit allergies  ·  q: quit",
    );
    f.render_widget(footer, root[2]);
}
