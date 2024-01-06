use std::process::exit;
// use rayon::prelude::*;
use image::{GenericImageView, ImageBuffer};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const RGB_SIZE: usize = 3;
const FILTER: usize = 6;

fn main() {

    //INICIAMOS CONTADOR DE TIEMPO
    let now = std::time::Instant::now();

    let image = image::open("img/gta.jpg").unwrap();
    let (width, height) = image.dimensions();
    println!("width: {}, height: {}", width, height);
    let image_bytes = image.into_bytes();
    
    let new_image_pixels ;

    match FILTER {
        1 => new_image_pixels = black_and_white(image_bytes, width),
        2 => new_image_pixels = blur(image_bytes, width, 5),
        3 => new_image_pixels = color(image_bytes, width, Vec::from([255, 255, 255])),
        4 => new_image_pixels = borders(image_bytes, width),
        5 => new_image_pixels = better_borders(image_bytes, width),
        6 => new_image_pixels = sharp(image_bytes, width, 2),
        _ => exit(1)
        
    }


    //FINALIZAMOS CONTADOR DE TIEMPO
    let elapsed = now.elapsed();
    println!("Tiempo de ejecucion: {:?}", elapsed);

    save_matrix_image(&new_image_pixels, width, height, "out-image");
}

fn black_and_white(image_bytes: Vec<u8>, width: u32) -> Vec<Vec<Vec<u8>>> {

    let pixels_matrix = get_pixel_matrix(image_bytes, width);

    //Aplicar el filtro (PARALELIZADO)
    let new_image_pixels: Vec<Vec<Vec<u8>>> = pixels_matrix.par_iter().map(|row| {
        row.par_iter().map(|pixel| {
            let suma: i32 = pixel.iter().map(|&x| x as i32).sum();
            let promedio: u8 = (suma / 3) as u8;
            Vec::from([promedio, promedio, promedio])
        }).collect()
    }).collect();

    new_image_pixels
}
fn blur(image_bytes: Vec<u8>, width: u32, intensidad: u8) -> Vec<Vec<Vec<u8>>>{
    let mut pixels_matrix = get_pixel_matrix(image_bytes, width);
    let mut new_pixels_matrix = pixels_matrix.clone();
    //Aplicar el filtro
    let kernel: [[i32; 3]; 3] = [
        [1, 1, 1],
        [1, 1, 1],
        [1, 1, 1],
    ];
    for _ in 0..intensidad as usize {
        for row in 1..pixels_matrix.len()- 1{
            for pixel in 1..pixels_matrix[row].len() - 1{
    
                for i in 0..RGB_SIZE{
                    let mut sum: i32 = 0;
                    for j in 0..3 as usize{
                        for k in 0..3 as usize{

                            let pixel_value = pixels_matrix[row-1+k][pixel-1+j][i] as i32;

                            sum += pixel_value * kernel[k][j] as i32;
                        }
                    }
                    let promedio = (sum / 9) as u8;
                    new_pixels_matrix[row][pixel][i] = promedio;
                }

                //PODEMOS CALCULAR PRIMERO LAS VENTANAS Y LUEGO RECORRERLAS 

                //QUIZÁ DE ESA FORMA PODAMOS ELUDIR LAS RESTRICCIONES DE PRESTAMOS MUTABLES


            }
        }
        pixels_matrix = new_pixels_matrix.clone();
    }
    new_pixels_matrix

}
fn color(image_bytes: Vec<u8>, width: u32, color: Vec<u8>) -> Vec<Vec<Vec<u8>>>{
        
        let mut pixels_matrix = get_pixel_matrix(image_bytes, width);

        //Aplicar el filtro
        for row in 1..pixels_matrix.len()- 1{
            for pixel in 1..pixels_matrix[row].len() - 1{
                pixels_matrix[row][pixel] = pixels_matrix[row][pixel].iter().zip(color.iter()).map(|(x, y)| x ^ y).collect();
            }
        }
        pixels_matrix
}


