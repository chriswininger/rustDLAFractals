extern crate rand;

use rand::Rng;

#[derive(Clone)]
pub struct ColorizedPoint {
  pub color: [u8; 4]
}

#[derive(Clone)]
pub enum FieldPosition {
  EMPTY,
  OCCUPIED(ColorizedPoint),
  STUCK(ColorizedPoint)
}

pub struct DLAField {
  pub positionHash: Vec<Vec<FieldPosition>>
}

impl DLAField {
  pub fn new(numPoints: i32, width: usize, height: usize) -> DLAField {
    let mut rng = rand::thread_rng();
    let mut positionHash = DLAField::generateEmptyPositionHash(width, height);

    for i in 0..numPoints {
      let mut x = rng.gen_range(0, width as u32);
      let mut y = rng.gen_range(0, height as u32);

      while DLAField::isPositionOccupied(&positionHash, x as i32, y as i32) {
        x = rng.gen_range(0, width as u32);
        y = rng.gen_range(0, height as u32);
      }

      // occupy the position
      positionHash[x as usize][y as usize] = FieldPosition::OCCUPIED(ColorizedPoint {
        color: [255, 0, 0, 255]
      });
    }

    DLAField {
      positionHash
    }
  }

  pub fn getOneDimensionalRepresentation(&self) -> Vec<u8> {
    let width = self.getWidth();
    let height = self.getHeight();
    let oneDimensionalLen = width * height;

    let mut values = vec![0 as u8; oneDimensionalLen * 4];

    for i in 0..oneDimensionalLen {
      let x = i % width;
      let y = i / height;

      let ndx = i as usize * 4;

      match &self.positionHash[x][y] {
        FieldPosition::OCCUPIED(colorizedPoint) => {
          values[ndx] = colorizedPoint.color[0];
          values[ndx + 1] = colorizedPoint.color[1];
          values[ndx + 2] = colorizedPoint.color[2];
          values[ndx + 3] = colorizedPoint.color[3];
        },
        FieldPosition::STUCK(colorizedPoint) => {
          values[ndx] = colorizedPoint.color[0];
          values[ndx + 1] = colorizedPoint.color[1];
          values[ndx + 2] = colorizedPoint.color[2];
          values[ndx + 3] = colorizedPoint.color[3];
        }
        FieldPosition::EMPTY => {
          values[ndx] = 0;
          values[ndx + 1] = 0;
          values[ndx + 2] = 0;
          values[ndx + 3] = 255;
        }
      }
    }

    values.to_vec()
  }

  pub fn nextState(&mut self) -> bool {
    let mut isDone = true;

    // might be better to walk the y array in reverse so we check/update lowest first
    let mut cntStuck = 0;

    /*
      TODO (CAW) While walking backwords is better perhaps it would be better to have list of (x, y)
      with only the occupied points (like we used to) and only iterate over those, only issue is
      can't go top to bottom so... (if doing this consider a move function(x1, y1, x2, y2) which
      updates both pieces of state
    */
    for x in 0..self.getWidth() as u32 {
      // walk y in reverse so points near the bottom get stuck first
      for y in (0..self.getHeight() as u32).rev() {
        let stuck = self.isStuck(x as i32, y as i32, false);
        let curVal =  &self.positionHash[x as usize][y as usize];

        if stuck {
          cntStuck += 1;
        }

        match curVal {
          FieldPosition::OCCUPIED(point) => {
            if !stuck {
              isDone = false;

              let newPosition = self.findNextPosition(x as i32, y as i32);

              self.positionHash[newPosition.0 as usize][newPosition.1 as usize] =
                  FieldPosition::OCCUPIED(ColorizedPoint {
                    color: [255, 0, 0, 255]
                  });

              self.positionHash[x as usize][y as usize] = FieldPosition::EMPTY;
            } else {
              self.positionHash[x as usize][y as usize] = FieldPosition::STUCK(ColorizedPoint {
                  color: [0, 255, 0, 255]
                });
            }
          },
          FieldPosition::STUCK(coloredPoint) => {},
          FieldPosition::EMPTY => {}
        }
      }
    }

    if cntStuck % 100 == 0 {
      println!("cntStuck: {}", cntStuck);
    }

    isDone
  }

  pub fn getWidth(&self) -> usize {
    self.positionHash.len()
  }

  pub fn getHeight(&self) -> usize {
    self.positionHash[0].len()
  }

