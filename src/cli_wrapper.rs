/*
    You can use this if you want to play on the CLI.
    Tho a much better GUI is coming soon.
*/

use crate::align_four_engine::{AlignFourEngine, Team};

pub struct CLIWrapper {
    engine: AlignFourEngine,
}

impl CLIWrapper {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            engine: AlignFourEngine::new(width, height),
        }
    }

    pub fn run(mut self) {
        loop {
            self.print();
            println!();
            // téma la gueule du pattern matching
            match self.engine.check_win() {
                Some((Team::Red | Team::Blue, _)) => {
                    println!("Les {} ont gagnés !!!", self.turn_symbol());
                    break;
                }
                Some((Team::Nothing, _)) => {
                    println!("C'est une égalité !!!");
                    break;
                }
                _ => (),
            }
            loop {
                let sanitized_user_input = self.get_sanitized_user_input();
                match self.engine.play_at(sanitized_user_input - 1) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("You cannot place anything in this column");
                        continue;
                    }
                }
                self.engine.switch_turns();
                break;
            }
        }
    }

    fn print(&mut self) {
        println!(
            "{}",
            (1..=self.engine.width())
                .map(|x| format!(" {}{}|", x, if x >= 10 { '\0' } else { ' ' }))
                .collect::<String>()
        );
        for y in 0..self.engine.height() {
            let mut line = String::new();
            line.push(' ');
            for x in 0..self.engine.width() {
                line.push(match self.engine.at(x, y) {
                    Team::Red => 'X',
                    Team::Blue => 'O',
                    Team::Nothing => ' ',
                });
                line.push_str(" | ");
            }
            println!("{}", line);
            println!(
                "{}",
                (1..=self.engine.width())
                    .map(|_| "---+")
                    .collect::<String>()
            )
        }
    }
    fn turn_symbol(&self) -> char {
        if self.engine.turn() == Team::Red {
            'X'
        } else {
            'O'
        }
    }

    fn get_sanitized_user_input(&self) -> usize {
        loop {
            println!(
                "[{}] >> Enter the column to play [1-{}] :",
                self.turn_symbol(),
                self.engine.width()
            );
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line from user");
            let trimed_input = input.trim();
            if trimed_input.len() > 2 {
                println!("You must enter max 2 digits\n");
                continue;
            }
            match trimed_input.parse::<usize>() {
                Ok(val) => {
                    if val > self.engine.width() || val < 1 {
                        println!(
                            "You must enter a digit between 1 and {}\n",
                            self.engine.width()
                        );
                        continue;
                    }
                    return val;
                }
                Err(_) => {
                    println!("You must enter a digit\n");
                    continue;
                }
            }
        }
    }
}
