use std::env;
use std::error::Error;
use std::process;
use std::fs;
use std::ffi::OsStr;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() -> std::io::Result<()>  {
    let args: Vec<String> = env::args().collect();
	
	let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
	
	let mut filesToShuffle: Vec<fs::DirEntry> = Vec::new();
	let mut fileNames: Vec<String> = Vec::new();

    for entry in fs::read_dir(config.path.as_path()).unwrap() {
		let path = entry.unwrap();
		let path_int = path.path();
		let extension = path_int.extension();
		if extension.is_some() && !config.exceptions.contains(&extension.unwrap().to_str().unwrap().to_lowercase()) {
			filesToShuffle.push(path);
			fileNames.push(path_int.file_stem().unwrap().to_str().unwrap().to_string())
		}
        //println!("Name: {} , extension: {:?}", path_int.display(), extension.unwrap().to_str())
    }
	// sort files 
	//filesToShuffle.sort_by(|a,b| a.path().file_stem().unwrap().to_str().unwrap().parse::<i32>().unwrap().cmp(&b.path().file_stem().unwrap().to_str().unwrap().parse::<i32>().unwrap()));
	for file in &filesToShuffle {
		let path_int = file.path();
		let extension = path_int.extension();
		println!("name: {} , extension: {:?}", path_int.display(), extension.unwrap().to_str())
	}
	
	for name in &fileNames {
		println!("file name: {} ", name)
	}
	
	fileNames.shuffle(&mut thread_rng());
	
	println!("After shuffle. ");
	
	for name in &fileNames {
		println!("file name: {} ", name)
	}
	
	for iter in filesToShuffle.iter().zip(fileNames.iter()) {
		
	}
	//create vector of positions
//	let mut positionVector: Vec<i32> = (0..filesToShuffle.len()).collect(); 
//	for pos in positionVector {
//		println!("name: {} , extension: {:?}", path_int.display(), extension.unwrap().to_str())
//	}
	 Ok(())
}

fn print_help()
{
	println!("Available flags: ");
	println!("-h/--help - displays this message ");
	println!("-p/--path PATH - sets PATH as path where the script shall be executed. Default path is the script location.");
	println!("-f/--flatten - enables the flatten mode, which doesnt shuffle, but tries to bring file numbers to be in order.");
	println!("-e/--exceptions EXCEPTIONS - sets which file extensions shall be excepted. Separate exceptions by comma, without spaces e.g xls,doc,txt. Default exceptions: txt,xls,obs,doc");
}


struct Config{
	path: std::path::PathBuf,
	alt_mode: bool,
	exceptions: Vec<String>
}

impl Config {
    fn new(args: &[String]) -> Result<Config,  Box<dyn Error>> {
		let mut path = env::current_dir()?;
		println!("Test {}", args[0].as_str());
		let mut alt_mode = false;
		let mut exceptions : Vec<String> = vec!["txt".into(), "xls".into(), "obs".into(), "doc".into()];
		let mut argument_number = 1;
	
		while argument_number < args.len() {
			match args[argument_number].as_str() {
				"-h" | "--help" => print_help(),
				"-p" | "--path" => {argument_number = argument_number+1;
									path = args[argument_number].as_str().into();},
				"-f" | "--flatten" => alt_mode = true,
				"-e" | "--exception" => {argument_number = argument_number+1;
										 exceptions = args[argument_number].as_str().split(',').map(|s| s.to_string()).collect();},
				_ => print_help()
			}
			argument_number = argument_number+1;
		}

		println!("The current directory is {}, exceptions are {:?} and flatten mode is set to {} ", path.display(), exceptions, alt_mode );
        Ok(Config { path, alt_mode, exceptions })
    }
}