use std::{collections::VecDeque, fmt::Debug};

use web_sys::CanvasRenderingContext2d;

use super::{coord::Coord, direction::Direction};

#[derive(Debug)]
pub struct Snake {
    body: VecDeque<Coord>,
    prev_tail: Option<Coord>,
}

impl Snake {
    pub fn new(Coord(x, y): Coord, length: isize) -> Snake {
        let mut body = VecDeque::with_capacity(3);

        for i in 0..length {
            body.push_front(Coord(x + i, y));
        }

        Snake {
            body,
            prev_tail: None,
        }
    }

    pub fn tick(&mut self, dir: Direction) {
        let tail = self.body.pop_front().unwrap();
        self.prev_tail = Some(tail);

        let next_head = self.new_head(dir);
        self.body.push_back(next_head);
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, size: f64) {
        for coord in self.body.iter() {
            coord.draw(ctx, size, "green");
        }

        if let Some(tail) = self.prev_tail {
            tail.clear(ctx, size);
        }
    }

    pub fn try_eat(&mut self, coord: Coord) -> bool {
        let head = *self.head();

        match (head == coord, self.prev_tail) {
            (true, Some(prev_tail)) => {
                self.body.push_front(prev_tail);
                self.prev_tail = None;
                true
            }
            _ => false,
        }
    }

    pub fn is_snake(&mut self, coord: &Coord) -> bool {
        self.body.contains(coord)
    }

    pub fn is_snake_body(&self, coord: Coord) -> bool {
        let body_count = self.body.len() - 1;
        self.body.iter().take(body_count).any(|&c| c == coord)
    }

    pub fn head(&self) -> &Coord {
        self.body.back().unwrap()
    }

    fn new_head(&self, dir: Direction) -> Coord {
        let head = *self.head();
        let Coord(x, y) = head;

        match dir {
            Direction::Up => Coord(x, y - 1),
            Direction::Down => Coord(x, y + 1),
            Direction::Left => Coord(x - 1, y),
            Direction::Right => Coord(x + 1, y),
        }
    }
}

#[test]
fn test_new_snake() {
    let snake = Snake::new(Coord(10, 10), 3);

    assert_eq!(snake.body.len(), 3);
    assert_eq!(snake.prev_tail, None);
}

#[test]
fn test_new_snake_coords() {
    let snake = Snake::new(Coord(10, 10), 3);

    let tail = snake.body.front().unwrap();
    assert_eq!(*tail, Coord(12, 10));

    let head = snake.body.back().unwrap();
    assert_eq!(*head, Coord(10, 10));
}

#[test]
fn test_tick() {
    let mut snake = Snake::new(Coord(10, 10), 3);

    snake.tick(Direction::Up);

    let head = snake.head();
    assert_eq!(*head, Coord(10, 9));

    let tail = snake.body.front().unwrap();
    assert_eq!(*tail, Coord(11, 10));

    assert_eq!(snake.prev_tail, Some(Coord(12, 10)));
}

#[test]
fn test_eat_tick() {
    let mut snake = Snake::new(Coord(10, 10), 3);
    let apple = Coord(10, 11);

    snake.tick(Direction::Down);

    assert_eq!(snake.body.len(), 3);
    assert_eq!(snake.try_eat(apple), true);

    snake.tick(Direction::Down);

    assert_eq!(snake.body.len(), 4);
    assert_eq!(*snake.head(), Coord(10, 12));
    assert_eq!(*snake.body.front().unwrap(), Coord(11, 10));
}

#[test]
fn test_is_snake_body() {
    let snake = Snake::new(Coord(10, 10), 3);

    assert_eq!(snake.is_snake_body(Coord(10, 10)), false);
    assert_eq!(snake.is_snake_body(Coord(11, 10)), true);
    assert_eq!(snake.is_snake_body(Coord(12, 10)), true);
}
