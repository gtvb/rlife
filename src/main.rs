// Game of life
//
// Rules:
// Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
//
// How it works:
// Think of the game as a set of states, with each given state as a screenshot. The next state is 
// dependent of the previous one, and so on. 
//
// Finding the neighbours:
//  - the three top neighbours are in the (x - 1) row, and (y - 1), (y) and (y + 1) columns;
//  - the two neighbours left are respectively, in the same row (x), (y - 1) and (y + 1) columns;
//  - the three bottom neighbours are in the (x + 1) row, and (y - 1), (y) and (y + 1) columns;
use itertools::Itertools;
use std::{fs, thread, time, io::{self, Write}};
use std::collections::{HashMap};

use terminal_size::{Width, Height, terminal_size};
use serde::{Deserialize, Serialize};

const ALIVE: char = '█';
const DEAD: char = '░';

#[derive(Serialize, Deserialize)]
struct JsonSeed {
    cells: Vec<[u16; 2]>
}

#[derive(Debug)]
struct Game {
    grid: Vec<Vec<u16>>,
    alive_cells: Vec<[u16; 2]>,
}

impl Game {
    pub fn new(seed: Vec<[u16; 2]>) -> Self {
        let size = terminal_size();

        if let Some((Width(w), Height(h))) = size {
            let mut grid: Vec<Vec<u16>> = Vec::with_capacity(h as usize);
            
            for i in 0..h {
                let row: Vec<u16> = vec![0; w as usize];
                grid.push(row);
            }

            let mut game = Game { grid, alive_cells: seed };
            game.populate();
            game
        } else { panic!() }
    }

    fn grid_size(&self) -> (u16, u16) {
        (self.grid.len() as u16, self.grid[0].len() as u16)
    }

    pub fn render(&self) {
        let (rows, cols) = self.grid_size();

        for i in 0..rows {
            for j in 0..cols {
                if self.grid[i as usize][j as usize] == 0 {
                    print!("{}", DEAD);
                } else {
                    print!("{}", ALIVE);
                }
            }
            if i < rows - 1 {
                print!("\n");
            }
        }
    }
    pub fn update_state(&mut self) {
        let mut dead_cells: Vec<[u16; 2]> = Vec::new();

        for cell in self.alive_cells.iter() {
            let dead_nbs = self.get_dead_neighbours(&cell);

            for neighbour in dead_nbs {
                dead_cells.push(neighbour);
            }
        }

        dead_cells = dead_cells.into_iter().unique().collect();

        let mut to_insert = Vec::new();
        for dead_cell in dead_cells {
            let living_count = self.get_living_neighbours_count(&dead_cell);

            if living_count == 3 {
                to_insert.push(dead_cell);
            }
        }

        let mut to_remove = Vec::new();
        for alive_cell in self.alive_cells.iter() {
            let living_count = self.get_living_neighbours_count(&alive_cell);

            if living_count < 2 || living_count > 3 {
                to_remove.push(*alive_cell);
            } else if living_count == 2 || living_count == 3 {
                continue
            }
        }

        println!("remove {}: {:?}", to_remove.len(), to_remove);
        for c in to_remove {
            self.alive_cells.retain(|arr| *arr != c);
            self.grid[c[0] as usize][c[1] as usize] = 0;
        }

        println!("insert {}: {:?}", to_insert.len(), to_insert);
        for c in to_insert {
            self.alive_cells.push(c);
            self.grid[c[0] as usize][c[1] as usize] = 1;
        }

        self.populate();
    }

    fn get_living_neighbours_count(&self, root: &[u16; 2]) -> u16 {
        let mut ret: u16 = 0;

        let (rows, cols) = self.grid_size();

        let row = root[0];
        let col = root[1];

        if row > 0 {
            if col > 0 && self.cell_value(row - 1, col - 1) == 1 {
                ret += 1;
            }

            if self.cell_value(row - 1, col) == 1 {
                ret += 1;
            }

            if col + 1 <= cols && self.cell_value(row - 1, col + 1) == 1 {
                ret += 1;
            }
        }

        if col > 0 && self.cell_value(row, col - 1) == 1 {
            ret += 1;
        }

        if col + 1 <= cols && self.cell_value(row, col + 1) == 1 {
            ret += 1;
        }

        if row + 1 <= rows {
            if col > 0 && self.cell_value(row + 1, col - 1) == 1 {
                ret += 1;
            }

            if self.cell_value(row + 1, col) == 1 {
                ret += 1;
            }

            if col + 1 <= cols && self.cell_value(row + 1, col + 1) == 1 {
                ret += 1;
            }
        }

        ret
    }
    
    fn get_dead_neighbours(&self, root: &[u16; 2]) -> Vec<[u16; 2]> {
        let mut ret = Vec::new();

        let (rows, cols) = self.grid_size();

        let row = root[0];
        let col = root[1];

        if row > 0 {
            if col > 0 && self.cell_value(row - 1, col - 1) == 0 {
                ret.push([row - 1, col - 1])
            }

            if self.cell_value(row - 1, col) == 0 {
                ret.push([row - 1, col])
            }

            if col + 1 <= cols && self.cell_value(row - 1, col + 1) == 0 {
                ret.push([row - 1, col + 1])
            }
        }

        if col > 0 && self.cell_value(row, col - 1) == 0 {
            ret.push([row, col - 1])
        }

        if col + 1 <= cols && self.cell_value(row, col + 1) == 0 {
            ret.push([row, col + 1])
        }

        if row + 1 <= rows {
            if col > 0 && self.cell_value(row + 1, col - 1) == 0 {
                ret.push([row + 1, col - 1])
            }

            if self.cell_value(row + 1, col) == 0 {
                ret.push([row + 1, col])
            }

            if col + 1 <= cols && self.cell_value(row + 1, col + 1) == 0 {
                ret.push([row + 1, col + 1])
            }
        }

        ret
    }

    fn cell_value(&self, row: u16, col: u16) -> u16 {
        self.grid[row as usize][col as usize]
    }

    fn populate(&mut self) {
        for cell in self.alive_cells.iter() {
            self.grid[cell[0] as usize][cell[1] as usize] = 1;
        }
    }

}

fn main() {
    let seed_file = fs::read_to_string("default.json").unwrap();
    let seed_json: JsonSeed = serde_json::from_str(&seed_file).unwrap();

    let mut g = Game::new(seed_json.cells);

    g.render();
    thread::sleep(time::Duration::from_secs(1));
    loop {
        g.update_state();
        g.render();
        thread::sleep(time::Duration::from_secs(1));
    }
}
