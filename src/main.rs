extern crate png;
use png::HasParameters;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

mod dla;
use dla::DLAField;

const NUM_POINTS: i32 = 5000;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const ONE_DIMENSIONAL_LENGTH: u32 = WIDTH * HEIGHT;

fn main() {
   let mut dlaField =
      DLAField::new(NUM_POINTS, WIDTH as usize, HEIGHT as usize);

   let mut trials = 0;
   while !dlaField.nextState() {
      if trials % 10 == 0 {
         println!("state loop running");
         saveToPNG(&dlaField);
      }

      trials += 1;
   }

   println!("done");
   saveToPNG(&dlaField);
}

fn saveToPNG(dlaField: &DLAField) {
   let path = Path::new(r"/Users/chris/projects/rustDLAFractals/testImageFractal.png");
   let file = File::create(path).unwrap();
   let ref mut w = BufWriter::new(file);

   let mut encocer = png::Encoder::new(w, WIDTH, HEIGHT);
   encocer.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
   let mut writer = encocer.write_header().unwrap();

   let data = dlaField.getOneDimensionalRepresentation();
   writer.write_image_data(&data).unwrap();
}
