use std::io;
use termion::input::TermRead;

pub fn get_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    use std::io::{stdout, Write};
    stdout().flush().unwrap();
}

pub fn get_hidden_inut() -> String {
    let stdin: io::Stdin = io::stdin();
    let mut stdin: io::Stdin = stdin;
    let stdout: io::Stdout = io::stdout();
    let mut stdout: io::Stdout = stdout;
    let input_password: Result<Option<String>, io::Error> = TermRead::read_passwd(&mut stdin, &mut stdout);
    let passwd: String = input_password.unwrap().unwrap();
    passwd
}
