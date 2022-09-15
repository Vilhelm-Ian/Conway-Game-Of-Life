use macroquad::prelude::*;
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
    if y < 0 || y >= number_of_rows {
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

#[macroquad::main("BasicShapes")]
async fn main() {
    let rows = 20;
    let collumns = 20;
    let mut game = vec![];
    for y in 0..rows {
        let mut row = vec![];
        for x in 0..collumns {
            let mut value: i32 = 0;
            if x % 2 == 0 {
                value = 1;
            }
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
    //let mut game_copy =
    //for y in 0..rows{
    //    for x in 0..collumns {
    //game_copy[y][x].update(&game);
    //}
    //}
    //let result = Game(game_copy);
    //println!("{result}");
    loop {
        clear_background(RED);
        let mut game_copy = copy_game(&game);
        for y in 0..rows {
            for x in 0..collumns {
                let mut cell = game_copy[y][x];
                let color = match cell.state {
                    CellType::Live => GREEN,
                    _ => WHITE,
                };
                draw_rectangle(
                    (screen_width() / collumns as f32) * cell.cordinates.x as f32,
                    (screen_height() / rows as f32) * cell.cordinates.y as f32,
                    screen_width() / collumns as f32,
                    screen_height() / rows as f32 - 1.0,
                    color,
                );
            }
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            let (mut x, mut y) = mouse_position();
            let square_width = screen_width() / collumns as f32;
            let square_height = screen_height() / rows as f32;
            x = x / square_width;
            y = y / square_height;
            println!("{:?}", x.ceil());
            println!("{:?}", y.ceil());
            game_copy[y as usize][x as usize].state = match game_copy[y as usize][x as usize].state
            {
                CellType::Live => CellType::Dead,
                CellType::Dead => CellType::Live,
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            for y in 0..collumns {
                for x in 0..rows {
                    game_copy[y][x].update(&game)
                }
            }
        }
        // let result = Game(game_copy);
        //println!("{result}");
        //
        game = game_copy;

        next_frame().await
    }
}
