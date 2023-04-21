use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, KeyboardEvent};

mod coord;
mod direction;
mod snake;

use self::{coord::Coord, direction::Direction, snake::Snake};

const MIN_TICK_RATE: u64 = 12;
const STARTING_TICK_RATE: u64 = 50;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Playing,
    Paused,
    GameOver,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    ctx: CanvasRenderingContext2d,
    grid: isize,
    grid_size: isize,
    snake: Snake,
    apple: Option<Coord>,
    direction: Direction,
    counter: u64,
    tick_rate: u64,
    status: GameStatus,
}

#[wasm_bindgen]
impl Game {
    pub fn new(ctx: CanvasRenderingContext2d, grid: isize, size: isize) -> Game {
        Game {
            ctx,
            grid,
            grid_size: size / grid,
            snake: Snake::new(Coord(grid / 2, grid / 2), 3),
            direction: Direction::Left,
            apple: Some(Coord::new_random(grid)),
            counter: 0,
            status: GameStatus::Paused,
            tick_rate: STARTING_TICK_RATE,
        }
    }

    pub fn control(&mut self, event: KeyboardEvent) {
        let key = event.key();

        match key.as_ref() {
            "Space" | " " => match self.status {
                GameStatus::Paused => self.status = GameStatus::Playing,
                GameStatus::Playing => self.status = GameStatus::Paused,
                GameStatus::GameOver => {
                    self.reset();
                }
            },
            _ => self.set_direction(&key),
        };
    }

    pub fn play(&mut self) {
        if self.counter % self.tick_rate == 0 {
            self.tick();
            self.counter = 0;
        }

        self.draw();
        self.counter += 1;
    }

    fn set_direction(&mut self, key: &str) {
        let direction = match key {
            "ArrowUp" | "w" => Direction::Up,
            "ArrowDown" | "s" => Direction::Down,
            "ArrowRight" | "d" => Direction::Right,
            "ArrowLeft" | "a" => Direction::Left,
            _ => self.direction,
        };

        if !direction.is_opposite(self.direction) {
            self.direction = direction;
        }
    }

    fn draw(&self) {
        self.snake.draw(&self.ctx, self.grid_size as f64);

        if let Some(apple) = self.apple {
            apple.draw(&self.ctx, self.grid_size as f64, "red");
        }
    }

    fn tick(&mut self) {
        if self.status != GameStatus::Playing {
            return;
        }

        self.snake.tick(self.direction);

        if self.snake.try_eat(self.apple.unwrap()) {
            self.apple = None;
            self.increase_tick_rate();
        }

        let snake_head = *self.snake.head();

        if self.snake.is_snake_body(snake_head) || self.is_out_of_bounds(snake_head) {
            self.status = GameStatus::GameOver;
        }

        if self.apple.is_none() {
            let mut apple = Coord::new_random(self.grid);

            while self.snake.is_snake(&apple) {
                apple = Coord::new_random(self.grid);
            }

            self.apple = Some(apple);
        }
    }

    fn reset(&mut self) {
        let grid = self.grid;

        self.snake = Snake::new(Coord(grid / 2, grid / 2), 3);
        self.direction = Direction::Left;
        self.apple = Some(Coord::new_random(grid));
        self.counter = 0;
        self.status = GameStatus::Paused;
        self.tick_rate = STARTING_TICK_RATE;

        let grid = self.grid as f64;
        let grid_size = self.grid_size as f64;
        let size = grid * grid_size;

        self.ctx.clear_rect(0.0, 0.0, size, size);
    }

    fn increase_tick_rate(&mut self) {
        if self.tick_rate >= MIN_TICK_RATE {
            self.tick_rate -= 2;
        }
    }

    fn is_out_of_bounds(&self, Coord(x, y): Coord) -> bool {
        x < 0 || y < 0 || x > self.grid - 1 || y > self.grid - 1
    }
}
