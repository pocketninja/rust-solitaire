

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Suite {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Face {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

pub struct Card {
    pub suite: Suite,
    pub face: Face,
    pub face_up: bool,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new_empty_deck() -> Deck {
        Deck { cards: Vec::new() }
    }

    pub fn new_standard_deck() -> Deck {
        let mut cards = Vec::new();
        for suite in vec![Suite::Hearts, Suite::Diamonds, Suite::Clubs, Suite::Spades] {
            for face in vec![
                Face::Ace, Face::Two, Face::Three, Face::Four, Face::Five, Face::Six, Face::Seven,
                Face::Eight, Face::Nine, Face::Ten, Face::Jack, Face::Queen, Face::King,
            ] {
                cards.push(Card { suite, face, face_up: false });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn take_cards(&mut self, count: usize) -> Vec<Card> {
        let mut taken = Vec::new();
        for _ in 0..count {
            if let Some(card) = self.cards.pop() {
                taken.push(card);
            }
        }
        taken
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) {
        self.cards.extend(cards);
    }

    pub fn flip_top_card(&mut self) {
        if let Some(card) = self.cards.last_mut() {
            card.face_up = true;
        }
    }

}