use ncurses::{getch, KEY_BACKSPACE, KEY_DOWN, KEY_END, KEY_ENTER, KEY_UP};

#[derive(Debug, Clone)]
pub enum KeyboardState {
    Exit,
    FinalSelection,
    Input,
    Noop,
}

#[derive(Clone)]
pub struct Keyboard {
    pub keyboard_state: KeyboardState,
    pub highlighted_selection: i32,
    pub input_buffer: String,

    max_selection: usize,
}

impl Keyboard {
    fn process_key(&mut self, key: i32) {
        self.keyboard_state = match key {
            KEY_ENTER | 10 => KeyboardState::FinalSelection,
            KEY_END | -1 => KeyboardState::Exit,
            KEY_BACKSPACE | 127 => {
                self.input_buffer.pop();
                KeyboardState::Input
            }
            KEY_UP => {
                let selection_is_within_bounds =
                    (self.highlighted_selection + 1) < (self.max_selection as i32);
                if selection_is_within_bounds {
                    self.highlighted_selection += 1
                }
                KeyboardState::Input
            }
            KEY_DOWN => {
                let selection_is_within_bounds = (self.highlighted_selection - 1) >= 0;
                if selection_is_within_bounds {
                    self.highlighted_selection -= 1
                }
                KeyboardState::Input
            }
            32..=126 => {
                let charified_char_code: char = char::from_u32(key as u32).unwrap();
                let formatted_string: String = format!("{}", charified_char_code);
                self.input_buffer.push_str(&formatted_string);
                KeyboardState::Input
            }
            // TODO: handle my edges
            _ => KeyboardState::Noop,
        };
    }

    pub fn listen(&mut self) {
        let key = getch();
        self.process_key(key);
    }
}

pub fn init_keyboard_handler(selections: &Vec<String>) -> Keyboard {
    Keyboard {
        keyboard_state: KeyboardState::Input,
        input_buffer: String::from(""),
        highlighted_selection: 0,
        max_selection: selections.len(),
    }
}
