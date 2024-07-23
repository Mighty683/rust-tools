mod bankpass;
mod utils;
use bankpass::bankpass_loop;
use utils::get_input;

fn main() {
    loop {
        println!("Choose a command, type help for list of commands:");
        let command: String = get_input();
        match command.trim() {
            "exit" => break,
            "bank" => bankpass_loop(),
            "help" => println!("Commands: bank, exit, help"),
            _ => println!("Invalid command!"),
        }
    }
}
