// Current prelude for using `wasm_bindgen`, and this'll get smaller over time!
#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::collections::LinkedList;

//Declaring the JS functions we want to use
#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn getKey() -> i32;
    fn logValue(s: &str);
    fn drawDot(x: i32, y: i32, radius: i32);
    fn eraseDot(x: i32, y: i32, radius: i32);
    fn drawFood(x: i32, y: i32);
    fn eraseFood(x: i32, y: i32);
    fn getRandomCoord(gridSize: i32, squareSize: i32) -> i32;
    fn endGame();
}

const GRID_SIZE: i32 = 200;
const SQUARE_SIZE: i32 = 10;

enum Direction {
  Left,
  Up,
  Right,
  Down
}

#[wasm_bindgen]
pub enum Collision {
  Food,
  Snake,
  None
}

#[wasm_bindgen]
pub struct Location {
  x: i32,
  y: i32
}

#[wasm_bindgen]
impl Location {
  pub fn new(x_val: i32, y_val: i32) -> Location {
    let wrap_x_val;
    let wrap_y_val;

    if x_val < 0 {
      wrap_x_val = GRID_SIZE;
    } else if x_val > GRID_SIZE {
      wrap_x_val = 0;
    } else {
      wrap_x_val = x_val;
    }

    if y_val < 0 {
      wrap_y_val = GRID_SIZE;
    } else if y_val > GRID_SIZE {
      wrap_y_val = 0;
    } else {
      wrap_y_val = y_val;
    }

    Location { x: wrap_x_val, y: wrap_y_val }
  }

  pub fn is_equal(a: &Location, b: &Location) -> bool {
    if (a.x == b.x) && (a.y == b.y) {
       return true
    }

    false
  }
}

pub struct Snake {
  direction: Direction,
  body: LinkedList<Location>,
}

impl Snake {
  pub fn new() -> Snake {
    Snake { direction: Direction::Left, body: Snake::baby_snake() }
  }

  fn baby_snake() -> LinkedList<Location> {
    let mut snake = LinkedList::new();
    snake.push_back(Location::new(50, 50));
    snake.push_back(Location::new(60, 50));
    snake.push_back(Location::new(70, 50));
    snake
  }

  fn get_next_snake_head(&mut self) -> Location {
    let head = self.body.front().unwrap();
    match self.direction {
        Direction::Left => Location::new(head.x - SQUARE_SIZE, head.y),
        Direction::Up => Location::new(head.x, head.y - SQUARE_SIZE),
        Direction::Right => Location::new(head.x + SQUARE_SIZE, head.y),
        Direction::Down => Location::new(head.x, head.y + SQUARE_SIZE)
      }
  }

  pub fn add_head(&mut self) {
    let next_head = self.get_next_snake_head();
    self.body.push_front(next_head);    
  }

  pub fn shrink_tail(&mut self) {
    let tail = self.body.pop_back().unwrap();
    eraseDot(tail.x, tail.y, SQUARE_SIZE);
  }
}

#[wasm_bindgen]
pub struct Game {
  snake: Snake,
  food: Location
}

#[wasm_bindgen]
impl Game {
  pub fn new() -> Game {
    let mut game = Game { snake: Snake::new(), food: Location::new(20, 20) };
    game.draw_food();
    game
  }

  pub fn draw_snake(&mut self) {
    let snake_head = self.snake.body.front().unwrap().clone();
    drawDot(snake_head.x, snake_head.y, SQUARE_SIZE/2)
  }

  pub fn draw_food(&mut self) {
    let food = &self.food;
    drawFood(food.x, food.y);
  }

  pub fn erase_food(&mut self) {
    let food = &self.food;
    eraseFood(food.x, food.y);
  }

  pub fn eat_food(&mut self) {
    self.erase_food();
    let x = getRandomCoord(GRID_SIZE, SQUARE_SIZE);
    let y = getRandomCoord(GRID_SIZE, SQUARE_SIZE);
    self.food = Location::new(x, y);
    self.draw_food();
  }

  pub fn detect_collision(&mut self) -> Collision {
    let snake_head = self.snake.body.front().unwrap().clone();
    let isHittingSelf = self.snake.body.iter()
      .skip(1)
      .any(|segment| Location::is_equal(&segment, snake_head));

    if isHittingSelf {
      alert(&format!("You Bit Yourself!"));
      return Collision::Snake
    }

    if Location::is_equal(&self.food, snake_head) {
      return Collision::Food
    }

    Collision::None
  }

  pub fn increment(&mut self) {
    self.snake.direction = match getKey() {
      37 => Direction::Left,
      38 => Direction::Up,
      39 => Direction::Right,
      40 => Direction::Down,
      _ => Direction::Down
    };

    self.snake.add_head();
    match self.detect_collision() {
      Collision::Food => self.eat_food(),
      Collision::Snake => endGame(),
      Collision::None => self.snake.shrink_tail()
    }

    self.draw_snake();
  }
}
