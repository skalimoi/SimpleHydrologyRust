use errorfunctions::RealErrorFunctions;
use image::io::Reader as ImageReader;
use image::ImageBuffer;
use image::Luma;
use image::Rgb;
use world::World;

pub mod map;
pub mod water;
pub mod world;

use world::Vec2;

const SEED: u64 = 1985;
const CYCLES: i32 = 100;
const FILE_NAME: &str = "heightmap.png";

fn main() {
    let img = ImageReader::open(FILE_NAME).unwrap().decode().unwrap().into_luma16();
    let (width, height) = img.dimensions();
    let heightmap = img.into_raw();
    let mut world = World::new(heightmap, width as usize, height as usize, SEED);

    let mut discharge_map = vec![0; (width * height) as usize];
    let mut momentum_map = vec![0; (width * height * 3) as usize];

    use std::time::Instant;
    let now = Instant::now();
    for cycle in 0..CYCLES {
        world.erode(width as usize);
        println!("{}", cycle);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    for i in 0..discharge_map.len() {
        let pos = Vec2::new(i as f64 % width as f64, (i / width as usize) as f64);
        discharge_map[i] = ((world.map.discharge(pos) + 1.0) * 0.5 * 255.0) as u8;
    }
    for i in 0..(momentum_map.len() / 3) {
        let pos = Vec2::new(i as f64 % width as f64, (i / width as usize) as f64);
        let cell = world.map.get(pos).unwrap();
        momentum_map[i * 3] = ((cell.momentum.x.erf() + 1.0) * 0.5 * 255.0) as u8;
        momentum_map[i * 3 + 1] = ((cell.momentum.y.erf() + 1.0) * 0.5 * 255.0) as u8;
        momentum_map[i * 3 + 2] = 255 / 2;
    }
    let heightmap = world.map.heightmap.iter().map(|x| (x.height * 255.0) as u16).collect();
    let heightmap_buffer: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(width, height, heightmap).unwrap();
    heightmap_buffer.save(format!("heightmap_.png")).unwrap();

    let discharge_buffer: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, discharge_map.clone()).unwrap();
    discharge_buffer.save(format!("discharge_.png")).unwrap();

    let momentum_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, momentum_map.clone()).unwrap();
    momentum_buffer.save(format!("momentum_.png")).unwrap();
}
