use crate::utils::*;

pub fn bankpass_loop() {
    clear_screen();
    println!("Enter your bank password:");
    let password = get_hidden_inut();
    let mut index_array: Vec<usize> = Vec::new();
    loop {
        clear_screen();
        println!("Enter character index to reveal, or type q to exit:");
        let new_index_string = get_input();
        if (new_index_string.trim() == "q") {
            break;
        }
        let new_index_number: usize = new_index_string.trim().parse().unwrap();
        index_array.push(new_index_number - 1);
        println!(
            "Password characters: {}",
            password
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if index_array.contains(&i) {
                        c
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        );
    }
}
