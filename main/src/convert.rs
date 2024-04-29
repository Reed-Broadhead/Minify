use image::io::Reader as ImageReader;
use image::*; // Using image crate: https://github.com/image-rs/image
use webp::*; // Using webp crate: https://github.com/jaredforth/webp
use std::path::Path;

use std::fs::File;
use std::io::Write;

pub struct ImgFile {
    pub name: String,
    pub format: String,
    pub path: String,
    pub replace: bool,
}
impl ImgFile {
    pub fn to_webp(&self) {
        println!("Converting to PNG {:?}", self.path);

        let img: DynamicImage = ImageReader::open(&self.path).unwrap().decode().unwrap();

        let encoder: Encoder = Encoder::from_image(&img).unwrap();
        
        let webp: WebPMemory = encoder.encode(90f32);
         
        let file_name = if self.replace {self.name.clone()} else {"m_".to_owned() + &self.name};

        let output_path = Path::new("assets").join(file_name).with_extension("webp");
        std::fs::write(&output_path, &*webp).unwrap();
        println!("Saved to {:?}", output_path);

    }
}
