use std::collections::HashMap;
use crate::cards::{Card, Deck, Face, Suite};
use crate::ui::UIInput;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum GameMode {
    Game,
    RenderCards,
}

impl GameMode {
    pub fn from_string(mode: &str) -> Result<GameMode, String> {
        match mode {
            "game" => Ok(GameMode::Game),
            "render_cards" => Ok(GameMode::RenderCards),
            _ => Err(format!("Invalid run mode: {}", mode)),
        }
    }
}

pub enum KlondikeDeckError {
    InvalidSuite,
    InvalidCardOrder,
}

pub struct Foundation {
    cards: Deck,
    suite: Suite,
}

pub struct KlondikeGame {
    pub(crate) game_mode: GameMode,
    pub(crate) stock: Deck,
    waste: Deck,
    foundations: HashMap<Suite, Foundation>,
    piles: [Deck; 7],
}

impl Foundation {
    pub fn take_cards(&mut self, count: usize) -> Vec<Card> {
        self.cards.take_cards(count)
    }

    pub fn add_card(&mut self, card: Card) -> Result<(), KlondikeDeckError> {
        if card.suite != self.suite {
            return Err(KlondikeDeckError::InvalidSuite);
        }

        let top_card = self.cards.cards.last();

        if let Some(top_card) = top_card {
            if top_card.face as u8 + 1 != card.face as u8 {
                return Err(KlondikeDeckError::InvalidCardOrder);
            }
        } else {
            if card.face != Face::Ace {
                return Err(KlondikeDeckError::InvalidCardOrder);
            }
        }

        self.cards.add_cards(vec![card]);

        Ok(())
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) -> Result<(), KlondikeDeckError> {
        for card in cards {
            self.add_card(card)?;
        }

        Ok(())
    }
}

impl KlondikeGame {
    pub fn new(game_mode: GameMode) -> KlondikeGame {
        let mut stock = Deck::new_standard_deck();

        let mut waste = Deck::new_empty_deck();

        let mut foundations: HashMap<Suite, Foundation> = HashMap::new();

        // Create foundations if game mode is Game...
        if game_mode == GameMode::Game {
            stock.shuffle();

            foundations.insert(Suite::Clubs, Foundation {
                cards: Deck::new_empty_deck(),
                suite: Suite::Clubs,
            });
            foundations.insert(Suite::Diamonds, Foundation {
                cards: Deck::new_empty_deck(),
                suite: Suite::Diamonds,
            });
            foundations.insert(Suite::Hearts, Foundation {
                cards: Deck::new_empty_deck(),
                suite: Suite::Hearts,
            });
            foundations.insert(Suite::Spades, Foundation {
                cards: Deck::new_empty_deck(),
                suite: Suite::Spades,
            });
        }

        let mut piles: [Deck; 7] = [
            Deck::new_empty_deck(),
            Deck::new_empty_deck(),
            Deck::new_empty_deck(),
            Deck::new_empty_deck(),
            Deck::new_empty_deck(),
            Deck::new_empty_deck(),
            Deck::new_empty_deck(),
        ];

        // File the piles in normal game mode...
        if game_mode == GameMode::Game {
            for i in 0..7 {
                let cards = stock.take_cards(i + 1);
                piles[i].add_cards(cards);
                piles[i].flip_top_card();
            }
        }

        KlondikeGame {
            game_mode,
            stock,
            waste,
            foundations,
            piles,
        }
    }

    pub fn send_input(&self, input: &UIInput) {
        //
    }
}