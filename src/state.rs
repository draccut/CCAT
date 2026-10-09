#[derive(Debug)]
pub struct State {
    pub line_nr: u64,
    pub flag_nr: bool,
    pub print_binary: bool,
    pub remove_all_ansi: bool,
    pub html_color: bool,
    pub hex: bool,
    pub stop_at_line: u64,
    pub to_c_array: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            line_nr: 0,
            flag_nr: false,
            print_binary: false,
            remove_all_ansi: false,
            html_color: false,
            hex: false,
            stop_at_line: 0xFFFFFFFFFFFFF000,
            to_c_array: false,
        }
    }
}