  fn generateEmptyPositionHash(width: usize, height: usize) -> Vec<Vec<FieldPosition>> {
    (0..width)
        .map(|x| {
          let col = (0..height)
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
    let width = self.getWidth() as i32;
    let height = self.getHeight() as i32;

    // TOOD (CAW): Consider precalculating available states and if there is just one possibility take it
    while newX < 0 || newY < 0 || newX >= width as i32 || newY >= height as i32 {
      newX = if rng.gen_bool(0.5) { x + 1 } else { x - 1 };
      newY = if rng.gen_bool(0.75) { y + 1 } else { y - 1 };
    }

    let mut attemptCount = 0;

    while  DLAField::isPositionOccupied(&self.positionHash, newX, newY) && attemptCount <= 4 {
      while newX < 0 || newY < 0 || newX >= width as i32 || newY >= height as i32 {
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
      FieldPosition::STUCK(point) => true,
      FieldPosition::EMPTY => false
    }
  }

  fn isStuck(&self, x: i32, y: i32, recursion: bool) -> bool {
    let width = self.getWidth() as i32;
    let height = self.getHeight() as i32;

    match &self.positionHash[x as usize][y as usize] {
      FieldPosition::EMPTY => {
        return false
      },
      FieldPosition::STUCK(colorizedPoint) => return true,
      FieldPosition::OCCUPIED(colorizedPoint) => {
        if y >= height - 1 as i32 {
          return true;
        }
      }
    }

    if recursion {
      return  false;
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

        if neighborX < width as i32 && neighborY < height as i32 && self.isStuck(neighborX, neighborY, true) {
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

  // this is more for testing
  pub fn getStuckCount(&self) -> u32 {
    let mut cnt = 0;

    for x in 0..self.getWidth() {
      for y in 0..self.getHeight() {
        match &self.positionHash[x][y] {
          FieldPosition::EMPTY => {}
          FieldPosition::OCCUPIED(colorizedPoint) => {}
          FieldPosition::STUCK(colorizedPoint) =>{
            cnt +=  1;
          }
        }
      }
    }

    cnt
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn isPositionOccupied_shouldReturnCorrectValues() {
    let colorizedPoint1: ColorizedPoint = ColorizedPoint { color: [0 ,0, 0,0 ]};
    let colorizedPoint2: ColorizedPoint = ColorizedPoint { color: [0 ,0, 0,0 ]};

    let positionHash = [
      [FieldPosition::EMPTY, FieldPosition::OCCUPIED(colorizedPoint1)].to_vec(),
      [FieldPosition::STUCK(colorizedPoint2), FieldPosition::EMPTY].to_vec()
    ].to_vec();

    let position = DLAField::isPositionOccupied(&positionHash, 0, 0);
    assert_eq!(position, false);

    let position = DLAField::isPositionOccupied(&positionHash, 0, 1);
    assert_eq!(position, true);

    let position = DLAField::isPositionOccupied(&positionHash, 1, 0);
    assert_eq!(position, true);

    let position = DLAField::isPositionOccupied(&positionHash, 1, 1);
    assert_eq!(position, false);
  }

  #[test]
  fn isStuck_shouldReturnCorrectValues() {
    let colorizedPoint1 = ColorizedPoint {
      color: [255, 255, 255, 255]
    };

    let colorizedPoint2 = ColorizedPoint {
      color: [255, 255, 255, 255]
    };

    let colorizedPoint3 = ColorizedPoint {
      color: [255, 255, 255, 255]
    };

    let colorizedPoint4 = ColorizedPoint {
      color: [255, 255, 255, 255]
    };

    let positionHash = [
      [FieldPosition::OCCUPIED(colorizedPoint1), FieldPosition::EMPTY, FieldPosition::EMPTY ].to_vec(),
      [FieldPosition::EMPTY, FieldPosition::OCCUPIED(colorizedPoint3), FieldPosition::EMPTY].to_vec(),
      [FieldPosition::OCCUPIED(colorizedPoint2), FieldPosition::EMPTY, FieldPosition::OCCUPIED(colorizedPoint4)].to_vec(),
    ].to_vec();

    let field = DLAField {
      positionHash
    };

    assert_eq!(field.isStuck(0,0, false), false);
    assert_eq!(field.isStuck(0,1, false), false);
    assert_eq!(field.isStuck(0,2, false), false);

    assert_eq!(field.isStuck(1,0, false), false);

    // is stuck because it's neighbor is stuck
    assert_eq!(field.isStuck(1,1, false), true);
    assert_eq!(field.isStuck(1,2, false), false);

    assert_eq!(field.isStuck(2,0, false), false);

    // not stuck because it's empty (even though it's neighbor is stuck)
    assert_eq!(field.isStuck(2,1, false), false);

    // stuck because it's at the bottom and occupied
    assert_eq!(field.isStuck(2,2, false), true);
  }
}
