use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Copy, Clone)]
enum CellType {
    Live,
    Dead,
}

#[derive(Copy, Clone)]
struct Cordinates {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Cell {
    state: CellType,
    neighbours: i32,
    cordinates: Cordinates,
}
fn is_valid_index(x: i32, y: i32, number_of_collumns: i32, number_of_rows: i32) -> bool {
    if y < 0 || y > number_of_rows {
        return false;
    }
    if x < 0 || x > number_of_collumns {
        return false;
    }
    true
}

impl Cell {
    fn new(value: i32, cordinates: Cordinates) -> Cell {
        let state = match value {
            1 => CellType::Live,
            _ => CellType::Dead,
        };
        Cell {
            state,
            neighbours: 0,
            cordinates,
        }
    }

    fn get_neighbours(&mut self, game: &Vec<Vec<Cell>>) {
        let mut result = 0;
        for y in (self.cordinates.y as i32) - 1..(self.cordinates.y as i32) + 1 {
            for x in (self.cordinates.x as i32) - 1..(self.cordinates.x as i32) + 1 {
                if !is_valid_index(y, x, game.len() as i32, game[0].len() as i32) {
                    continue;
                };
                if y == self.cordinates.y as i32 && x == self.cordinates.x as i32 {
                    continue;
                };
                result += match &game[y as usize][x as usize].state {
                    CellType::Live => 1,
                    CellType::Dead => 0,
                }
            }
        }
        self.neighbours = result;
    }

    fn update(&mut self, game: &Vec<Vec<Cell>>) {
        self.get_neighbours(&game);
        match self.state {
            CellType::Live => {
                if self.neighbours != 2 && self.neighbours != 3 {
                    self.state = CellType::Dead
                }
            }
            CellType::Dead => {
                if self.neighbours == 3 {
                    self.state = CellType::Live
                }
            }
        }
    }
}
struct Game(pub Vec<Vec<Cell>>);

fn copy_game(vector: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut result = vec![];
    for row in vector {
        let mut new_row = vec![];
        for element in row {
            new_row.push(*element)
        }
        result.push(new_row)
    }
    result
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from("");
        for current_row in self.0.iter() {
            let mut row = String::from("");
            for element in current_row {
                row += match element.state {
                    CellType::Live => "1",
                    CellType::Dead => "0",
                };
            }
            row.push('\n');
            result.push_str(&row);
        }
        write!(f, "{result}")
    }
}

fn main() {
    let rows = 10;
    let collumns = 10;
    let mut rng = thread_rng();
    let mut game = vec![];
    for y in 0..rows {
        let mut row = vec![];
        for x in 0..collumns {
            let value: i32 = rng.gen_range(0..10);
            let cordinates = Cordinates { y, x };
            let cell = Cell::new(value, cordinates);
            row.push(cell)
        }
        game.push(row);
    }
    {
        let game_copy = copy_game(&game);
        let first_iteration = Game(game_copy);
        println!("{first_iteration}");
    }
    let mut game_copy = copy_game(&game);
    for y in 0..rows {
        for x in 0..collumns {
            game_copy[y][x].update(&game);
        }
    }
    let result = Game(game_copy);
    println!("{result}")
}
