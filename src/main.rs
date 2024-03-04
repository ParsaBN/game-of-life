use std::ops::Add;
use macroquad::prelude::*;

use std::thread::sleep;
use std::time::Duration;

const GRID: i16 = 32 + 1;
const LIGHTBLUE: Color = Color { r: 102.0 / 255.0, g: 204.0 / 255.0, b: 255.0 / 255.0, a: 0.85 };

#[derive(PartialEq, Clone, Copy)]
struct Coord {
    x: i16, 
    y: i16,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Coord {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub fn coord(self) -> Coord {
        match self {
            Direction::North => Coord::new(0, 1),
            Direction::NorthEast => Coord::new(1, 1),
            Direction::East => Coord::new(1, 0),
            Direction::SouthEast => Coord::new(1, -1),
            Direction::South => Coord::new(0, -1),
            Direction::SouthWest => Coord::new(-1, -1),
            Direction::West => Coord::new(-1, 0),
            Direction::NorthWest => Coord::new(-1, 1),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Agent {
    coord: Coord,
    // genome
    // brain
    // dir: Direction,
}

// struct Grid {

// }

#[macroquad::main("Project Cresco")]
async fn main() {

    let directions = [Direction::North, Direction::South, Direction::East, Direction::West, Direction::NorthEast, Direction::NorthWest, Direction::SouthEast, Direction::SouthWest];

    let mut grid: Vec<Vec<bool>> = vec![vec![false; GRID.try_into().unwrap()]; GRID.try_into().unwrap()];

    let DELAY = Duration::from_secs_f32(0.08);

    let mut agents: Vec<Agent> = Vec::new();

    let mut initialised = false;
    let mut iteration = 0;
    
    loop {
        clear_background(LIGHTGRAY);
        
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / GRID as f32;
        
        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);
        
        for i in 1..GRID {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
        }
        
        for i in 1..GRID {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                LIGHTGRAY,
            );
        }

        if !initialised && iteration > 0 {
            // could get rid of holding down E
            if is_key_down(KeyCode::E) {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    // println!("Mouse Clicked at: ({:?}, {:?})", ((mouse_x - offset_x) / sq_size).round() - 1.0, ((mouse_y - offset_y) / sq_size).round() - 1.0);

                    let adjusted_x = ((mouse_x - offset_x) / sq_size).round() - 1.0;
                    let adjusted_y = ((mouse_y - offset_y) / sq_size).round() - 1.0;

                    let new_agent = Agent {
                        coord: Coord { x: adjusted_x as i16, y: adjusted_y as i16 },
                        // dir: Direction::East,
                    };
                    grid[new_agent.coord.y as usize][new_agent.coord.x as usize] = true;
                    agents.push(new_agent);

                }
            }
            if is_key_pressed(KeyCode::Enter) {
                initialised = true;
            }

            for agent in &agents {
                draw_circle(
                    offset_x + (agent.coord.x).rem_euclid(GRID - 1) as f32 * sq_size + sq_size,
                    offset_y + (agent.coord.y).rem_euclid(GRID - 1) as f32 * sq_size + sq_size,
                    sq_size / 2.0,
                    LIGHTBLUE,
                );
            }

        } else {
            iteration += 1;

        


        // // go through all cells/agents? should i have done this differently witohut agensts so i dont end up with a huge list of agents? its fine rn
        // for agent in &agents {
        //     println!("agent at {:?}, {:?}", agent.coord.x, agent.coord.y);

        //     grid[agent.coord.y as usize][agent.coord.x as usize] = true;

        //     let mut live_neighbours = 0;

        //     for direction in directions {
        //         if grid[((agent.coord.y + direction.coord().y).rem_euclid(GRID)) as usize][((agent.coord.x + direction.coord().x).rem_euclid(GRID)) as usize] {
        //             live_neighbours += 1;
        //         }
        //     }

        //     draw_circle(
        //         offset_x + (agent.coord.x).rem_euclid(GRID - 1) as f32 * sq_size + sq_size,
        //         offset_y + (agent.coord.y).rem_euclid(GRID - 1) as f32 * sq_size + sq_size,
        //         sq_size / 2.0,
        //         RED,
        //     );
            
        // }


        // // drawing potential neighbours
        // for agent in &agents {
        //     for direction in directions {
        //         let mut colour = GREEN;
        //         if grid[((agent.coord.y + direction.coord().y).rem_euclid(GRID)) as usize][((agent.coord.x + direction.coord().x).rem_euclid(GRID)) as usize] == true {
        //             colour = BLUE;
        //         }
        //         draw_circle(
        //             offset_x + (agent.coord.x + direction.coord().x).rem_euclid(GRID - 1) as f32 * sq_size + sq_size,
        //             offset_y + (agent.coord.y + direction.coord().y).rem_euclid(GRID - 1) as f32 * sq_size + sq_size,
        //             sq_size / 4.0, 
        //             colour,
        //         );
        //     }
        // }


        // testing above
        // logic below



            let mut next_grid: Vec<Vec<bool>> = vec![vec![false; GRID.try_into().unwrap()]; GRID.try_into().unwrap()];


            for row in 0..GRID {
                for col in 0..GRID {
                    let mut live_neighbours = 0;
                    for direction in directions {
                        if grid[((row + direction.coord().y).rem_euclid(GRID)) as usize][((col + direction.coord().x).rem_euclid(GRID)) as usize] == true {
                            live_neighbours += 1;
                        }
                    }
                    
                    if grid[row as usize][col as usize] == true {
                        // live cell

                        // conditions to stay alive
                        if live_neighbours == 2 || live_neighbours == 3 {
                            draw_circle(
                                offset_x + (col).rem_euclid(GRID - 1) as f32 * sq_size + sq_size,
                                offset_y + (row).rem_euclid(GRID - 1) as f32 * sq_size + sq_size,
                                sq_size / 2.0,
                                LIGHTBLUE,
                            );
                            next_grid[row as usize][col as usize] = true;
                        } else {
                            next_grid[row as usize][col as usize] = false;
                            agents.retain(|&x| x != Agent { coord: Coord {x: col, y: row }});
                        }

                    } else {
                        // dead cell

                        // conditions to become alive (as if by reproduction)
                        if live_neighbours == 3 {
                            // println!("new cell!");
                            draw_circle(
                                offset_x + (col).rem_euclid(GRID - 1) as f32 * sq_size + sq_size,
                                offset_y + (row).rem_euclid(GRID - 1) as f32 * sq_size + sq_size,
                                sq_size / 2.0,
                                LIGHTBLUE,
                            );
                            next_grid[row as usize][col as usize] = true;
                            agents.push(Agent { coord: Coord { x: col, y: row } });
                        }
                        
                    }

                }
            }

            grid = next_grid;
            sleep(DELAY);
        }
        next_frame().await
    }
}