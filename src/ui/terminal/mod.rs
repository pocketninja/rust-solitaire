use crate::game::KlondikeGame;
use crate::ui::{RendersKlondikeUI, UIError};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Stdout};
use std::result::Result;

pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    // game: KlondikeGame,
}

fn make_terminal() -> std::io::Result<Terminal<CrosstermBackend<Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

impl UI {
    pub(crate) fn new() -> Result<Self, UIError> {
        let terminal = make_terminal();

        match terminal {
            Ok(_) => {}
            Err(error) => return Err(UIError::UnableToCreateUI(error.to_string()))
        }

        Ok(UI {
            terminal: terminal.unwrap()
        })
    }
}

impl RendersKlondikeUI for UI {
    fn poll_for_input(&self) -> Result<(), UIError> {
        todo!()
    }

    fn render_menu(&self) -> Result<(), UIError> {
        Ok(())
    }

    fn render_game(&self) -> Result<(), UIError> {
        Ok(())
    }
}
