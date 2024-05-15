use image::io::Reader as ImageReader;
//use image::*; // Using image crate: https://github.com/image-rs/image
use webp::*; // Using webp crate: https://github.com/jaredforth/webp
use std::path::Path;

pub struct ImgFile {
    pub name: String,
    pub format: String,
    pub path: String,
    pub replace: bool,
    pub quality: f32,
}
impl ImgFile {
    pub fn to_webp(&self) {
        println!("Converting to PNG {:?}", &self.path);

        let img_results = ImageReader::open(&self.path).unwrap().decode();

        let img = match img_results{
           Ok(file) => file,
           Err(_) => {println!("Could not convert: {:?}", &self.path); return } 
        };

        let encoder_results = Encoder::from_image(&img);

        let encoder = match encoder_results {
            Ok(file) => file,
            Err(_) => {println!("Could not convert: {:?}", &self.path); return }
        }; 

        let webp: WebPMemory = encoder.encode(self.quality);

        let output_path = Path::new("").join(&self.name).with_extension("webp");

        let _ = if self.replace == true {
             std::fs::remove_file(&self.path)
        } else {Ok(())};
        
        std::fs::write(&output_path, &*webp).unwrap();
        println!("Saved to {:?}", output_path);

    }
    pub fn compress(&self) {
            println!("Compressing {:?}", &self.path);
    }
}
