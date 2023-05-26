mod keyboard;
mod tui;

use keyboard::KeyboardState;
use std::{io, process};

fn get_stdin() -> Vec<String> {
    let stdin = io::stdin();

    stdin.lines().map(|line| line.unwrap()).collect()
}

fn start() {
    tui::spawn();

    // Dependents
    let selections = get_stdin();

    let mut keyboard_handler = keyboard::init_keyboard_handler(&selections);

    loop {
        match keyboard_handler.keyboard_state {
            KeyboardState::Exit => {
                tui::clear_screen();
                process::exit(0);
            }
            KeyboardState::FinalSelection => {
                tui::exit();

                let selection_index = keyboard_handler.highlighted_selection.clone() as usize;
                let final_selection = selections.get(selection_index).unwrap();
                print!("{}", final_selection);
                break;
            }
            KeyboardState::Input => {
                tui::clear_screen();
                tui::display_selections(&selections, keyboard_handler.highlighted_selection);
                
                // TODO: revisit how we want to delimit
                let delimiter = "============".to_owned(); 
                tui::display_input(&delimiter);
                // TODO: add some sort of input matching that ties into the selection highlight
                tui::display_input(&keyboard_handler.input_buffer);
            }
            // TODO: handle me
            _ => {}
        }

        keyboard_handler.listen();
    }
}

fn main() {
    start();
}
