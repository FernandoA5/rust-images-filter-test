use std::iter;

use image::{GenericImageView, ImageBuffer, DynamicImage};

fn main() {
    let img = image::open("src/test.png");
    let img_clone = img.unwrap().clone();

    let width = img_clone.width();
    let height = img_clone.height();

    //INFORMACION DE LA IMAGEN
    println!("dimensions {:?}", img_clone.dimensions());

    let image_bytes = img_clone.into_bytes();

    let mut new_image_pixels: Vec<u8> = Vec::new();
    for i in (0..image_bytes.len()).step_by(3){
        let suma: i32 = image_bytes[i..i+3].iter().map(|&x| x as i32).sum();
        let promedio: u8 = (suma / 3) as u8;
        new_image_pixels.push(promedio as u8);
    }
    //GUARDAR LA IMAGEN new_image_pixels
    let mut new_image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for (x, y, pixel) in new_image.enumerate_pixels_mut() {
        let index = (x + y * width) as usize;
        *pixel = image::Rgb([new_image_pixels[index], new_image_pixels[index], new_image_pixels[index]]);
    }
    new_image.save("src/out_test.png").unwrap();

}


