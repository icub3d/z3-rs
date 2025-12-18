use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::io::{self, Stdout};
use std::time::Duration;
use z3::{
    SatResult, Solver,
    ast::{Ast, Bool, Int},
};

// --- Z3 Solver Module ---

struct SudokuSolver {
    solver: Solver,
    // We keep these to easily construct assertions
    cells: Vec<Vec<Int>>,
}

impl SudokuSolver {
    fn new() -> Self {
        let solver = Solver::new();

        // Create a cell for each available position.
        let mut cells = Vec::with_capacity(4);
        for r in 0..4 {
            let mut row = Vec::with_capacity(4);
            for c in 0..4 {
                let name = format!("c_{}_{}", r, c);
                row.push(Int::new_const(name.as_str()));
            }
            cells.push(row);
        }

        let s = Self { solver, cells };
        // Initialize constraints.
        s.init_constraints();
        s
    }

    fn init_constraints(&self) {
        // Range 1..=4
        for r in 0..4 {
            for c in 0..4 {
                self.solver.assert(self.cells[r][c].ge(1));
                self.solver.assert(self.cells[r][c].le(4));
            }
        }

        // Rows Distinct
        for r in 0..4 {
            let row_refs: Vec<&Int> = self.cells[r].iter().collect();
            self.solver.assert(<Int as Ast>::distinct(&row_refs));
        }

        // Cols Distinct
        for c in 0..4 {
            let col_vec: Vec<&Int> = (0..4).map(|r| &self.cells[r][c]).collect();
            self.solver.assert(<Int as Ast>::distinct(&col_vec));
        }

        // Boxes Distinct
        for br in 0..2 {
            for bc in 0..2 {
                let mut box_cells = Vec::new();
                for r_off in 0..2 {
                    for c_off in 0..2 {
                        box_cells.push(&self.cells[br * 2 + r_off][bc * 2 + c_off]);
                    }
                }
                self.solver.assert(<Int as Ast>::distinct(&box_cells));
            }
        }
    }

    // Push a new scope and assert a value
    fn push_assertion(&self, r: usize, c: usize, val: i64) {
        self.solver.push();
        let z3_val = Int::from_i64(val);
        // Track this assertion so we can identify it in unsat core
        let name = format!("({},{})={}", r, c, val);
        let tracker = Bool::new_const(name.as_str());
        self.solver
            .assert_and_track(self.cells[r][c].eq(&z3_val), &tracker);
    }

    // Pop the last scope
    fn pop_assertion(&self) {
        self.solver.pop(1);
    }

    // Check satisfiability
    fn check(&self) -> SatResult {
        self.solver.check()
    }

    // Get unsat core as strings
    fn get_unsat_core(&self) -> Vec<String> {
        let core = self.solver.get_unsat_core();
        core.iter().map(|b| format!("{}", b)).collect()
    }
}

// --- Application State ---

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Playing,
    Error, // UNSAT state, blocking input until fixed
    Solved,
}

struct App {
    grid: [[Option<i64>; 4]; 4],
    fixed: [[bool; 4]; 4],  // Initial puzzle values are immutable
    cursor: (usize, usize), // (row, col)
    state: GameState,
    error_info: Option<String>,

    // History of user moves to sync with Z3 stack
    // (row, col, value)
    history: Vec<(usize, usize, i64)>,

    solver: SudokuSolver,
}

impl App {
    fn new() -> Self {
        let mut app = Self {
            grid: [[None; 4]; 4],
            fixed: [[false; 4]; 4],
            cursor: (0, 0),
            state: GameState::Playing,
            error_info: None,
            history: Vec::new(),
            solver: SudokuSolver::new(),
        };

        // Initial Puzzle (Shidoku)
        // . . 3 .
        // 4 . . .
        // . . . 1
        // . 2 . .
        app.set_fixed(0, 2, 3);
        app.set_fixed(1, 0, 4);
        app.set_fixed(2, 3, 1);
        app.set_fixed(3, 1, 2);

        app
    }

