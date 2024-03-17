use crate::game::KlondikeGame;

pub mod terminal;


#[derive(Debug)]
pub enum UIError {
    UnableToCreateUI(String),
    NoInput,
    Generic,
}

pub struct UIInput {
    pub key: char,
}

pub trait RendersKlondikeUI {
    // fn new() -> Result<Self, UIError> where Self: Sized;
    fn poll_for_input(&self) -> Result<UIInput, UIError>;
    fn render_game(&mut self, game: &KlondikeGame) -> Result<(), UIError>;
    fn render_cards(&mut self, game: &KlondikeGame) -> Result<(), UIError>;

    fn shutdown(&self);
}