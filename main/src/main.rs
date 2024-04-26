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

}

fn main() {
    let args = Args::parse();

    
    let paths = fs::read_dir("./assets/").unwrap();
    println!("{:?}", paths);


    for path in paths {
        
        let mut place = (0,0);

        for (i, item) in path.as_ref().unwrap().path().display().to_string().chars().rev().enumerate() {
            match item {
                '.' => place.1 = (path.as_ref().unwrap().path().display().to_string().len() - i).try_into().unwrap(),
                '/' => { place.0 = (path.as_ref().unwrap().path().display().to_string().len() - i).try_into().unwrap(); break},
                _ => continue,
            }
        }

        let name = &path.as_ref().unwrap().path().display().to_string()[place.0..place.1-1];

        match &args.file {
            None => convert::convert(&path.as_ref().unwrap().path().display().to_string(), &name), 
            Some(_) => {
                if name == args.file.clone().unwrap() {
                    println!("Found {:?}", path.as_ref().unwrap().path().display().to_string());    
                    convert::convert(&path.as_ref().unwrap().path().display().to_string(), &name );
                } },  
        }
    };
//    println!("No file named {:?} found", args.file.clone().unwrap());
}

