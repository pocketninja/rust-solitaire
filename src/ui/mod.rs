pub mod terminal;


#[derive(Debug)]
pub enum UIError {
    UnableToCreateUI(String),
    Generic,
}

pub struct UIInput {
    pub key: char,
}

pub trait RendersKlondikeUI {
    // fn new() -> Result<Self, UIError> where Self: Sized;
    fn poll_for_input(&self) -> Result<UIInput, UIError>;
    fn render_menu(&self) -> Result<(), UIError>;
    fn render_game(&self) -> Result<(), UIError>;
}