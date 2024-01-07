use std::fs::File;

use gif::Repeat;
use image::{ImageBuffer, Rgb};
use rand::Rng;

fn create_random() -> [bool;800]{
    let mut arr:[bool; 800] = [false; 800];
    let mut rng = rand::thread_rng();
    for _ in 0..rng.gen_range(0..arr.len()) {
        let r = rng.gen_range(0..arr.len());
        arr[r] = true
    }
    arr
}

fn write_buffer() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let imgx = 800;
    let imgy = 800;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let r_x = create_random();
    let r_y = create_random();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut()  {
        
        let r = match r_x[x as usize] {
            true => 255,
            _ => 0
        };
        let b = match r_y[y as usize] {
            true => 255,
            _ => 0
        };
        *pixel = image::Rgb([r, 0_u8,  b]);
    }
        imgbuf

}

fn main() {
    let mut image = File::create("gifs/tartan.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut image, 800,800, &[]).unwrap();

    for _ in 0..10 {
        let img = write_buffer();
        let frame = gif::Frame::from_rgb(800,800,&img);
        let _ = encoder.write_frame(&frame);
    }
    encoder.set_repeat(Repeat::Infinite).unwrap();

    println!("Done.")
}