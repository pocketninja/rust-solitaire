use std::collections::HashMap;
use rand::Rng;
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
    deck: Deck,
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
        self.deck.take_cards(count)
    }

    pub fn add_card(&mut self, card: Card) -> Result<(), KlondikeDeckError> {
        if card.suite != self.suite {
            return Err(KlondikeDeckError::InvalidSuite);
        }

        let top_card = self.deck.cards.last();

        if let Some(top_card) = top_card {
            if top_card.face as u8 + 1 != card.face as u8 {
                return Err(KlondikeDeckError::InvalidCardOrder);
            }
        } else {
            if card.face != Face::Ace {
                return Err(KlondikeDeckError::InvalidCardOrder);
            }
        }

        self.deck.add_cards(vec![card]);

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
        stock.name = String::from("Stock");

        let mut waste = Deck::new_empty_deck();
        waste.name = String::from("Waste");

        let mut foundations: HashMap<Suite, Foundation> = HashMap::new();

        // Create foundations if game mode is Game...
        if game_mode == GameMode::Game {
            stock.shuffle();

            let mut deck = Deck::new_empty_deck();
            deck.name = String::from("Clubs");
            foundations.insert(Suite::Clubs, Foundation { deck, suite: Suite::Clubs });

            let mut deck = Deck::new_empty_deck();
            deck.name = String::from("Diamonds");
            foundations.insert(Suite::Diamonds, Foundation { deck, suite: Suite::Diamonds });

            let mut deck = Deck::new_empty_deck();
            deck.name = String::from("Hearts");
            foundations.insert(Suite::Hearts, Foundation { deck, suite: Suite::Hearts });

            let mut deck = Deck::new_empty_deck();
            deck.name = String::from("Spades");
            foundations.insert(Suite::Spades, Foundation { deck, suite: Suite::Spades });
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
                piles[i].flip_top_cards(1);
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

    pub fn send_input(&mut self, input: &UIInput) {
        match self.game_mode {
            GameMode::Game => {
                match input.key {
                    _ => println!("Unknown game mode input: {}", input.key)
                }
            }
            GameMode::RenderCards => {
                match input.key {
                    'f' => self.flip_stock(),
                    'g' => self.stock.flip_top_cards(3),
                    'r' => {
                        // one random card...
                        let mut rng = rand::thread_rng();
                        let i = rng.gen_range(0..self.stock.cards.len());
                        let mut card = self.stock.cards.get_mut(i).unwrap();
                        card.face_up = !card.face_up;
                    }
                    _ => println!("Unknown render cards mode input: {}", input.key)
                }
            }
        }
    }
    fn flip_stock(&mut self) {
        for i in 0..self.stock.cards.len() {
            self.stock.cards[i].face_up = !self.stock.cards[i].face_up;
        }
    }
}