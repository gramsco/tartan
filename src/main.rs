use std::fs::{File, self};

use gif::Repeat;
use image::{ImageBuffer, Rgb};
use rand::Rng;
use rayon::prelude::*;

const SIZE :usize= 400;

fn create_random() -> [u8;SIZE]{
    let mut arr:[u8; SIZE] = [0; SIZE];
    let mut rng = rand::thread_rng();
    for i in 0..SIZE{
        arr[i] = rng.gen_range(0..255);
    }
    arr
}

fn write_buffer() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let imgx = SIZE;
    let imgy = SIZE;

    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);

    let r_x = create_random();
    let r_y = create_random();
    let mut rng = rand::thread_rng();
    
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut()  {
        let r = r_x[x as usize];
        let g = rng.gen_range(0..255);
        let b = r_y[y as usize] ;

        *pixel = image::Rgb([r, g,  b]);
    }
        imgbuf

}

fn get_filename(depth:u8) -> String{ 
    let depth_str = match depth {
        0 => String::from(""),
        other => format!("_{}", other)
    };
    match fs::read(format!("gifs/tartan{depth_str}.gif")) {
        Ok(_) => {
            get_filename(depth + 1)
        }
        Err(_) => {
            format!("gifs/tartan{depth_str}.gif")
        }
    }
}

fn main() {
    let mut image = File::create(get_filename(0)).unwrap();
    let mut encoder = gif::Encoder::new(&mut image, SIZE as u16,SIZE as u16, &[]).unwrap();
    let mut vec = vec![];
   
     (0..10).into_par_iter().map(|_| {
        let img = write_buffer();
        gif::Frame::from_rgb(SIZE as u16,SIZE as u16,&img)
    }).collect_into_vec(&mut vec);
   
   for frame in vec {
       let _ = encoder.write_frame(&frame);
   }
    
    encoder.set_repeat(Repeat::Infinite).unwrap();

    println!("Done.")
}