use ncurses::*;

fn disable_cursor() {
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn disable_input_echo() {
    noecho();
}

fn enable_byte_by_byte_input() {
    raw();
}

fn enable_extended_keyboard() {
    keypad(stdscr(), true);
}

fn open_tty() -> *mut libc::FILE {
    unsafe {
        let tty_filename = std::ffi::CString::new("/dev/tty").unwrap();
        let tty = libc::fopen(tty_filename.as_ptr(), &('r' as libc::c_char));
        tty
    }
}

fn open_std_error() -> *mut libc::FILE {
    unsafe {
        let std_error = libc::fdopen(libc::STDERR_FILENO, &('w' as libc::c_char));
        std_error
    }
}

fn lay_terminal_pipe() {
    let std_error = open_std_error();
    let tty = open_tty();

    // NOTE:
    // We display the tui in stderror to free up stdout.
    // Input is thrown through the tty so we can pipe into stdin and still accept input.
    newterm(None, std_error, tty);
}

pub fn display_selections(selections: &Vec<String>, highlighted_selection: i32) {
    selections
        .iter()
        .enumerate()
        .for_each(|(index, selection)| {
            let formatted_string = format!("{}\n", selection);

            if index as i32 == highlighted_selection {
                attr_on(A_BOLD());
                addstr(&formatted_string);
                attr_off(A_BOLD());
            } else {
                addstr(&formatted_string);
            }
        });
}

pub fn display_input(input: &String) {
    addstr(&format!("{}\n", &input));
}

pub fn exit () {
    endwin();
}

pub fn clear_screen() {
    clear();
}

pub fn spawn() {
    lay_terminal_pipe();

    enable_byte_by_byte_input();
    enable_extended_keyboard();

    disable_cursor();
    disable_input_echo();
}
