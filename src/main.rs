use std::io::Write;
use getopts::{Matches, Options};
use std::env;
use std::fs;

mod filter;
mod dictionary;

fn validate_config(opts: &Matches) -> Vec<&'static str> {
    let mut errors: Vec<&'static str> = Vec::new();

    if opts.opt_present("f") && opts.opt_present("u") {
        errors.push("Can't fetch the dictionary from both a file and URL at the same time!")
    }

    if !opts.opt_present("f") && !opts.opt_present("u") {
        errors.push("Provide at least a file dictionary or a url to a dictionary!")
    }

    if !opts.opt_present("l") {
        errors.push("Provide a list of letters!")
    }

    if opts.opt_present("w") && opts.opt_present("c") {
        errors.push("Select only words or count, both are enabled by default, so remove those arguments!");
    }

    errors
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn setup_options() -> Option<Matches> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "displays help page");
    opts.optopt("f", "file", "dictionary file path", "FILE");
    opts.optopt("l", "letters", "list of acceptable letters", "LETTERS");
    opts.optopt("u", "url", "URL to fetch dictionary from", "URL");
    opts.optopt("o", "output", "output file path", "OUTPUT");
    opts.optflag("w", "words", "output only the words");
    opts.optflag("c", "count", "output only the amount of words");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f)
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return None;
    }

    let errors = validate_config(&matches);
    let error_amount = errors.len();
    if error_amount != 0 {
        if error_amount == 1 {
            eprintln!("There is 1 error:");
        } else {
            eprintln!("There are {} errors:", error_amount);
        }

        for error in errors {
            eprintln!("- {}", error);
        }

        return None;
    }

    Some(matches)
}

fn output<T: AsRef<str>>(text: T, stream: &Option<fs::File>) {
    if stream.is_some() {
        writeln!(stream.as_ref().unwrap(), "{}", &text.as_ref()).expect("Couldn't write output to stream!");
    } else {
        println!("{}", &text.as_ref());
    }
}

fn main() {
    let opts = match setup_options() {
        Some(o) => o,
        None => std::process::exit(1)
    };

    let filter = filter::make_filter(&opts.opt_str("l").unwrap());
    
    let dict;
    if opts.opt_present("u") {
        dict = match dictionary::make_dictionary_url(&opts.opt_str("u").unwrap()) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("There was a problem fetching the dictionary from the specified URL!");
                std::process::exit(1);
            }
        };
    } else {
        dict = match dictionary::make_dictionary(&opts.opt_str("f").unwrap()) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("There was a problem opening the file.");
                std::process::exit(1);
            }
        };
    }

    let mut out: Option<fs::File> = None;

    if opts.opt_present("o") {
        let file = match fs::OpenOptions::new().write(true).append(true).create(true).open(opts.opt_str("o").unwrap()) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Couldn't open file for writing the output!");
                std::process::exit(-1);
            }
        };

        out = Some(file);
    }

    let matches = dictionary::get_matches(&dict, &filter);
    // if -w, or if both -w and -c weren't supplied
    if (opts.opt_present("w") && !opts.opt_present("c")) || (!opts.opt_present("w") && !opts.opt_present("c")) {
        for word in &matches {
            output(word, &out);
        }
    }
    // if -c or if both -w and -c weren't supplied
    if opts.opt_present("c") || (!opts.opt_present("w") && !opts.opt_present("c")) {
        output(format!("Word amount: {}", matches.len()), &out);
    }
}