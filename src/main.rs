use std::thread::sleep;
use crate::game::GameMode;
use crate::ui::{RendersKlondikeUI, UIError};

mod cards;
mod game;
mod ui;
mod math;


fn main() {
    let game_args: Vec<String> = std::env::args().collect();

    let game_mode = if game_args.iter().count() > 1 {
        match GameMode::from_string(game_args[1].as_str()) {
            Ok(mode) => mode,
            Err(error) => panic!("{:?}", error),
        }
    } else {
        GameMode::Game
    };

    println!("Hello, klondike!");
    println!("Run mode: {:?}", game_mode);

    for i in 0..2 {
        println!("{}...", 2 - i);
        sleep(std::time::Duration::from_secs(1));
    }

    let mut game = game::KlondikeGame::new(game_mode);
    let ui = ui::terminal::UI::new();

    match ui {
        Ok(_) => {}
        Err(error) => panic!("{:?}", error)
    }

    let mut ui = ui.unwrap();

    let frame_delay = std::time::Duration::from_millis(1000 / 20);

    loop {
        let input = ui.poll_for_input();

        match input {
            Ok(_) => {
                let input = input.unwrap();

                game.send_input(&input);

                if input.key == 'q' {
                    break;
                }
            }
            Err(UIError::NoInput) => {}
            Err(error) => panic!("{:?}", error)
        }

        let render = match game.game_mode {
            GameMode::Game => ui.render_game(&game),
            GameMode::RenderCards => ui.render_all_cards(&game),
        };

        match render {
            Ok(_) => {}
            Err(error) => panic!("{:?}", error)
        }

        sleep(frame_delay);
    }

    ui.shutdown();

    println!("Done!");
}
