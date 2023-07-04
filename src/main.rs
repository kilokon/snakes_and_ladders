// #![allow(clippy::never_loop)]
use rand::{thread_rng, Rng};
use std::collections::HashMap;

// Define the dimensions of the grid
const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub position: u8,
    pub is_winner: bool,
}

impl Player {
    fn play(&mut self) {
        let mut current_score = 0;
        // loop {
        let rng = roll_dice();
        if rng == 6 {
            current_score += rng;
            println!("{} rolls a 6 and gets to roll again", self.name);
            std::thread::sleep(std::time::Duration::from_secs(1));
            let rng_sec = roll_dice();
            if rng_sec == 6 {
                current_score += rng_sec;
                println!("{} rolls a 6 again......", self.name);
                std::thread::sleep(std::time::Duration::from_secs(1));
                let rng_third = roll_dice();
                if rng_third == 6 {
                    println!("{} rolls a 6 thrice.....", self.name);
                    current_score = 0;
                    // break;
                } else {
                    current_score += rng_third;
                    // break;
                }
            } else {
                current_score += rng_sec;
                // break;
            }
        } else {
            current_score += rng;
            // break;
        }
        self.position += current_score;

        match self.position {
            100 => {
                self.is_winner = true;
                println!(
                    "{} rolled a {} and moves to {}.",
                    self.name, current_score, self.position
                );
                println!("{} is the winner", self.name);
            }
            101..=u8::MAX => {
                self.position -= current_score;
                println!(
                    "{} rolled a {} stays at {}.",
                    self.name, current_score, self.position
                );
            }
            _ => {
                println!(
                    "{} rolled a {} and moves to {}.",
                    self.name, current_score, self.position
                );
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}

struct Players {
    players: HashMap<u8, Player>,
    current_player: Option<Player>,
    current_index: u8,
}

impl Players {
    pub fn new() -> Self {
        Players {
            players: HashMap::new(),
            current_player: None,
            current_index: 0,
        }
    }

    pub fn next_player(&mut self) {
        let total_players = self.players.len() as u8;
        if total_players <= 1 {
            std::process::exit(0);
        }
        if self.current_index == total_players {
            self.current_index = 1;
        } else {
            self.current_index += 1;
        }
    }

    pub fn add_player(&mut self, player: Player) {
        if let Some(prev_key) = self.players.keys().last() {
            self.players.insert(prev_key + 1, player);
        } else {
            self.players.insert(1, player);
        }
    }
}

pub struct Cell {
    pub position: (u8, u8),
    pub state: CellState,
    pub cell_type: CellType,
}

pub enum CellType {
    Snake,
    Ladder,
    Normal,
}

pub enum CellState {
    Empty,
    Occupied,
}

pub struct Board {
    pub cells: HashMap<u8, Cell>,
    pub snakes: Vec<(u8, u8)>,
    pub ladders: Vec<(u8, u8)>,
}

#[derive(Default)]
pub struct BoardBuilder {
    cells: HashMap<u8, Cell>,
    snakes: Vec<(u8, u8)>,
    ladders: Vec<(u8, u8)>,
}

impl BoardBuilder {
    pub fn new() -> Self {
        BoardBuilder {
            cells: HashMap::new(),
            snakes: Vec::new(),
            ladders: Vec::new(),
        }
    }

    pub fn add_cells(mut self) -> Self {
        // let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];
        let mut count = 0;

        for i in 0..GRID_HEIGHT {
            for j in 0..GRID_WIDTH {
                let cell = Cell {
                    position: (i as u8, j as u8),
                    state: CellState::Empty,
                    cell_type: CellType::Normal,
                };
                count += 1;
                self.cells.insert(count, cell);
            }
        }

        self
    }

    pub fn add_snakes(mut self) -> Self {
        self.snakes.push((17, 7));
        self.snakes.push((54, 34));
        self.snakes.push((62, 19));
        self.snakes.push((64, 60));
        self.snakes.push((87, 36));
        self.snakes.push((92, 73));
        self.snakes.push((95, 75));
        self.snakes.push((98, 79));

        for snake in self.snakes.iter() {
            let (start, _) = snake;
            let cell = self.cells.get_mut(start).unwrap();
            cell.cell_type = CellType::Snake;
        }
        self
    }

    pub fn add_ladder(mut self) -> Self {
        self.ladders.push((1, 38));
        self.ladders.push((4, 14));
        self.ladders.push((9, 31));
        self.ladders.push((21, 42));
        self.ladders.push((28, 84));
        self.ladders.push((51, 67));
        self.ladders.push((72, 91));
        self.ladders.push((80, 99));

        for ladder in self.ladders.iter() {
            let (start, _) = ladder;
            let cell = self.cells.get_mut(start).unwrap();
            cell.cell_type = CellType::Ladder;
        }
        self
    }

    pub fn build(self) -> Board {
        Board {
            cells: self.cells,
            snakes: self.snakes,
            ladders: self.ladders,
        }
        //todo!()
    }
}

enum Game {
    Start,
    Play,
    End,
}

struct SnakesAndLadders {
    players: Players,
    board: Board,
}

impl SnakesAndLadders {
    pub fn new() -> Self {
        SnakesAndLadders {
            players: Players::new(),
            board: BoardBuilder::new()
                .add_cells()
                .add_snakes()
                .add_ladder()
                .build(),
        }
    }

    fn play_game(&mut self) -> Game {
        for (_, players) in self.players.players.iter_mut() {
            players.play();
            for snake in self.board.snakes.iter() {
                let (start, end) = snake;
                if players.position == *start {
                    players.position = *end;
                    println!(
                        "Snake bites Player {} ..... {} -> {}",
                        players.name, start, end
                    );
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
            for ladder in self.board.ladders.iter() {
                let (start, end) = ladder;
                if players.position == *start {
                    players.position = *end;
                    println!(
                        "Player {} climbs ladder..... {} -> {}",
                        players.name, start, end
                    );
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
            if players.is_winner {
                return Game::End;
            }
        }
        Game::Play
    }
}

fn roll_dice() -> u8 {
    thread_rng().gen_range(1..=6)
}
fn game_loop() {
    let mut game = Game::Start;
    let mut round = 0;
    let mut snakes_and_ladders = SnakesAndLadders::new();
    loop {
        match game {
            Game::Start => {
                snakes_and_ladders.players.add_player(Player {
                    name: "Player 1".to_string(),
                    position: 0,
                    is_winner: false,
                });
                snakes_and_ladders.players.add_player(Player {
                    name: "Player 2".to_string(),
                    position: 0,
                    is_winner: false,
                });
                snakes_and_ladders.players.add_player(Player {
                    name: "Player 3".to_string(),
                    position: 0,
                    is_winner: false,
                });
                game = Game::Play;
            }
            Game::Play => {
                round += 1;
                println!("                 ROUND{}", round);
                game = snakes_and_ladders.play_game();
            }
            Game::End => {
                break;
            }
        }
    }
}

fn main() {
    game_loop();
}
