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
		let path = env::current_dir()?;
		println!("The current directory is {}", path.display());
		let mut alt_mode = false;
		let exceptions : Vec<String> = vec!["txt".to_string(), "xls".to_string(), "obs".to_string(), "doc".to_string()];
	
		for argument in args.iter() {
			match argument.as_str() {
				"-h" | "--help" => print_help(),
				"-p" | "--path" => print_help(),
				"-f" | "--flatten" => alt_mode = true,
				"-e" | "--exception" => print_help(),
				_ => print_help()
			
			}
		}

        //let query = args[1].clone();
        //let filename = args[2].clone();

        Ok(Config { path, alt_mode, exceptions })
    }
}