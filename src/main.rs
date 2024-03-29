extern crate png;
extern crate time;

use time::{PreciseTime};
use png::HasParameters;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

mod dla;
use dla::DLAField;

const NUM_POINTS: i32 = 120000;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1920;

fn main() {
   let start = PreciseTime::now();
   let mut dlaField =
      DLAField::new(NUM_POINTS, WIDTH as usize, HEIGHT as usize);

   println!("initialized with: {} points", dlaField.getOccpupiedCount());

   let mut trials = 0;
   while !dlaField.nextState() {
      trials += 1;
   }

   println!("done in {}", start.to(PreciseTime::now()));
   println!("with {} stuck points out of {}", dlaField.getStuckCount(), NUM_POINTS);
   println!("ended with {} occupied points", dlaField.getOccpupiedCount());
   saveToPNG(&dlaField);
}

fn saveToPNG(dlaField: &DLAField) {
   let scale = 1;
   let path = Path::new(r"/Users/chris/projects/rustDLAFractals/testImageFractal.png");
   let file = File::create(path).unwrap();
   let ref mut w = BufWriter::new(file);

   let mut encocer = png::Encoder::new(w, WIDTH * scale, HEIGHT * scale);
   encocer.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
   let mut writer = encocer.write_header().unwrap();

   let data = dlaField.getOneDimensionalRepresentation(scale as usize);
   writer.write_image_data(&data).unwrap();
}
