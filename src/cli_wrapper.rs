/*
    You can use this if you want to play on the CLI.
    Tho a much better GUI is coming soon.
*/

use crate::align_four_engine::{AlignFourEngine, Team};

pub struct CLIWrapper {
    game: AlignFourEngine,
}

impl CLIWrapper {
    pub fn default() -> Self {
        Self {
            game: AlignFourEngine::default(),
        }
    }

    pub fn run(mut self) {
        loop {
            self.print();
            println!();
            match self.game.check_win() {
                Team::Red | Team::Blue => {
                    println!("Les {} ont gagnés !!!", self.turn_symbol());
                    break;
                }
                Team::None => (),
            }
            loop {
                let sanitized_user_input = self.get_sanitized_user_input();
                match self.game.play_at(sanitized_user_input - 1) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("You cannot place anything in this column");
                        continue;
                    }
                }
                self.game.switch_turns();
                break;
            }
        }
    }

    fn print(&mut self) {
        println!(" 1 | 2 | 3 | 4 | 5 | 6 | 7 |");
        for y in 0..self.game.height() {
            let mut line = String::new();
            line.push(' ');
            for x in 0..self.game.width() {
                // dbg!(x, y, self.game.at(x, y));
                // dbg!(self.game.at(6, 4), self.game.at(0, 5));
                line.push(match self.game.at(x, y) {
                    Team::Red => 'X',
                    Team::Blue => 'O',
                    Team::None => ' ',
                });
                line.push_str(" | ");
            }
            println!("{}", line);
            println!("---+---+---+---+---+---+---+");
        }
    }
    fn turn_symbol(&self) -> char {
        if self.game.turn() == Team::Red {
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
