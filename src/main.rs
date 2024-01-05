use std::process::exit;

// use rayon::prelude::*;
use image::{GenericImageView, ImageBuffer};

const RGB_SIZE: usize = 3;

fn main() {

    //INICIAMOS CONTADOR DE TIEMPO
    let now = std::time::Instant::now();

    let image = image::open("src/test2.jpg").unwrap();
    let (width, height) = image.dimensions();
    println!("width: {}, height: {}", width, height);
    let image_bytes = image.into_bytes();
    

    // black_and_white(image_bytes, width, height);
    // blur(image_bytes, width, height, 1)K;
    color(image_bytes, width, height, Vec::from([255, 0, 255]));

    //FINALIZAMOS CONTADOR DE TIEMPO
    let elapsed = now.elapsed();
    println!("Tiempo de ejecucion: {:?}", elapsed);


}

fn black_and_white(image_bytes: Vec<u8>, width: u32, height: u32)
{
    // let num_threads = rayon::current_num_threads();
    // println!("Número de hilos: {}", num_threads);

    // let new_image_pixels: Vec<u8> = image_bytes.par_chunks(3)
    //     .map(|chunk| {
    //         let suma: i32 = chunk.iter().map(|&x| x as i32).sum();
    //         let promedio: u8 = (suma / 3) as u8;
    //         promedio as u8
    // }).collect();

    let mut pixels_matrix = get_pixel_matrix(image_bytes, width);

    //Aplicar el filtro
    for row in 1..pixels_matrix.len()- 1{
        for pixel in 1..pixels_matrix[row].len() - 1{

            for i in 0..RGB_SIZE{
                for j in 0..3 as usize{
                    let mut sum: u32 = 0;
                    for k in 0..3 as usize{
                        sum += pixels_matrix[row-1+i][pixel-1+j][k] as u32;
                    }
                    let promedio = (sum / RGB_SIZE as u32) as u8;
                pixels_matrix[row][pixel][i] = promedio;
                }
            }
        }
    }

    save_matrix_image(&pixels_matrix, width, height, "out_black_and_white");
}
fn blur(image_bytes: Vec<u8>, width: u32, height: u32, intensidad: u8){
    
    let mut pixels_matrix = get_pixel_matrix(image_bytes, width);
    //Aplicar el filtro
    for _ in 0..intensidad as usize {
        for row in 1..pixels_matrix.len()- 1{
            for pixel in 1..pixels_matrix[row].len() - 1{
    
                for i in 0..RGB_SIZE{
                    let mut sum: u32 = 0;
                    for j in 0..3 as usize{
                        for k in 0..3 as usize{
    
                            let condicion = k == 1 && j == 1;
                            let op = if condicion { 1 } else { 1 };
    
                            sum += pixels_matrix[row-1+k][pixel-1+j][i] as u32 * op;
                        }
                    }
                    let promedio = (sum / 9) as u8;
                    pixels_matrix[row][pixel][i] = promedio;
                }
            }
        }
    }

    save_matrix_image(&pixels_matrix, width, height, "out_blur");

}
fn color(image_bytes: Vec<u8>, width: u32, height: u32, color: Vec<u8>){
        
        let mut pixels_matrix = get_pixel_matrix(image_bytes, width);

        //Aplicar el filtro
        for row in 1..pixels_matrix.len()- 1{
            for pixel in 1..pixels_matrix[row].len() - 1{
                pixels_matrix[row][pixel] = pixels_matrix[row][pixel].iter().zip(color.iter()).map(|(x, y)| x ^ y).collect();
            }
        }
        
    
        save_matrix_image(&pixels_matrix, width, height, "out_shape");
}


fn get_pixel_matrix(image_bytes: Vec<u8>, width: u32) -> Vec<Vec<Vec<u8>>>
{
    //Trabajemoslo como una matriz (Convertir el vector en una matriz)
    let mut pixels_array: Vec<Vec<u8>> = Vec::new();
    for i in (0..image_bytes.len()).step_by(3){
        let mut row: Vec<u8> = Vec::new();
        
        image_bytes[i..i+3].iter().for_each(|x| row.push(*x));

        pixels_array.push(row);
    }
    // println!("Array: {:?}\n", pixels_array);

    //Necesito una matríz de pixeles. Usando el ancho y el alto de la imagen.
    let mut pixels_matrix: Vec<Vec<Vec<u8>>> = Vec::new();

    for i in (0..pixels_array.len()).step_by(width as usize){
        let mut row: Vec<Vec<u8>> = Vec::new();
        //ESTO ESTÁ ALMACENANDO SOLO 1 BYTE POR PIXEL
        pixels_array[i..i+width as usize].iter().for_each(|x| row.push(x.to_vec()));
        pixels_matrix.push(row);
    }

    pixels_matrix
}

fn save_matrix_image(pixels_matrix: &Vec<Vec<Vec<u8>>>, width: u32, height: u32, filename: &str){
    let mut new_image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for (x, y, pixel) in new_image.enumerate_pixels_mut() {
        let x = x as usize;
        let y = y as usize;
        *pixel = image::Rgb([pixels_matrix[y][x][0], pixels_matrix[y][x][1], pixels_matrix[y][x][2]]);
    }

    new_image.save(format!("src/{}.png", filename )).unwrap();
}
