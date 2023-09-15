use std::fs::File;
use std::io::{self, Read, Write};
use rand::Rng;
use prettytable::{Table, Row, Cell};
use ansi_term::Colour;

fn main() {
    println!("User Data Application");

    let mut user_data = load_user_data();
    let mut removed_users = Vec::new(); // Maintain a list of temporarily removed users

    loop {
        println!("Choose an action:");
        println!("1. Add User");
        println!("2. Remove User");
        println!("3. View User Data");
        println!("4. Play Game");
        println!("5. Exit");

        let choice = read_input_as_integer();

        match choice {
            Ok(1) => {
                let user = add_user();
                user_data.push(user);
            }
            Ok(2) => {
                remove_user(&mut user_data, &mut removed_users);
            }
            Ok(3) => {
                view_user_data(&user_data);
            }
            Ok(4) => {
                let player_won = play_game(&mut user_data, &mut removed_users);
                if player_won {
                    println!("You won!");
                } else {
                    println!("The imposter won!");
                }
            }
            Ok(5) => {
                save_user_data(&user_data);
                break;
            }
            _ => {
                println!("Invalid choice. Please select a valid option.");
            }
        }

        // Check if the game should end
        if user_data.len() == 1 && user_data[0].is_imposter() {
            println!("The imposter won!");
            break;
        }
    }
}

#[derive(Clone, PartialEq)]
struct User {
    name: String,
    age: String,
}

impl User {
    fn is_imposter(&self) -> bool {
        self.age == "Imposter"
    }
}

fn load_user_data() -> Vec<User> {
    let mut user_data = Vec::new();

    if let Ok(mut file) = File::open("userdata.txt") {
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read userdata.txt");

        for line in content.lines() {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let age = parts[1].to_string();
                user_data.push(User { name, age });
            }
        }
    }

    user_data
}

fn read_input_as_integer() -> Result<u32, std::num::ParseIntError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse()
}

fn add_user() -> User {
    println!("What is the user's name?");
    let name = read_input_as_string();

    println!("How old is the user?");
    let age = read_input_as_string();

    User { name, age }
}

fn read_input_as_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn remove_user(user_data: &mut Vec<User>, removed_users: &mut Vec<User>) {
    if user_data.is_empty() {
        println!("There are no users to remove.");
        return;
    }

    println!("Enter the index of the user to remove:");
    view_user_data(&user_data);

    let index = read_input_as_integer();

    match index {
        Ok(idx) if idx < user_data.len() as u32 => {
            let removed_user = user_data.remove(idx as usize);
            removed_users.push(removed_user);
            println!("User removed.");
        }
        _ => {
            println!("Invalid index. No user removed.");
        }
    }
}

fn view_user_data(user_data: &Vec<User>) {
    if user_data.is_empty() {
        println!("User data is empty.");
    } else {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Name").style_spec("bF"),
            Cell::new("Age").style_spec("bF"),
        ]));

        for user in user_data {
            table.add_row(Row::new(vec![
                Cell::new(&user.name),
                Cell::new(&user.age),
            ]));
        }

        table.printstd();
    }
}

fn save_user_data(user_data: &Vec<User>) {
    let mut file = File::create("userdata.txt").expect("Failed to create userdata.txt");
    for user in user_data {
        write!(file, "{};{}\n", user.name, user.age).expect("Failed to write to userdata.txt");
    }
}

fn play_game(user_data: &mut Vec<User>, removed_users: &mut Vec<User>) -> bool {
    let mut rng = rand::thread_rng();
    let imposter_index = rng.gen_range(0..user_data.len());
    let mut remaining_users: Vec<usize> = (0..user_data.len()).collect();

    let mut player_won = false;

    loop {
        view_user_data(user_data);

        println!("Guess the imposter! Enter the index of the user you think is the imposter:");

        let guess = read_input_as_integer();

        match guess {
            Ok(index) if index < user_data.len() as u32 => {
                if index == imposter_index as u32 {
                    let imposter_name = user_data[index as usize].name.to_string();
                    println!(
                        "Congratulations! You guessed correctly. The imposter was {}.",
                        Colour::Green.bold().paint(imposter_name)
                    );
                    // Temporarily remove the user and add them to the removed_users list
                    let removed_user = user_data.remove(index as usize);
                    removed_users.push(removed_user);
                    player_won = true;
                    break;
                } else {
                    println!(
                        "Sorry, your guess is incorrect. Try Again."
                    );
                    let remove_index = remaining_users.iter().position(|&x| x == index as usize);
                    if let Some(remove_index) = remove_index {
                        remaining_users.remove(remove_index);
                    }
                }
            }
            _ => {
                println!("Invalid index. Please enter a valid user index.");
            }
        }

        if remaining_users.is_empty() {
            println!("No more users left to guess from. The imposter won!");
            break;
        }
    }

    player_won
}
