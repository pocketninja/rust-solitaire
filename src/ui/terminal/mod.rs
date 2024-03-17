use crate::game::KlondikeGame;
use crate::ui::{RendersKlondikeUI, UIError, UIInput};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{Frame, prelude::{CrosstermBackend, Stylize, Terminal}, widgets::Paragraph};
use std::io::{stdout, Stdout};
use std::result::Result;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::block::Title;
use crate::cards::{Card, Face, Suite};
use crate::math::Vector;

const CARD_SIZE: Vector = Vector { x: 6, y: 4 };

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
    pub fn new() -> Result<Self, UIError> {
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
    fn poll_for_input(&self) -> Result<UIInput, UIError> {
        if event::poll(std::time::Duration::from_millis(1)).expect("input poll failed") {
            if let event::Event::Key(
                KeyEvent { code: KeyCode::Char(c), .. }
            ) = event::read().expect("input read failed") {
                return Ok(UIInput { key: c });
            }
        }

        Err(UIError::NoInput)
    }

    fn render_game(&mut self, game: &KlondikeGame) -> Result<(), UIError> {
        clear_terminal(&mut self.terminal).expect("Could not clear terminal");
        Ok(())
    }

    fn render_cards(&mut self, game: &KlondikeGame) -> Result<(), UIError> {
        clear_terminal(&mut self.terminal).expect("Could not clear terminal");
        self.terminal.draw(|frame| {
            render_all_cards(frame, game);
        }).expect("Could not render the cards");
        Ok(())
    }

    fn shutdown(&self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}

fn clear_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), String> {
    terminal.clear().expect("Failed to clear terminal");
    Ok(())
}

fn render_all_cards(frame: &mut Frame, game: &KlondikeGame) {
    let frame_rect = frame.size();

    let mut x = 0;
    let mut y = 0;

    for card in game.stock.cards.iter() {
        render_card(frame, card, Vector { x, y });
        x += CARD_SIZE.x;

        // Wrap to next line...
        if x + CARD_SIZE.x > frame_rect.width as i32 {
            x = 0;
            y += CARD_SIZE.y;
        }

        // No more space, bail...
        if y + CARD_SIZE.y > frame_rect.height as i32 {
            break;
        }
    }
}

fn render_card(mut frame: &mut Frame, card: &Card, position: Vector) {
    let face = match card.face {
        Face::Ace => "A",
        Face::Two => "2",
        Face::Three => "3",
        Face::Four => "4",
        Face::Five => "5",
        Face::Six => "6",
        Face::Seven => "7",
        Face::Eight => "8",
        Face::Nine => "9",
        Face::Ten => "10",
        Face::Jack => "J",
        Face::Queen => "Q",
        Face::King => "K",
    };

    let suite = match card.suite {
        Suite::Hearts => "♥",
        Suite::Diamonds => "♦",
        Suite::Clubs => "♣",
        Suite::Spades => "♠",
    };

    let card_rect = Rect::new(
        position.x as u16,
        position.y as u16,
        CARD_SIZE.x as u16,
        CARD_SIZE.y as u16,
    );

    let card_block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::THICK);

    let card_text = format!("{}{}", face, suite);
    let card_text = Paragraph::new(card_text).block(card_block);
    frame.render_widget(card_text, card_rect);
}