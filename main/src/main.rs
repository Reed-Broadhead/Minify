use clap::{Parser, Subcommand};
use std::fs;

mod convert;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands{
    Convert {
        #[arg(short, long)]
        file: Option<String>,

        quality: Option<f32>,

        #[arg(short, long)]
        replace: bool
    },
    Compress {
        #[arg(short, long)]
        file: Option<String>,

        #[arg(short, long)]
        quality: Option<f32>,

        #[arg(short, long)]
        replace: bool,

        #[arg(short, long)]
        target: Option<String>,
    }
}

fn main() {
    let args = Args::parse();

    match &args.commands {
         Some(Commands::Convert {file, quality, replace}) => {
               process_args( file, quality, replace, "converting", &None); 
         },
         Some(Commands::Compress {file, quality, replace, target }) => {
             process_args(file, quality, replace, "compressing", target );
         },
         None => (),
        }

}
fn process_args(file: &Option<String>, quality: &Option<f32>, replace: &bool, operation: &str, file_type: &Option<String>) {

    let paths = fs::read_dir("./assets/").unwrap();

    let mut operate: bool = false;
    
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
            replace: *replace, 
            quality: match quality {
                Some(x) => {
                    if x * 10.0 > 100.0 {
                        println!("{x} is not a valid value. \nplease enter a number between 1 and 10"); 
                        operate = true;
                        break
                    } else {x * 10.0}
                 },
                None => 100.0,
            },
        };

        match &file {
            Some(x) => {
                if &img.name != x {continue};
            },
            None => (),
        }
        match operation {
            "converting" => {
                if vec!["png", "jpg"].contains(&img.format.to_lowercase().as_str()) { 
                    img.to_webp();
                    operate = true
                } 
            },
            "compressing" => {
               match &file_type {
                    Some(x) => {
                        if img.format.to_lowercase() == x.to_lowercase() {
                            img.compress();
                            operate = true
                        }
                    },
                    None => {
                        img.compress();
                        operate = true 
                    }
                } 
                
            },
            &_ => todo!(),
        } 
    };
    if operate == false {
        println!("No files match {:?}", file); 
    } else {
        println!("Operation complete")
    }; 
}
