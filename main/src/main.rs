use clap::Parser;
use std::fs;

mod convert;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
//    #[arg(short, long, default_value_t = 1)]
//    count: u8,

    #[arg(short, long, default_missing_value = "All")]
    file: Option<String>,

    #[arg(short, long)]
    quality: Option<f32>,

    #[arg(short, long)]
    replace: bool,

}

fn main() {
    let args = Args::parse();

    let paths = fs::read_dir("./assets/").unwrap();
//    println!("{:?}", args.quality);
    
    for path in paths {

        let path = path.unwrap().path().display().to_string();
        
        let mut place = (0,0);

        for (i, item) in path.chars().rev().enumerate() {
            match item {
                '.' => place.1 = (path.len() - i).try_into().unwrap(),
                '/' => { place.0 = (path.len() - i).try_into().unwrap(); break},
                _ => continue,
            }
        }
        
        let img = convert::ImgFile{
            name: (path[place.0..place.1-1]).to_string(), 
            format: (path[place.1..path.len()]).to_string(),
            path: path,
            replace: args.replace,
            quality: match args.quality {
                Some(x) => {
                    if x * 10.0 > 100.0 {
                        println!("{x} is not a valid value. \nplease enter a number between 1 and 10"); 
                        break
                    } else {x * 10.0}
                 },
                None => 100.0,
            },
        };
        if img.name != args.file.clone().unwrap() {continue}; 

        if vec!["png", "jpg"].contains(&img.format.to_lowercase().as_str()) { 
            img.to_webp();
        };
    };
}

