pub enum AlignFourError {
    ColumnFull,
}

type Cell = (isize, isize);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Team {
    Red,
    Blue,
}

pub struct AlignFourEngine {
    grid: Vec<Option<Team>>,
    pub width: usize,
    pub height: usize,
    turn: Team,
}

impl AlignFourEngine {
    pub fn default() -> Self {
        Self {
            grid: vec![None; 7 * 6],
            width: 7,
            height: 6,
            turn: Team::Blue,
        }
    }
    #[deprecated]
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![None; width * height],
            width,
            height,
            turn: Team::Blue,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<Team> {
        self.grid[y * self.width + x]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Option<Team> {
        &mut self.grid[y * self.width + x]
    }

    pub fn switch_turns(&mut self) {
        self.turn = match self.turn {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        }
    }

    pub fn play_at(&mut self, x: usize) -> Result<usize, AlignFourError> {
        // println!("Call from play_at !");
        // dbg!(&self.grid);
        for y in (0..self.height).rev() {
            match self.at(x, y) {
                Some(_) => continue,
                None => {
                    *self.at_mut(x, y) = Some(self.turn);
                    return Ok(y);
                }
            }
        }

        return Err(AlignFourError::ColumnFull);
    }

    pub fn check_win(&self) -> Option<Team> {
        /**
         *
         * It is very weird how I am at the same time
         * horrified by what I just wrote and
         * amazingly proud of myself for the masterpiece of engineering
         * that THIS is
         *
         */

        type Pattern = [Cell; 4];
        let start_point: Pattern = [(-1, 0), (0, -1), (-3, -1), (self.width as isize + 2, -1)]; // one less/more that the actual start
        let next_line: Pattern = [(1, 0), (0, 1), (1, 0), (-1, 0)];
        let cell_repeats: [isize; 4] = [6, 7, 6, 6];
        let line_repeats: [isize; 4] = [7, 6, 7, 7];
        let next_cell: Pattern = [(0, 1), (1, 0), (1, 1), (-1, 1)];

        for strategy in 0..4usize {
            for line_repeat in 0..line_repeats[strategy] {
                let mut current_cell: Cell = (
                    start_point[strategy].0 + next_line[strategy].0 * line_repeat,
                    start_point[strategy].1 + next_line[strategy].1 * line_repeat,
                );
                let mut suite_color: Option<Team> = None;
                let mut longest: u8 = 0;

                for cell_repeat in 0..cell_repeats[strategy] {
                    current_cell.0 += next_cell[strategy].0;
                    current_cell.1 += next_cell[strategy].1;

                    if !self.is_in_grid(&current_cell) {
                        continue;
                    }

                    match self.at(current_cell.0 as usize, current_cell.1 as usize) {
                        Some(cell_team) => match suite_color {
                            Some(suite_team) => {
                                if cell_team == suite_team {
                                    if longest >= 4 {
                                        return Some(suite_team);
                                    } else {
                                        longest += 1
                                    }
                                } else {
                                    longest = 1;
                                }
                            }
                            None => longest = 1,
                        },
                        None => longest = 0,
                    }
                    suite_color = self.at(current_cell.0 as usize, current_cell.1 as usize);
                }
            }
        }

        return None;
    }

    fn is_in_grid(&self, coo: &Cell) -> bool {
        coo.0 > 0 && coo.1 > 0 && coo.0 < self.width as isize && coo.1 < self.height as isize
    }
}
