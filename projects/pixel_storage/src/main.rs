
extern crate image;
use std::{fs, task::Context};
use std::path::Path;
use image::{ImageBuffer, Rgb};
use std::time::{Instant};

#[derive(Debug)]
struct Dimensions{
    width:u32,
    height:u32,
}


fn main() {
    //Time calculation
    let start_time = Instant::now();

    let path = "./src/files/input.txt";
    let path = Path::new(path);
    let contents = fs::read_to_string(path).expect("Unanble to read the input.txt file");

    let content = contents.as_bytes();


    let dimensions = get_dimensions(&contents.len());

    let image: ImageBuffer<Rgb<u8>, Vec<u8>>= ImageBuffer::new(dimensions.width, dimensions.height);

    println!("{:?}, Length {:?}", dimensions, contents.len());

    generate_image(image, content, dimensions);

    //time calculation
    let elapsed_time = start_time.elapsed();
    println!("\n\n\t\t- - - E X E C U T I O N  T I M E: {}.{:03} - - -", elapsed_time.as_secs(), elapsed_time.subsec_millis());
}

fn generate_image(image: ImageBuffer<Rgb<u8>, Vec<u8>>, content:&[u8], dimensions:Dimensions){
    println!("Element: {}", content[0]);

    for chunk in content.chunks(3) {
        //if length is not 3 then do someting else
        println!("{:?}", chunk);
    }

    image.save("./src/files/encoded_image.png").unwrap();
}

fn get_dimensions(length: &usize) -> Dimensions {
    let length = (length / 3) + 1 ;

    println!("Length divided: {}",length);

    let width = (length as f64).sqrt();
    let width = (width as u32) + 1; // takes the square root and then plus one. now go through and see when the lenght is enough

    let mut height = 0;
    loop {
        height += 1;
        if (height * width) > length as u32 {
            return Dimensions{width, height};
        }
    }
}
