pub enum AlignFourError {
    ColumnFull,
}

type Cell = (isize, isize);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Team {
    Red,
    Blue,
    Nothing,
}

pub struct AlignFourEngine {
    grid: Vec<Team>,
    width: usize,
    height: usize,
    turn: Team,
}

impl AlignFourEngine {
    // Constructors
    pub fn default() -> Self {
        Self {
            grid: vec![Team::Nothing; 7 * 6],
            width: 7,
            height: 6,
            turn: Team::Blue,
        }
    }

    #[allow(dead_code)]
    pub fn from_grid(grid_str: &str) -> Self {
        let mut grid = Vec::new();
        for c in grid_str.chars() {
            match c {
                'X' => grid.push(Team::Red),
                'O' => grid.push(Team::Blue),
                '-' => grid.push(Team::Nothing),
                _ => (),
            }
        }
        Self {
            grid,
            width: 7,
            height: 6,
            turn: Team::Blue,
        }
    }

    #[deprecated]
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![Team::Nothing; width * height],
            width,
            height,
            turn: Team::Blue,
        }
    }

    // Getters \ Setters
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn turn(&self) -> Team {
        self.turn
    }

    pub fn at(&self, x: usize, y: usize) -> Team {
        self.grid[y * self.width + x]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Team {
        &mut self.grid[y * self.width + x]
    }

    pub fn grid(&mut self) -> &mut Vec<Team> {
        &mut self.grid
    }

    fn is_in_grid(&self, coo: &Cell) -> bool {
        coo.0 >= 0 && coo.1 >= 0 && coo.0 < self.width as isize && coo.1 < self.height as isize
    }

    // Others
    pub fn switch_turns(&mut self) {
        self.turn = match self.turn {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
            _ => panic!("Absolutly not supposed to have a 'None' here"),
        }
    }

    pub fn play_at(&mut self, x: usize) -> Result<usize, AlignFourError> {
        for y in (0..self.height).rev() {
            match self.at(x, y) {
                Team::Nothing => {
                    *self.at_mut(x, y) = self.turn;
                    return Ok(y);
                }
                _ => continue,
            }
        }

        return Err(AlignFourError::ColumnFull);
    }

    pub fn check_win(&self) -> Option<Team> {
        /* It is very weird how I am at the same time horrified by what I just wrote and amazingly proud of myself for the masterpiece of engineering that THIS is */

        type Pattern = [Cell; 4];
        let start_point: Pattern = [
            (0, -1),
            (-1, 0),
            (self.height as isize - 3, -1),
            (self.width as isize + self.height as isize - 3, -1),
        ]; // one less/more that the actual start
        let next_line: Pattern = [(1, 0), (0, 1), (1, 0), (-1, 0)];
        let next_cell: Pattern = [(0, 1), (1, 0), (1, 1), (-1, 1)];
        let cell_repeats: [isize; 4] = [6, 7, 6, 6];
        let line_repeats: [isize; 4] = [7, 6, 7, 7];

        for strategy in 0..4usize {
            for line_repeat in 0..line_repeats[strategy] {
                let mut current_cell: Cell = (
                    start_point[strategy].0 + next_line[strategy].0 * line_repeat,
                    start_point[strategy].1 + next_line[strategy].1 * line_repeat,
                );
                let mut suite_team: Team = Team::Nothing;
                let mut longest: u8 = 0;

                for _cell_repeat in 0..cell_repeats[strategy] {
                    current_cell.0 += next_cell[strategy].0;
                    current_cell.1 += next_cell[strategy].1;
                    if !self.is_in_grid(&current_cell) {
                        continue;
                    }
                    let team_at_current = self.at(current_cell.0 as usize, current_cell.1 as usize);
                    if team_at_current == suite_team && team_at_current != Team::Nothing {
                        longest += 1;
                        if longest >= 4 {
                            return Some(team_at_current);
                        }
                    } else {
                        longest = 1;
                    }
                    suite_team = team_at_current;
                }
            }
        }

        for cell in self.grid.iter() {
            match cell {
                Team::Nothing => return None,
                _ => (),
            }
        }
        return Some(Team::Nothing);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let engine = AlignFourEngine::from_grid(
            r"
                -------
                -------
                ---X---
                ---X---
                ---X---
                ---X---
            ",
        );
        assert_eq!(engine.check_win(), Some(Team::Red));
    }

    #[test]
    fn test_2() {
        let engine = AlignFourEngine::from_grid(
            r"
                -------
                -------
                -------
                -------
                -------
                XXXX---
            ",
        );
        assert_eq!(engine.check_win(), Some(Team::Red));
    }
    #[test]
    fn test_3() {
        let engine = AlignFourEngine::from_grid(
            r"
                -------
                -------
                ------X
                ------X
                ------X
                ------X
            ",
        );
        assert_eq!(engine.check_win(), Some(Team::Red));
    }
    #[test]
    fn test_4() {
        let engine = AlignFourEngine::from_grid(
            r"
                -------
                -------
                ----X--
                ---X---
                --X----
                -X-----
            ",
        );
        assert_eq!(engine.check_win(), Some(Team::Red));
    }
    #[test]
    fn test_5() {
        let engine = AlignFourEngine::from_grid(
            r"
                XXXX---
                -------
                -------
                -------
                -------
                -------
            ",
        );
        assert_eq!(engine.check_win(), Some(Team::Red));
    }
    #[test]
    fn test_6() {
        let engine = AlignFourEngine::from_grid(
            r"
                -XXXXXX
                --X----
                ---X---
                ----X--
                -------
                -------
            ",
        );
        assert_eq!(engine.check_win(), Some(Team::Red));
    }
}
