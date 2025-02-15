mod bankpass;
mod pomodoro;
mod utils;

use bankpass::bankpass_loop;
use pomodoro::pomodoro_loop;
use utils::get_input;

fn main() {
    loop {
        println!("Choose a command, type help for list of commands:");
        let command: String = get_input();
        match command.trim() {
            "exit" => break,
            "bank" => bankpass_loop(),
            "pomodoro" => pomodoro_loop(),
            "help" => list_commands(),
            _ => {
                println!("Invalid command!");
                list_commands();
            }
        }
    }
}

fn list_commands() {
    println!("Commands: bank, exit, pomodoro, help");
}