extern crate rand;
extern crate png;

use rand::Rng;
use png::HasParameters;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

mod dla;

const NUM_POINTS: u32 = 5000;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const ONE_DIMENSIONAL_LENGTH: u32 = WIDTH * HEIGHT;

fn main() {
   let dlaField = DLAField::new(NUM_POINTS);
   saveToPNG(dlaField);
}

fn saveToPNG(dlaField: DLAField) {
   let path = Path::new(r"/home/chris/projects/rustDLAFractals/testImageFractal.png");
   let file = File::create(path).unwrap();
   let ref mut w = BufWriter::new(file);

   let mut encocer = png::Encoder::new(w, WIDTH, HEIGHT);
   encocer.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
   let mut writer = encocer.write_header().unwrap();

   let data = dlaField.getOneDimensionalRepresentation();
   writer.write_image_data(&data).unwrap();
}

struct DLAField {
   numPoints: u32,
   field: Vec<dla::ColorizedPoint>,
   positionHash: Vec<Vec<bool>>
}

impl DLAField {
   pub fn new(numPoints: u32) -> DLAField {
      let mut rng = rand::thread_rng();
      let mut positionHash = DLAField::generateEmptyPositionHash();

      let field = (0..numPoints)
         .map(|ndx| {
            let mut x = rng.gen_range(0, WIDTH);
            let mut y = rng.gen_range(0, HEIGHT);

            while DLAField::isPositionOccupied(&positionHash, x, y) {
               x = rng.gen_range(0, WIDTH);
               y = rng.gen_range(0, HEIGHT);
            }

            positionHash[x as usize][y as usize] = true;

            // return the new point
            dla::ColorizedPoint {
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

   fn getOneDimensionalRepresentation(self) -> [u8; ONE_DIMENSIONAL_LENGTH as usize * 4] {
      let mut values = [0; ONE_DIMENSIONAL_LENGTH as usize * 4];
      for i in 0..ONE_DIMENSIONAL_LENGTH {
         let x = i % WIDTH;
         let y = i / HEIGHT;

         let ndx = i as usize * 4;

         if DLAField::isPositionOccupied(&self.positionHash, x, y) {
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

   fn generateEmptyPositionHash() -> Vec<Vec<bool>> {
      (0..WIDTH)
         .map(|x| {
            let col = (0..HEIGHT)
               .map(|y| {
                  false
               })
               .collect();

            col
         })
         .collect()
   }

   fn isPositionOccupied(positionHash: &Vec<Vec<bool>>, x: u32, y: u32) -> bool {
      positionHash[x as usize][y as usize]
   }
}