fn better_borders(image_bytes: Vec<u8>, width: u32) -> Vec<Vec<Vec<u8>>>{
    let pixels_matrix = get_pixel_matrix(image_bytes, width);
    let mut new_image_pixels: Vec<Vec<Vec<u8>>> = pixels_matrix.clone();

    //KERNEL PARA SHARPEN
    let kernel: [[i32; 3]; 3] = [
        [-1, 0, 1],
        [-2, 0, 2],
        [-1, 0, 1],
    ];
    //KERNEL 2
    let kernel2: [[i32; 3]; 3] = [
        [-1, -2, -1],
        [0, 0, 0],
        [1, 2, 1],
    ];

    //Aplicar el filtro
    for row in 1..pixels_matrix.len()- 1{
        for pixel in 1..pixels_matrix[row].len() - 1{

            let mut sum: i32 = 0;
            let mut sum2: i32 = 0;

            for rgb in 0..RGB_SIZE{
                for j in 0..3 as usize{
                    for k in 0..3 as usize{

                        let pixel_value = pixels_matrix[row-1+k][pixel-1+j][rgb] as i32;

                        sum += pixel_value * kernel[k][j] as i32;
                        sum2 += pixel_value * kernel2[k][j] as i32;
                    }
                }
                // new_image_pixels[row][pixel][rgb] = ((sum + sum2) / 2)  as u8;
                let cuadrados: f64 = (sum as f64).powf(2.0) + (sum2 as f64).powf(2.0);
                new_image_pixels[row][pixel][rgb] = cuadrados.sqrt() as u8;
            }

            

        }
    }
    new_image_pixels

}

fn sharp(image_bytes: Vec<u8>, width: u32, intensidad: u8) -> Vec<Vec<Vec<u8>>> {
    let mut pixels_matrix = get_pixel_matrix(image_bytes, width);
    let mut new_image_pixels: Vec<Vec<Vec<u8>>> = pixels_matrix.clone();
    //KERNEL PARA SHARPEN
    let kernel: [[i32; 3]; 3] = [
        [0, -1, 0],
        [-1, 5, -1],
        [0, -1, 0],
    ];

    //Aplicar el filtro
    for _ in 0..intensidad as usize {
        for row in 1..pixels_matrix.len()- 1{
            for pixel in 1..pixels_matrix[row].len() - 1{
    
                let mut sum: i32 = 0;
                    for j in 0..3 as usize{
                        for k in 0..3 as usize{
    
                            let pixel_value = pixels_matrix[row-1+k][pixel-1+j][0] as i32;
    
                            sum += pixel_value * kernel[k][j] as i32;
                        }
                    }
    
                    let last_r = new_image_pixels[row][pixel][0] as i32;
                    let last_g = new_image_pixels[row][pixel][1] as i32;
                    let last_b = new_image_pixels[row][pixel][2] as i32;
    
                    //REGLA DE 3
                    let mut g = last_g * sum / last_r.max(1);
                    let mut b = last_b * sum / last_r.max(1);
    
                    sum = if sum > 255 { 255 } else { sum };
                    sum = if sum < 0 { 0 } else { sum };
    
                    g = if g > 255 { 255 } else { g };
                    g = if g < 0 { 0 } else { g };
    
                    b = if b > 255 { 255 } else { b };
                    b = if b < 0 { 0 } else { b };
                    
                    new_image_pixels[row][pixel][0] = (sum)  as u8;
                    new_image_pixels[row][pixel][1] = (g)  as u8;
                    new_image_pixels[row][pixel][2] = (b)  as u8;
                }
        }
        pixels_matrix = new_image_pixels.clone();
    }
    new_image_pixels
}


fn borders(image_bytes: Vec<u8>, width: u32) -> Vec<Vec<Vec<u8>>>{
    let pixels_matrix = get_pixel_matrix(image_bytes, width);
    let mut new_image_pixels: Vec<Vec<Vec<u8>>> = pixels_matrix.clone();

    //KERNEL PARA SHARPEN
    let kernel: [[i32; 3]; 3] = [
        [-1, -1, -1],
        [-1, 8, -1],
        [-1, -1, -1],
    ];

    //Aplicar el filtro
    for row in 1..pixels_matrix.len()- 1{
        for pixel in 1..pixels_matrix[row].len() - 1{

            let mut sum: i32 = 0;

            for rgb in 0..RGB_SIZE{
                for j in 0..3 as usize{
                    for k in 0..3 as usize{

                        let pixel_value = pixels_matrix[row-1+k][pixel-1+j][rgb] as i32;

                        sum += pixel_value * kernel[k][j] as i32;
                    }
                }

                sum = if sum > 255 { 255 } else { sum };
                sum = if sum < 0 { 0 } else { sum };

                new_image_pixels[row][pixel][rgb] = (sum)  as u8;
            }

            

        }
    }
    new_image_pixels
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

    new_image.save(format!("img/{}.png", filename )).unwrap();
}
