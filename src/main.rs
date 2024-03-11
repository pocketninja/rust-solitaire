mod cards;
mod game;
mod ui;

fn main() {
    println!("Hello, klondike!");

    let mut game = game::KlondikeGame::new();
    let ui = ui::terminal::UI::new();

    match ui {
        Ok(_) => {}
        Err(error) => panic!("{:?}", error)
    }

    println!("Done!");
}
