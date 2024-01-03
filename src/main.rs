
use rayon::prelude::*;
use image::{GenericImageView, ImageBuffer};

fn main() {

    //INICIAMOS CONTADOR DE TIEMPO
    let now = std::time::Instant::now();

    let image = image::open("src/red.jpg").unwrap();
    let (width, height) = image.dimensions();
    println!("width: {}, height: {}", width, height);
    let image_bytes = image.into_bytes();
    

    black_and_white(image_bytes, width, height);

    //FINALIZAMOS CONTADOR DE TIEMPO
    let elapsed = now.elapsed();
    println!("Tiempo de ejecucion: {:?}", elapsed);

}

fn black_and_white(image_bytes: Vec<u8>, width: u32, height: u32)
{
    let num_threads = rayon::current_num_threads();
    println!("Número de hilos: {}", num_threads);

    let new_image_pixels: Vec<u8> = image_bytes.par_chunks(3)
        .map(|chunk| {
            let suma: i32 = chunk.iter().map(|&x| x as i32).sum();
            let promedio: u8 = (suma / 3) as u8;
            promedio as u8
        }).collect();
        // let mut new_image_pixels: Vec<u8> = Vec::new();
    // for i in (0..image_bytes.len()).step_by(3){
    //     let suma: i32 = image_bytes[i..i+3].iter().map(|&x| x as i32).sum();
    //     let promedio: u8 = (suma / 3) as u8;
    //     new_image_pixels.push(promedio as u8);
    // }

    // GUARDAR LA IMAGEN new_image_pixels
    let mut new_image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for (x, y, pixel) in new_image.enumerate_pixels_mut() {
        let index = (x + y * width) as usize;
        *pixel = image::Rgb([new_image_pixels[index], new_image_pixels[index], new_image_pixels[index]]);
    }
    
    // let mut new_image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    // new_image.enumerate_pixels_mut().par_bridge().for_each(|(x,y,pixel)|{
    //     let index = (x + y * width) as usize;
    //     *pixel = image::Rgb([new_image_pixels[index], new_image_pixels[index], new_image_pixels[index]]);
    // });

    new_image.save("src/out_test.png").unwrap();
}


