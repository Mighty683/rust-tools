use std::io;
use termion::input::TermRead;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

pub fn get_hidden_inut() -> String {
    let stdin = io::stdin();
    let mut stdin = stdin;
    let stdout = io::stdout();
    let mut stdout = stdout;
    let input_password = TermRead::read_passwd(&mut stdin, &mut stdout);
    let passwd = input_password.unwrap().unwrap();
    passwd
}
