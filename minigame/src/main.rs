use std::io;
use rand::Rng;

fn main() {
    loop {
        println!("Choose the hand");
        println!("1 - Rock, 2 - Paper, 3 - Scissors, 4 - End the game");
        let bot_hand: u8 = rand::thread_rng().gen_range(1..=3); // генерация случайного выбора
    
        // обработка пользовательского ввода
        let mut user_hand = String::new();
        io::stdin()
            .read_line(&mut user_hand)
            .expect("Failed to read line");
        
        // исключение неверного ввода
        let user_hand: u8 = match user_hand.trim().parse() {
            Ok(user_hand) => user_hand,
            Err(e) => {
                println!("Error - {}", e);
                continue;
            },
        };

        // выход из программы
        if user_hand == 4 {
            println!("Goodbye!");
            break;
        }

        choose(user_hand, bot_hand); // функция сверки выбора
        println!("--------")


    }
}

fn choose (user_hand: u8, bot_hand: u8) {
    if user_hand == 1 {
        match bot_hand {
            1 => {
                println!("Your opponent have scissors");
                println!("You win");
            },
            2 => {
                println!("Your opponent have paper");
                println!("You lose");
            },
            3 => {
                println!("Your opponent have rock too");
                println!("Draw!");
            }
            _ => print!("Error")
        }
    } else if user_hand == 2 {
        match bot_hand {
            1 => {
                println!("Your opponent have rock");
                println!("You win")
            },
            2 => {
                println!("Your opponent have paper too");
                println!("Draw!")
            },
            3 => {
                println!("Your opponent have scissors");
                println!("You lose")
            }
            _ => print!("Error")
        }
    } else {
        match bot_hand {
            1 => {
                println!("Your opponent have rock");
                println!("You lose")
            },
            2 => {
                println!("Your opponent have paper");
                println!("You win!")
            },
            3 => {
                println!("Your opponent have scissors too");
                println!("Draw!")
            }
            _ => print!("Error")
        }
    }
    // if user_hand == 1 && bot_hand == 3 {
    //     println!("Your opponent have scissors");
    //     println!("You win")
    // } else if user_hand == 1 && bot_hand == 2 {
    //     println!("Your opponent have paper");
    //     println!("You lose")
    // } else if user_hand == 1 && bot_hand == 1 {
    //     println!("Your opponent have rock too");
    //     println!("Draw!")
    // } else if user_hand == 2 && bot_hand == 3 {
    //     println!("Your opponent have scissors");
    //     println!("You lose")
    // } else if user_hand == 2 && bot_hand == 2 {
    //     println!("Your opponent have paper too");
    //     println!("Draw")
    // } else if user_hand == 2 && bot_hand == 1 {
    //     println!("Your opponent have rock");
    //     println!("You win")
    // } else if user_hand == 3 && bot_hand == 3 {
    //     println!("Your opponent have scissors too");
    //     println!("Draw!")
    // } else if user_hand == 3 && bot_hand == 2 {
    //     println!("Your opponent have paper");
    //     println!("You win!")
    // } else if user_hand == 3 && bot_hand == 1 {
    //     println!("Your opponent have rock");
    //     println!("You lose")
    // }
}
