extern crate rand;

use rand::Rng;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const ONE_DIMENSIONAL_LENGTH: u32 = WIDTH * HEIGHT;

pub struct ColorizedPoint {
  pub x: u32,
  pub y: u32,
  pub color: [u8; 4]
}

pub enum FieldPosition {
  EMPTY,
  OCCUPIED(ColorizedPoint),
  STUCK
}

pub struct DLAField {
  pub numPoints: i32,
  pub field: Vec<ColorizedPoint>,
  pub positionHash: Vec<Vec<FieldPosition>>
}

impl DLAField {
  pub fn new(numPoints: i32) -> DLAField {
    let mut rng = rand::thread_rng();
    let mut positionHash = DLAField::generateEmptyPositionHash();

    let field = (0..numPoints)
       .map(|ndx| {
         let mut x = rng.gen_range(0, WIDTH);
         let mut y = rng.gen_range(0, HEIGHT);

         while DLAField::isPositionOccupied(&positionHash, x as i32, y as i32) {
           x = rng.gen_range(0, WIDTH);
           y = rng.gen_range(0, HEIGHT);
         }

         positionHash[x as usize][y as usize] = FieldPosition::OCCUPIED(ColorizedPoint {
           x,
           y,
           color: [255, 0, 0, 255]
         });

         // return the new point
         ColorizedPoint {
           x,
           y,
           color: [255, 0, 0, 255]
         }
       })
       .collect();

    DLAField {
      numPoints,
      field,
      positionHash
    }
  }

  pub fn getOneDimensionalRepresentation(&self) -> [u8; ONE_DIMENSIONAL_LENGTH as usize * 4] {
    let mut values = [0; ONE_DIMENSIONAL_LENGTH as usize * 4];
    for i in 0..ONE_DIMENSIONAL_LENGTH {
      let x = i % WIDTH;
      let y = i / HEIGHT;

      let ndx = i as usize * 4;

      if DLAField::isPositionOccupied(&self.positionHash, x as i32, y as i32) {
        values[ndx] = 255;
        values[ndx + 1] = 0;
        values[ndx + 2] = 0;
        values[ndx + 3] = 255;
      } else {
        values[ndx] = 0;
        values[ndx + 1] = 0;
        values[ndx + 2] = 0;
        values[ndx + 3] = 255;
      }
    }

    values
  }

  pub fn nextState(&mut self) -> bool {
    let mut isDone = true;

    let mut cntStuck = 0;
    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        let stuck = self.isStuck(x as i32, y as i32, false);

        if !stuck {
          cntStuck += 1;
        }

        if  !stuck && !self.isEmpty(x as i32, y as i32) {
          isDone = false;

          let newPosition = self.findNextPosition(x as i32, y as i32);

          self.positionHash[newPosition.0 as usize][newPosition.1 as usize] =
             FieldPosition::OCCUPIED(ColorizedPoint {
               x: newPosition.0 as u32,
               y: newPosition.1 as u32,
               color: [255, 0, 0, 255]
             });

          self.positionHash[x as usize][y as usize] = FieldPosition::EMPTY;
        }
      }
    }

    if cntStuck > 0 {
      println!("!!! cntStuc: {}", cntStuck);
    }
    isDone
  }

  fn generateEmptyPositionHash() -> Vec<Vec<FieldPosition>> {
    (0..WIDTH)
       .map(|x| {
         let col = (0..HEIGHT)
            .map(|y| {
              FieldPosition::EMPTY
            })
            .collect();

         col
       })
       .collect()
  }

  fn findNextPosition(&self, x: i32, y: i32) -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let mut newX = if rng.gen_bool(0.5) { x + 1 } else { x - 1 };
    let mut newY = if rng.gen_bool(0.75) { y + 1 } else { y - 1 };

    while newX < 0 || newY < 0 || newX >= WIDTH as i32 || newY >= HEIGHT as i32 {
      newX = if rng.gen_bool(0.5) { x + 1 } else { x - 1 };
      newY = if rng.gen_bool(0.75) { y + 1 } else { y - 1 };
    }

    let mut attemptCount = 0;

    while  DLAField::isPositionOccupied(&self.positionHash, newX, newY) && attemptCount <= 4 {
      while newX < 0 || newY < 0 || newX >= WIDTH as i32 || newY >= HEIGHT as i32 {
        newX = if rng.gen_bool(0.5) { x + 1 } else { x - 1 };
        newY = if rng.gen_bool(0.75) { y + 1 } else { y - 1 };
      }

      attemptCount += 1;
    }

    if attemptCount < 4 {
      return (newX, newY)
    } else {
      return (x, y)
    }
  }

  fn isPositionOccupied(positionHash: &Vec<Vec<FieldPosition>>, x: i32, y: i32) -> bool {
    let val = &positionHash[x as usize][y as usize];
    match val {
      FieldPosition::OCCUPIED(point) => true,
      FieldPosition::STUCK => true,
      FieldPosition::EMPTY => false
    }
  }

  fn isStuck(&self, x: i32, y: i32, recursion: bool) -> bool {
//      if x >= WIDTH as i32 {
//         return false
//      } else if y >= HEIGHT as i32 {
//         return  true
//      } else

    if let FieldPosition::STUCK = self.positionHash[x as usize][y as usize] { // SEEMS LIKE WE SHOULD ALWAYS CHECK AND SAY STUCK IF THIS IS STUCK AND JUST NEED TO MAKE SURE WE NEVER ASK OUTSIDE BOUNDS
      return true
    } else if y + 1 >= HEIGHT as i32 {
      return true
    } else if recursion {
      return false
    }

    let mut dx: i32 = -1;
    if x == 0 {
      dx = 0;
    }

    while dx < 2 {
      let mut dy: i32 = -1;

      if y == 0 {
        dy = 0;
      }

      while dy < 2 {
        let neighborX = x + dx as i32;
        let neighborY = y + dy as i32;

        if neighborX < WIDTH as i32 && neighborY < HEIGHT as i32 && self.isStuck(neighborX, neighborY, true) {
          return true
        }

        dy += 1;
      }

      dx += 1
    }

    false
  }

  fn isEmpty(&self, x: i32, y: i32) -> bool {
    if let FieldPosition::EMPTY = self.positionHash[x as usize][y as usize] {
      return true
    }

    false
  }
}