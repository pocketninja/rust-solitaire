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
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Wrap};
use ratatui::widgets::block::Title;
use crate::cards::{Card, Deck, Face, Suite};
use crate::math::Vector;

const CARD_SIZE: Vector = Vector { x: 8, y: 7 };
const DECK_SIZE: Vector = Vector { x: CARD_SIZE.x + 2, y: CARD_SIZE.y + 2 };

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

    fn render_all_cards(&mut self, game: &KlondikeGame) -> Result<(), UIError> {
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

    render_deck(frame, &game.stock, Vector { x, y });

    x += DECK_SIZE.x;

    if x + DECK_SIZE.x > frame_rect.width as i32 {
        x = 0;
        y += CARD_SIZE.y;
    }

    render_deck(frame, &game.stock, Vector { x, y });
}

fn render_card(mut frame: &mut Frame, card: &Card, position: Vector) {
    let frame_rect = frame.size();

    let x = position.x as u16;
    let y = position.y as u16;

    if x > frame_rect.width || y > frame_rect.height {
        return;
    }

    let mut card_text;

    if card.face_up {
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

        card_text = format!("{}{}", face, suite);
        // Now center align it in a full line the size of the card.
        card_text = format!("{}✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧", card_text);
        // Dodgy trick to get more space. We want to try and paint over the background, if we are
        // overlaying other cards/etc.
        card_text += "\n✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧";
    } else {
        card_text = String::from("✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧✧");
    }

    let mut width = CARD_SIZE.x as u16;
    let mut height = CARD_SIZE.y as u16;

    if x + width > frame_rect.width {
        width = frame_rect.width - x;
    }

    if y + height > frame_rect.height {
        height = frame_rect.height - y;
    }

    let card_rect = Rect::new(
        x,
        y,
        width,
        height,
    );

    let block_style: Style = match card.face_up {
        false => Style::new().blue().on_white().bold(),
        true => match card.suite {
            Suite::Hearts => Style::new().red().on_white().bold(),
            Suite::Diamonds => Style::new().red().on_white().bold(),
            Suite::Clubs => Style::new().black().on_white().bold(),
            Suite::Spades => Style::new().black().on_white().bold(),
        }
    };

    let card_block = Block::default()
        .style(block_style)
        .borders(Borders::ALL)
        .border_set(border::THICK);

    let card_text = Paragraph::new(card_text)
        .wrap(Wrap { trim: true })
        // .alignment(Alignment::Center)
        .block(card_block);

    frame.render_widget(card_text, card_rect);
}

fn render_deck(mut frame: &mut Frame, deck: &Deck, position: Vector) {
    let frame_rect = frame.size();

    let mut x = position.x;
    let mut y = position.y;

    let deck_block = Block::default()
        .title(deck.name.as_str())
        .borders(Borders::ALL)
        .border_set(border::THICK);

    let deck_rect = Rect::new(
        x as u16,
        y as u16,
        DECK_SIZE.x as u16,
        DECK_SIZE.y as u16,
    );

    frame.render_widget(deck_block, deck_rect);

    let mut card_x = position.x + (DECK_SIZE.x - CARD_SIZE.x) / 2;
    let mut card_y = position.y + (DECK_SIZE.y - CARD_SIZE.y) / 2;

    // for card in deck.cards.iter() {
    for i in 0..deck.cards.len() {
        let card = &deck.cards[i];

        render_card(frame, card, Vector { x: card_x, y: card_y });

        // If this card is face up, give enough space for the "card title"
        if card.face_up {
            card_y += 2;
        } else {
            // If it's not face up, and the next _is_, give a bump so we can see there are cards
            // "under" the next one.
            let next_index = i + 1;
            if next_index < deck.cards.len() && deck.cards[next_index].face_up {
                card_y += 1;
            }
        }
    }
}