    fn set_fixed(&mut self, r: usize, c: usize, val: i64) {
        self.grid[r][c] = Some(val);
        self.fixed[r][c] = true;
        // Fixed values are part of the base constraints for this session
        // We push them to Z3 but NOT to history (so they can't be popped via undo)
        self.solver.push_assertion(r, c, val);
    }

    fn on_key(&mut self, key: KeyCode) -> bool {
        // Quit
        if let KeyCode::Char('q') | KeyCode::Esc = key {
            return true;
        }

        // Navigation (Allowed in all states, unless strict error blocking required)
        // User said: "not left them continue until it's removed"
        // This implies if Error, we can ONLY remove the bad move.
        // But maybe we can look at the bad move? Let's allow movement but block input.
        if self.state != GameState::Error {
            match key {
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.cursor.0 > 0 {
                        self.cursor.0 -= 1
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.cursor.0 < 3 {
                        self.cursor.0 += 1
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if self.cursor.1 > 0 {
                        self.cursor.1 -= 1
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if self.cursor.1 < 3 {
                        self.cursor.1 += 1
                    }
                }
                _ => {}
            }
        }

        match key {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                if self.state == GameState::Error {
                    return false; // Block adding more if error
                }

                let val = c.to_digit(10).unwrap() as i64;
                if (1..=4).contains(&val) {
                    self.try_add_move(val);
                }
            }
            KeyCode::Backspace | KeyCode::Delete => {
                // If in Error state, we MUST allow undoing the last move.
                // Does Backspace undo the cell under cursor, or the last history item?
                // User prompt: "Whenever a number is removed... it should pop a state."
                // In Z3 stack, we can only pop the top.
                // So Backspace acts as "Global Undo" here to respect the Z3 stack visualization.
                self.undo_last_move();
            }
            _ => {}
        }

        false
    }

    fn try_add_move(&mut self, val: i64) {
        let (r, c) = self.cursor;

        // Cannot overwrite fixed cells
        if self.fixed[r][c] {
            return;
        }

        // If cell is already filled, we must remove it first (rebuild stack)
        // But to simplify for this "Push/Pop" tutorial, let's enforce:
        // You can only fill empty cells. If you want to change, delete first.
        if self.grid[r][c].is_some() {
            return;
        }

        // Apply Move
        self.grid[r][c] = Some(val);
        self.history.push((r, c, val));

        // Z3 Push
        self.solver.push_assertion(r, c, val);

        // Check Validity
        match self.solver.check() {
            SatResult::Sat => {
                self.error_info = None;
                // Check if full
                if self.history.len() + 4 /*fixed*/ == 16 {
                    self.state = GameState::Solved;
                } else {
                    self.state = GameState::Playing;
                }
            }
            SatResult::Unsat => {
                self.state = GameState::Error;
                let core = self.solver.get_unsat_core();
                self.error_info = Some(format!("Conflict: [{}]", core.join(", ")));
            }
            SatResult::Unknown => {
                self.state = GameState::Error; // Treat unknown as error
                self.error_info = Some("Unknown error".to_string());
            }
        }
    }

    fn undo_last_move(&mut self) {
        if let Some((r, c, _)) = self.history.pop() {
            // Revert Grid
            self.grid[r][c] = None;

            // Z3 Pop
            self.solver.pop_assertion();

            // Re-evaluate State
            // If we popped the error causing move, we should be back to Playing
            match self.solver.check() {
                SatResult::Sat => {
                    self.state = GameState::Playing;
                    self.error_info = None;
                }
                SatResult::Unsat => {
                    self.state = GameState::Error; // Should not happen if history logic is sound
                    let core = self.solver.get_unsat_core();
                    self.error_info = Some(format!("Conflict: [{}]", core.join(", ")));
                }
                _ => {
                    self.state = GameState::Playing;
                    self.error_info = None;
                }
            }

            // Move cursor to the undone cell for better UX
            self.cursor = (r, c);
        }
    }
}

// --- UI Rendering ---

fn draw_ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &App) -> io::Result<()> {
    terminal.draw(|f| {
        // Vertical Layout: Title, Spacer, Puzzle, Spacer, Info
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title Box
                Constraint::Min(1),     // Top Spacer
                Constraint::Length(10), // Puzzle Grid
                Constraint::Min(1),     // Bottom Spacer
                Constraint::Length(5),  // Info Panel
            ])
            .split(f.area());

        // Horizontal Centering for Puzzle
        let puzzle_area = vertical_chunks[2];
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(25), // Approximate width of the grid
                Constraint::Min(1),
            ])
            .split(puzzle_area);
        let center_area = horizontal_chunks[1];

        // --- Title ---
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan))
            .title(" Z3 Tutorial ");

        let title_text = Paragraph::new("Shidoku (Push/Pop)")
            .block(title_block)
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_widget(title_text, vertical_chunks[0]);

        // --- Grid ---
        // We render the grid manually as a Paragraph with styled spans
        let mut grid_text = Vec::new();

        // Add a top margin line inside the centering box
        grid_text.push(Line::from(""));

        grid_text.push(Line::from("  ╔═══════╤═══════╗"));

        for r in 0..4 {
            let mut row_spans = Vec::new();
            row_spans.push(Span::raw("  ║"));

            for c in 0..4 {
                // Determine Cell Style
                let is_cursor = (r, c) == app.cursor;
                // Use " V " format (3 chars)
                let val_str = if let Some(v) = app.grid[r][c] {
                    format!(" {} ", v)
                } else {
                    " _ ".to_string()
                };

                let mut style = Style::default();
                if app.fixed[r][c] {
                    style = style.fg(Color::Cyan); // Fixed values
                } else if app.grid[r][c].is_some() {
                    style = style.fg(Color::Yellow); // User values
                }

                if is_cursor {
                    style = style.bg(Color::White).fg(Color::Black);
                }

                row_spans.push(Span::styled(val_str, style));

                if c == 1 {
                    row_spans.push(Span::raw("│")); // 1 char
                } else if c < 3 {
                    row_spans.push(Span::raw(" ")); // 1 char spacer between cols 0-1 and 2-3
                }
            }
            row_spans.push(Span::raw("║"));
            grid_text.push(Line::from(row_spans));

            if r == 1 {
                grid_text.push(Line::from("  ╟───────┼───────╢"));
            }
        }

        grid_text.push(Line::from("  ╚═══════╧═══════╝"));

        let grid_widget = Paragraph::new(grid_text)
            .alignment(Alignment::Left) // Aligned left within the centered chunk
            .block(Block::default()); // No border, the grid itself is a border

        f.render_widget(grid_widget, center_area);

        // --- Info / Status ---
        let status_text = match app.state {
            GameState::Playing => {
                Span::styled("Status: Playing (SAT)", Style::default().fg(Color::Green))
            }
            GameState::Error => {
                let msg = if let Some(ref info) = app.error_info {
                    format!("Status: UNSAT! {}", info)
                } else {
                    "Status: UNSAT! Conflict.".to_string()
                };
                Span::styled(
                    msg,
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )
            }
            GameState::Solved => Span::styled(
                "Status: SOLVED!",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        };

        let controls_text = match app.state {
            GameState::Error => "BACKSPACE: Undo Conflict | Q: Quit",
            _ => "ARROWS: Move | 1-4: Fill | BACKSPACE: Undo | Q: Quit",
        };

        let help_text = vec![
            Line::from(status_text),
            Line::from(Span::styled(
                controls_text,
                Style::default().fg(Color::Gray),
            )),
        ];

        let info_block = Paragraph::new(help_text)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .title(" Info ")
                    .style(Style::default().fg(Color::Blue)),
            )
            .alignment(Alignment::Center);

        f.render_widget(info_block, vertical_chunks[4]);
    })?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App
    let mut app = App::new();

    // Main Loop
    loop {
        draw_ui(&mut terminal, &app)?;

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && app.on_key(key.code)
        {
            break;
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
