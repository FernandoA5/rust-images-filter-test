use image::{GenericImageView, ImageBuffer, DynamicImage};

fn main() {
    let img = image::open("src/test.jpg");
    let img_clone = img.unwrap().clone();

    let width = img_clone.width();
    let height = img_clone.height();

    //INFORMACION DE LA IMAGEN
    println!("dimensions {:?}", img_clone.dimensions());

    let image_bytes = img_clone.into_bytes();

    let mut new_image_pixels: Vec<u8> = Vec::new();
    for i in 0..image_bytes.len(){
        if i % 3 == 0 &&  i<image_bytes.len()-3{
            let sum = (image_bytes[i] as i32 + image_bytes[i+1] as i32 + image_bytes[i+2] as i32) / 3;
            new_image_pixels.push(sum as u8);
        }
        if i == image_bytes.len()-1{
            let sum = (image_bytes[i] as i32 + image_bytes[i-1] as i32 + image_bytes[i-2] as i32) /3;
            new_image_pixels.push(sum as u8);
        }
    }
    //GUARDAR LA IMAGEN new_image_pixels
    let mut new_image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for (x, y, pixel) in new_image.enumerate_pixels_mut() {
        let index = (x + y * width) as usize;
        *pixel = image::Rgb([new_image_pixels[index], new_image_pixels[index], new_image_pixels[index]]);
    }
    new_image.save("src/out_test.png").unwrap();

}


