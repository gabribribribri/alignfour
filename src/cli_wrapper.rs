/*
    You can use this if you want to play on the CLI.
    Tho a much better GUI is coming soon.
*/

use crate::align_four_engine::{AlignFourEngine, Team};

pub struct CLIWrapper {
    engine: AlignFourEngine,
}

impl CLIWrapper {
    pub fn default() -> Self {
        Self {
            engine: AlignFourEngine::default(),
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
        println!(" 1 | 2 | 3 | 4 | 5 | 6 | 7 |");
        for y in 0..self.engine.height() {
            let mut line = String::new();
            line.push(' ');
            for x in 0..self.engine.width() {
                // dbg!(x, y, self.game.at(x, y));
                // dbg!(self.game.at(6, 4), self.game.at(0, 5));
                line.push(match self.engine.at(x, y) {
                    Team::Red => 'X',
                    Team::Blue => 'O',
                    Team::Nothing => ' ',
                });
                line.push_str(" | ");
            }
            println!("{}", line);
            println!("---+---+---+---+---+---+---+");
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
                "[{}] >> Enter the column to play [1-7] :",
                self.turn_symbol()
            );
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line from user");
            let trimed_input = input.trim();
            if trimed_input.len() > 1 {
                println!("You must enter only one digit\n");
                continue;
            }
            match trimed_input.parse::<usize>() {
                Ok(val) => {
                    if val > 7 || val < 1 {
                        println!("You must enter a digit between 1 and 7\n");
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
