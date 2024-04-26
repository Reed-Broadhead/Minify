use image::io::Reader as ImageReader;
use image::*; // Using image crate: https://github.com/image-rs/image
use webp::*; // Using webp crate: https://github.com/jaredforth/webp
use std::path::Path;

use std::fs::File;
use std::io::Write;

pub fn convert(file_path: &str, name: &str) {

    let mut place: usize = 0;
 
    for (i, item) in file_path.chars().rev().enumerate() {
        if item == '.' {
            place = (file_path.len() - i).try_into().unwrap();
            break;
        }
    } 
    let format = &file_path[place..file_path.len()];

    match format.to_lowercase().as_str() {
       "png" => to_webp(file_path, name), 
       "jpg" => to_webp(file_path, name),
        _ => println!("Err: [File format not supported] {:?}", file_path), 
    }
}

fn to_webp(file_path: &str, name: &str) {
   let mut place: [usize; 2] = [0,0]; 
   
   for (i, item) in file_path.chars().rev().enumerate() {
       match item{
            '.' => place[1] = (file_path.len() -  i).try_into().unwrap(),
            '/' => { place[0] = (file_path.len() -  i).try_into().unwrap(); break},
            _ => continue,
       }
   } 

//   let format = "m_".to_owned() + &file_path[place[0]..place[1]-1];

   let format = "m_".to_owned() + name; 

    println!("Converting to PNG {:?}", file_path);

    let img: DynamicImage = ImageReader::open(file_path).unwrap().decode().unwrap();

    let encoder: Encoder = Encoder::from_image(&img).unwrap();
        
    let webp: WebPMemory = encoder.encode(90f32);

    let output_path = Path::new("assets").join(format).with_extension("webp");
    std::fs::write(&output_path, &*webp).unwrap();
    println!("Saved to {:?}", output_path);
}
