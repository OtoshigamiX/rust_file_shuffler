use std::env;
use std::error::Error;
use std::process;
use std::fs;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::Write;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

#[derive(Debug)]
#[derive(PartialEq)]
enum Mode {
	Standard,
	Simple,
	Flatten,
}

fn get_file_name(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|s| s.to_string())
}

fn get_extension(path_buf: &PathBuf) -> Option<String> {
    path_buf
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}

fn get_files_in_directory(config: & Config) -> Result<(Vec<fs::DirEntry>, Vec<String>),  Box<dyn Error>> {
	let mut filepaths_to_shuffle: Vec<fs::DirEntry> = Vec::new();
	let mut filenames_without_extensions: Vec<String> = Vec::new();
	
	for entry in fs::read_dir(config.path.as_path())? {
		let path = entry?;
		let path_int = path.path();
		let extension = get_extension(&path_int);
		if extension.is_some() && !config.exceptions.contains(&extension.ok_or("No extension!")?) {
			let file_name = get_file_name(&path_int)
                .ok_or_else(|| format!("Could not get file name for {}", path_int.display()))?;
															  
			filepaths_to_shuffle.push(path);
			filenames_without_extensions.push(file_name);
		}
    }

	Ok((filepaths_to_shuffle, filenames_without_extensions))
}

fn shuffle_files_and_write_output(filepaths_to_shuffle: & Vec<fs::DirEntry>, shuffled_filenames: & Vec<String>, config: & Config) -> Result<Vec<String>,  Box<dyn Error>> {
	let mut output_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(config.path.to_str().clone().expect("Cannot clone string").to_owned() + "\\list.txt")
        .expect("Unable to open file");

	let mut new_filenames: Vec<String> = Vec::new();
	
	for iter in filepaths_to_shuffle.iter().zip(shuffled_filenames.iter()) {
		let (path_to_rename, shuffled_filename) = iter;
		
		let path = path_to_rename.path();
		if config.mode == Mode::Simple || config.mode == Mode::Flatten {
			output_file.write_all( format!("{}\n", shuffled_filename).as_bytes() ).expect("Unable to write data");
		}
		else {
			output_file.write_all( format!("{}\t{}\n", get_file_name(&path).ok_or("Cannot extract filename!")?, shuffled_filename).as_bytes() ).expect("Unable to write data");
		}
		let old_name = path.to_str().ok_or("Cannot convert old name to string!")?;
		let tmp_new_name = format!("{}\\{}.{}tmp", config.path.to_str().ok_or("Cannot convert path to str!")?, shuffled_filename, get_extension(&path).ok_or("Cannot extract extension!")?);
		
		println!("file name tmp: {} ", tmp_new_name);
		
		fs::rename(old_name, &tmp_new_name)?;
		new_filenames.push(tmp_new_name);
	}

	Ok(new_filenames)
}


fn remove_temporary_postfix_from_filenames(temporary_filenames: & Vec<String>) -> Result<(),Box<dyn Error>> {
	for name in temporary_filenames {
		let name_without_temp: &str = &name[0..name.len() - 3];
		println!("final name: {} ", name_without_temp);
		fs::rename(name, name_without_temp)?;
	}
	Ok(())
}


fn main() -> std::io::Result<()>  {
    let args: Vec<String> = env::args().collect();
	
	let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
	
	let (mut filesToShuffle, mut fileNames) = get_files_in_directory(&config).expect("Couldn't get files from directory");

	if config.mode == Mode::Simple || config.mode == Mode::Flatten {
		filesToShuffle.sort_by(|a,b| a.path().file_stem().unwrap().to_str().unwrap().parse::<i32>().unwrap().cmp(&b.path().file_stem().unwrap().to_str().unwrap().parse::<i32>().unwrap()));
		fileNames.sort_by(|a,b| a.parse::<i32>().unwrap().cmp(&b.parse::<i32>().unwrap()));
	}

	for file in &filesToShuffle {
		let path_int = file.path();
		let extension = path_int.extension();
		println!("name: {} , extension: {:?}", path_int.display(), extension.unwrap().to_str())
	}
	
	for name in &fileNames {
		println!("file name: {} ", name)
	}
	
	if config.mode == Mode::Flatten {
		fileNames = (1..(filesToShuffle.len()+1)).map(|i| i.to_string()).collect(); 
	} else {
		fileNames.shuffle(&mut thread_rng());
	}
	
	println!("After shuffle. ");
	
	for name in &fileNames {
		println!("file name: {} ", name)
	}
	

	let mut new_temporary_filenames: Vec<String> = shuffle_files_and_write_output(&filesToShuffle, &fileNames, &config).expect("Couldn't shuffle the files!");
	
	remove_temporary_postfix_from_filenames(& new_temporary_filenames);
	 Ok(())
}

fn print_help()
{
	println!("Available flags: ");
	println!("-h/--help - displays this message ");
	println!("-p/--path PATH - sets PATH as path where the script shall be executed. Default path is the script location.");
	println!("-f/--flatten - enables flatten mode, which doesnt shuffle, but tries to bring file numbers to be in order. This mode expects your non-excepted files to have only numbers in names.");
	println!("-s/--simple - enables simple mode, which generates only one column in output file (only post-rename names, line number is the previous filename). This mode expects your non-excepted files to have only numbers in names.");
	println!("-e/--exceptions EXCEPTIONS - sets which file extensions shall be excepted. Separate exceptions by comma, without spaces e.g xls,doc,txt. Default exceptions: txt,xls,obs,doc");
}


struct Config{
	path: std::path::PathBuf,
	mode: Mode,
	exceptions: Vec<String>
}

impl Config {
    fn new(args: &[String]) -> Result<Config,  Box<dyn Error>> {
		let mut path = env::current_dir()?;
		println!("Test {}", args[0].as_str());
		let mut mode = Mode::Standard;
		let mut exceptions : Vec<String> = vec!["txt".into(), "xls".into(), "obs".into(), "doc".into()];
		let mut argument_number = 1;
	
		while argument_number < args.len() {
			match args[argument_number].as_str() {
				"-h" | "--help" => print_help(),
				"-p" | "--path" => {argument_number += 1;
									path = args[argument_number].as_str().into();},
				"-f" | "--flatten" => mode = Mode::Flatten,
				"-s" | "--simple" => mode = Mode::Simple,
				"-e" | "--exception" => {argument_number += 1;
										 exceptions = args[argument_number].as_str().split(',').map(|s| s.to_string()).collect();},
				_ => print_help()
			}
			argument_number = argument_number+1;
		}

		println!("The current directory is {}, exceptions are {:?} and mode is set to {:?} ", path.display(), exceptions, mode );
        Ok(Config { path, mode, exceptions })
    }
}