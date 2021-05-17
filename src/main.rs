use std::env;
use std::error::Error;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
	
	let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
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