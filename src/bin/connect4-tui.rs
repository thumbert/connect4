use std::fmt::Debug;

use clap::Parser;
use color_eyre::{
    eyre::{bail, Ok, WrapErr},
    Result,
};
use connect4::tui;
use connect4::*;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

/// Play Connect4 from the terminal line
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Level of the AI strength (1-8)
    #[arg(short, long, default_value="3")]
    level: usize,
}



#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    state: State,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        let mut state = State {
            kind: StateKind::Live,
            player1: Player {
                name: "Adrian".to_string(),
                strategy: manual_strategy,
                // strategy: random_strategy,
            },
            player2: Player {
                name: "Bottie".to_string(),
                // strategy: random_strategy,
                // strategy: minimax_strategy3,
                strategy: minimax_strategy_level::<3>,
            },
            moves_player1: Vec::new(),
            moves_player2: Vec::new(),
            height: [0; 7],
        };
        // self.state = state;
    
        while !self.exit || self.state.kind == StateKind::Live {
            state = play(state);
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('1') => self.exit(),
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter()?,
            KeyCode::Right => self.increment_counter()?,
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) -> Result<()> {
        self.counter += 1;
        if self.counter > 2 {
            bail!("counter overflow");
        }
        Ok(())
    }

    fn decrement_counter(&mut self) -> Result<()> {
        self.counter -= 1;
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Connect 4 ".bold());
        let instructions = Title::from(Line::from(vec![
            " New Game ".into(),
            "<N>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - 95) / 2),
                Constraint::Percentage(95),
                Constraint::Percentage((100 - 95) / 2),
            ])
            .split(area);

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK)
            // .bg(Color::LightBlue)
            ;

        let mut contents = vec![
            Line::from(" "),
            Line::from(" "),
            Line::from(" "),
        ];
        let mut board: Vec<_> = format!("{:#?}", self.state)
            .lines()
            .map(|e| Line::from(e.to_string()))
            .collect();
        contents.append(&mut board);
        contents.append(&mut vec![
            Line::from(" "),
            Line::from(" "),
            Line::from(" "),
            Line::from(" "),
            Line::from(" "),
            Line::from(" "),
            Line::from(vec![
                "Enter column number (1-7) and press ".into(),
                "<Enter>".blue().bold(),
            ]),
            Line::from(" "),
        ]);
        let text_contents = Text::from(contents);

        let percent_x = 95;
        let lay = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1];

        Paragraph::new(text_contents)
            .centered()
            .block(block)
            .render(lay, buf);
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Game level: {}", args.level);

    let mut state = State {
        kind: StateKind::Live,
        player1: Player {
            name: "Adrian".to_string(),
            strategy: manual_strategy,
            // strategy: random_strategy,
        },
        player2: Player {
            name: "Bottie".to_string(),
            // strategy: random_strategy,
            // strategy: minimax_strategy3,
            strategy: minimax_strategy_level::<3>,
        },
        moves_player1: Vec::new(),
        moves_player2: Vec::new(),
        height: [0; 7],
    };


    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}


